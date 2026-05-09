<div align="center">

# 🎵 nocy

**một cái music player. chạy được trên máy tui. máy bạn thì... may rủi.**

![Rust](https://img.shields.io/badge/Rust-2024-orange?style=flat-square&logo=rust)
![TUI](https://img.shields.io/badge/interface-terminal-black?style=flat-square&logo=gnometerminal)
![Works on My Machine](https://img.shields.io/badge/works%20on-my%20machine-brightgreen?style=flat-square)
![Vibes](https://img.shields.io/badge/vibes-immaculate-ff69b4?style=flat-square)

![nocy screenshot](assets/screenshot.png)

</div>

---

## nó là cái gì

`nocy` là một TUI music player viết bằng Rust, chạy thẳng trong terminal. Không cần GUI, không cần electron ngốn 2GB RAM, không cần gì cả ngoài một cửa sổ terminal và đôi tai biết nghe nhạc.

Stream nhạc từ SoundCloud, phát ngay trong terminal, có queue, có shuffle, có album art mờ mờ kiểu lofi cho đủ vibe.

---

## tại sao lại có cái này

- Tui nghe nhạc bằng terminal vì... tại sao không?
- Spotify thì cần account premium
- VLC thì phải dùng chuột (ew)
- Viết Rust thì vui hơn làm việc thật sự

---

## cài đặt

### yêu cầu

- [Rust](https://rustup.rs/) — edition 2024, cần **Rust 1.85+**
- Một terminal không quá cổ lỗ sĩ

### build

```bash
git clone https://github.com/slattyii/nocy-rs
cd nocy-rs
cargo run --release
```

Lần đầu build sẽ lâu vì Rust phải compile mấy chục crates. Đi pha ly cà phê đi, tui chờ.

> ⚠️ **Lưu ý:** Repo này chủ yếu dùng cá nhân. Nếu chạy không được trên máy bạn thì... đó là feature, không phải bug.

---

## layout

```
nocy v0.0.1 internal  |  Found 50 Tracks
┌── Nocy Nocy ───────────────────────────────────────┐  ┌── Playing ────────────────┐
│  Opalite                                           │  │                           │
├── Board ───────────────────────────────────────────┤  │       Opalite             │
│   1.  The Fate of Ophelia        SoundCloud  00:30 │  │       Taylor Swift        │
│ ▶ 2.  Opalite                    SoundCloud  00:30 │  │                           │
│ # 3.  Wildest Dreams             SoundCloud  00:30 │  │  ██████████████░  00:28   │
│ # 4.  Love Story (Taylor's Ver.) SoundCloud  00:30 │  │  vol 100%  00:30   nocy   │
│   5.  Cruel Summer               SoundCloud  00:30 │  ├── Queue  1h 59m 58s ──────┤
│   6.  Taylor Swift - The Fate... SoundCloud  03:36 │  │ ▶ Wildest Dreams          │
│   ...                                              │  │   Love Story              │
│                                                    │  │   Cruel Summer            │
└────────────────────────────────────────────────────┘  │   ...                     │
                                                        └───────────────────────────┘
 Del quit  /  search  ↑↓ navigate  Shift ↑↓ reorder  Ctrl a/z add/remove all
 Ctrl r shuffle board  Ctrl Alt r shuffle queue  Space pause/resume  ~ queue
```

---

## keybindings

| Phím                  | Tác dụng                   |
| --------------------- | -------------------------- |
| `Space`               | Pause / Resume             |
| `/`                   | Tìm kiếm                   |
| `↑` / `↓`             | Di chuyển trong danh sách  |
| `Shift ↑` / `Shift ↓` | Đổi thứ tự track (reorder) |
| `Ctrl a`              | Add all vào queue          |
| `Ctrl z`              | Remove all khỏi queue      |
| `Ctrl r`              | Shuffle board              |
| `Ctrl Alt r`          | Shuffle queue              |
| `~`                   | Mở / đóng queue            |
| `Del`                 | Thoát                      |

---

## tech stack

| Crate                                                    | Dùng để làm gì                |
| -------------------------------------------------------- | ----------------------------- |
| [`ratatui`](https://github.com/ratatui-org/ratatui)      | Vẽ TUI đẹp lung linh          |
| [`rodio`](https://github.com/RustAudio/rodio)            | Phát audio (mp3, wav, flac)   |
| [`reqwest`](https://github.com/seanmonstar/reqwest)      | Fetch nhạc từ internet        |
| [`scraper`](https://github.com/causal-agent/scraper)     | Scrape metadata bài hát       |
| [`tokio`](https://tokio.rs/)                             | Async runtime cho người sang  |
| [`crossterm`](https://github.com/crossterm-rs/crossterm) | Cross-platform terminal magic |

---

## platform support

| OS              | Tình trạng                  |
| --------------- | --------------------------- |
| Máy tui (Linux) | ✅ Chạy ngon                |
| Mac             | 🤷 Chưa test, probably fine |
| Windows         | 🙏 Cầu may                  |
| Máy bạn         | ❓ Không biết               |

---

## license

Không có license nghĩa là bản quyền thuộc về tui. Nhưng thôi, dùng đi, tui không quan tâm lắm.

Nếu bạn fork về làm triệu phú thì... kể tui nghe với.

---

<div align="center">

_made with ♥ and too much caffeine_

</div>
