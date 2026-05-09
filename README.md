<div align="center">

# 🎵 nocy

A terminal music player that streams from SoundCloud.

![Rust](https://img.shields.io/badge/Rust-2024-orange?style=flat-square&logo=rust)
![TUI](https://img.shields.io/badge/interface-terminal-black?style=flat-square&logo=gnometerminal)
![Works on My Machine](https://img.shields.io/badge/works%20on-my%20machine-brightgreen?style=flat-square)

![nocy screenshot](assets/screenshot.png)

</div>

---

## About

nocy is a TUI music player built with Rust. Search SoundCloud, build a board of tracks, manage a queue, and play — all from the terminal.

---

## Requirements

- Rust **1.85+** (edition 2024)

---

## Installation

```bash
git clone https://github.com/slattyii/nocy-rs
cd nocy-rs
cargo run --release
```

> Works on my machine. May or may not work on yours.

---

## Layout

```
nocy v0.0.1  |  Found 50 Tracks
┌── Board ───────────────────────────────────────────┐  ┌── Playing ────────────────┐
│   1.  The Fate of Ophelia    SoundCloud  00:30     │  │                           │
│ ▶ 2.  Opalite                SoundCloud  00:30     │  │         Opalite           │
│ # 3.  Wildest Dreams         SoundCloud  00:30     │  │         Taylor Swift      │
│ # 4.  Love Story             SoundCloud  00:30     │  │                           │
│   5.  Cruel Summer           SoundCloud  00:30     │  │  ██████████████░  00:28   │
│   ...                                              │  │  vol 100%  00:30          │
└────────────────────────────────────────────────────┘  ├── Queue  1h 59m 58s ──────┤
                                                        │ ▶ Wildest Dreams          │
▶  currently playing                                    │   Love Story              │
#  in queue                                             │   ...                     │
                                                        └───────────────────────────┘
```

---

## Keybindings

| Key                   | Action                |
| --------------------- | --------------------- |
| `Space`               | Pause / Resume        |
| `/`                   | Search                |
| `↑` / `↓`             | Navigate              |
| `Shift ↑` / `Shift ↓` | Reorder tracks        |
| `Ctrl a`              | Add all to queue      |
| `Ctrl z`              | Remove all from queue |
| `Ctrl r`              | Shuffle board         |
| `Ctrl Alt r`          | Shuffle queue         |
| `~`                   | Toggle queue panel    |
| `Del`                 | Quit                  |

---

## Built With

| Crate                                                  | Purpose          |
| ------------------------------------------------------ | ---------------- |
| [ratatui](https://github.com/ratatui-org/ratatui)      | TUI framework    |
| [rodio](https://github.com/RustAudio/rodio)            | Audio playback   |
| [reqwest](https://github.com/seanmonstar/reqwest)      | HTTP client      |
| [scraper](https://github.com/causal-agent/scraper)     | HTML scraping    |
| [tokio](https://tokio.rs/)                             | Async runtime    |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Terminal backend |

---

## License

MIT
