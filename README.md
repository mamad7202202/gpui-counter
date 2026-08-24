# gpui-counter

یک برنامه‌ی کوچک شمارنده ساخته‌شده با [GPUI](https://gpui.rs) — فریم‌ورک UI گپ‌شتاب‌دهی‌شده‌ی تیم Zed.

A tiny counter app built with [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui), Zed's GPU-accelerated UI framework.

## Build

```sh
cargo run --release
```

Requires Rust 1.85+ (edition 2024 is used by the `gpui` crate).

On Linux you may need system packages:

```sh
sudo apt install cmake pkg-config libfontconfig1-dev libxkbcommon-dev
```

## CI

GitHub Actions builds release binaries for Windows, macOS and Linux on every push and uploads them as artifacts.
