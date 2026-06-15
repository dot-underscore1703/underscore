# === UNDERSCORE ===
## GTK3 Notifications Daemon 

Underscore is a fast, lightweight Rust-powered notifications daemon made for Linux, built with GTK3 and compatible with the freedesktop.org spec.

## Features
- Rust, fast, memory safe and sturdy.
- GTK3 popups
- freedesktop.org notifications
- Minimal resource usage
- Custom themes

## Installation
```
git clone https://github.com/dot-underscore1703/underscore/
cd underscore/underscore
cargo build --release
```
Run with
`./target/release/underscore &`

## Dependencies
- GTK3
- glib
- Dbus
- cargo (for installation)
      - See underscore/Cargo.toml for a list of build dependencies.

Debian/Ubuntu
`sudo apt install libgtk-3-dev libglib2.0-dev libdbus-1-dev pkg-config`

## License
Underscore is licensed under MIT.
See the LICENSE file for more information.
