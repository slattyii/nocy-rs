use anyhow::Result;
use bytes::Bytes;
use futures_util::StreamExt;
use std::{
	fs::File,
	io::{self, BufWriter, Read, Seek, SeekFrom, Write},
};
use tempfile::tempfile;
use tokio::{runtime::Handle, sync::mpsc};

const CHANNEL_BUFFER: usize = 32;
const WRITER_BUFFER: usize = 256 * 1024;

pub struct StreamReader {
	rx: mpsc::Receiver<Bytes>,
	writer: BufWriter<File>,
	reader: File,
	write_pos: u64,
	flushed_pos: u64,
	read_pos: u64,
	exhausted: bool,
	handle: Handle,
}

impl StreamReader {
	pub async fn new(url: &str) -> Result<Self> {
		let mut stream = reqwest::get(url).await?.bytes_stream();
		let (tx, rx) = mpsc::channel(CHANNEL_BUFFER);
		tokio::spawn(async move {
			while let Some(Ok(chunk)) = stream.next().await {
				if tx.send(chunk).await.is_err() {
					break;
				}
			}
		});
		let file = tempfile()?;
		let reader = file.try_clone()?;
		Ok(Self {
			rx,
			writer: BufWriter::with_capacity(WRITER_BUFFER, file),
			reader,
			write_pos: 0,
			flushed_pos: 0,
			read_pos: 0,
			exhausted: false,
			handle: Handle::current(),
		})
	}

	fn recv_chunk(&mut self) -> io::Result<bool> {
		if self.exhausted {
			return Ok(false);
		}
		match tokio::task::block_in_place(|| {
			self.handle.block_on(self.rx.recv())
		}) {
			Some(chunk) => {
				self.writer.write_all(&chunk)?;
				self.write_pos += chunk.len() as u64;
				Ok(true)
			}
			None => {
				self.exhausted = true;
				self.writer.flush()?;
				self.flushed_pos = self.write_pos;
				Ok(false)
			}
		}
	}

	fn buffer_until(&mut self, until: u64) -> io::Result<()> {
		while self.write_pos < until && self.recv_chunk()? {}
		Ok(())
	}

	fn drain(&mut self) -> io::Result<()> {
		while self.recv_chunk()? {}
		Ok(())
	}

	fn ensure_flushed(&mut self, until: u64) -> io::Result<()> {
		if self.flushed_pos < until {
			self.writer.flush()?;
			self.flushed_pos = self.write_pos;
		}
		Ok(())
	}
}

impl Read for StreamReader {
	fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
		let needed = self.read_pos + out.len() as u64;
		self.buffer_until(needed)?;
		if self.read_pos >= self.write_pos {
			return Ok(0);
		}
		self.ensure_flushed(self.read_pos + 1)?;
		self.reader.seek(SeekFrom::Start(self.read_pos))?;
		let n = self.reader.read(out)?;
		self.read_pos += n as u64;
		Ok(n)
	}
}

impl Seek for StreamReader {
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		let target = match pos {
			SeekFrom::Start(n) => n,
			SeekFrom::Current(n) => self.read_pos.saturating_add_signed(n),
			SeekFrom::End(n) => {
				self.drain()?;
				self.write_pos.saturating_add_signed(n)
			}
		};
		self.buffer_until(target)?;
		self.ensure_flushed(target.min(self.write_pos))?;
		self.read_pos = target.min(self.write_pos);
		Ok(self.read_pos)
	}
}
