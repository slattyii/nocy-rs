# 🎵 nocy

**một cái music player. chạy được trên máy tui. máy bạn thì... may rủi.**

![Rust](https://img.shields.io/badge/Rust-2024-orange?style=flat-square&logo=rust)
![TUI](https://img.shields.io/badge/interface-terminal-black?style=flat-square&logo=gnometerminal)
![Works on My Machine](https://img.shields.io/badge/works%20on-my%20machine-brightgreen?style=flat-square)
![Vibes](https://img.shields.io/badge/vibes-immaculate-ff69b4?style=flat-square)

<!--
  THÊM SCREENSHOT VÀO ĐÂY!
  Gợi ý: dùng `vhs` để record terminal (https://github.com/charmbracelet/vhs)
  hoặc chụp màn hình rồi để vào thư mục assets/ trong repo
  sau đó thay dòng dưới:
-->

![nocy demo](assets/demo.gif)

</div>

---

## nó là cái gì

`nocy` là một TUI music player viết bằng Rust, chạy thẳng trong terminal. Không cần GUI, không cần electron ngốn 2GB RAM, không cần gì cả ngoài một cửa sổ terminal và đôi tai biết nghe nhạc.

Stream nhạc từ web, phát ngay trong terminal. Đơn giản vậy thôi.

```
┌─────────────────────────────────────────────┐
│  nocy — now playing                         │
│                                             │
│  ♪  Tên bài hát hay ho nào đó              │
│     Artist • Album                          │
│                                             │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━────  2:34/4:12  │
│                                             │
│  [q]uit  [space] pause  [n]ext  [p]rev      │
└─────────────────────────────────────────────┘
```

---

## tại sao lại có cái này

- Tui nghe nhạc bằng terminal vì... tại sao không?
- Spotify thì cần account premium
- VLC thì phải dùng chuột (ew)
- Viết Rust thì vui hơn làm việc thật sự

---

## cài đặt

### yêu cầu

- [Rust](https://rustup.rs/) — phiên bản mới nhất, không cãi
- Edition 2024 → cần **Rust 1.85+**
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

## keybindings

| Phím      | Tác dụng                             |
| --------- | ------------------------------------ |
| `Space`   | Play / Pause                         |
| `n`       | Bài tiếp theo                        |
| `p`       | Bài trước                            |
| `↑` / `↓` | Di chuyển trong danh sách            |
| `Enter`   | Phát bài được chọn                   |
| `q`       | Thoát (và trở lại với cuộc đời thực) |

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

## screenshot

> _Sắp có. Đang lười._

<!--
  TODO: thêm ảnh thật vào đây. Gợi ý cách chụp đẹp:

  1. Dùng vhs (https://github.com/charmbracelet/vhs):
     - Viết file .tape mô tả thao tác
     - Chạy: vhs demo.tape
     - Output ra demo.gif xịn sò

  2. Dùng asciinema (https://asciinema.org/):
     - asciinema rec demo.cast
     - Chơi một lúc rồi Ctrl+D
     - Upload lên asciinema.org hoặc convert sang gif bằng agg

  3. Chụp màn hình bình thường cũng được, đừng có màn hình tối đen ngủ gật
-->

---

## license

Không có license nghĩa là bản quyền thuộc về tui. Nhưng thôi, dùng đi, tui không quan tâm lắm.

Nếu bạn fork về làm triệu phú thì... kể tui nghe với.

---

<div align="center">

_made with ♥ and too much caffeine_
