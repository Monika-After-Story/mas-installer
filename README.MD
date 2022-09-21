[![CI](https://github.com/Monika-After-Story/mas-installer/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/Monika-After-Story/mas-installer/actions/workflows/ci.yml) [![Build](https://github.com/Monika-After-Story/mas-installer/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/Monika-After-Story/mas-installer/actions/workflows/build.yml)

[![CI](https://github.com/Monika-After-Story/mas-installer/actions/workflows/ci.yml/badge.svg?branch=dev)](https://github.com/Monika-After-Story/mas-installer/actions/workflows/ci.yml)

### Description
Cross-platform installer for [Monika After Story](https://github.com/Monika-After-Story/MonikaModDev)

### Main Features:
- Cross-platform (Windows, MacOS, Linux)
- Small executables size
- Fast
- Dynamic version fetching
- Detection of DDLC directory
- Multiple downloads options:
- - default version
- - deluxe version
- - spritepacks (separated)
- Music during installation

### Music Credits:
Installer theme by [MyNewSoundtrack](https://www.youtube.com/user/MyNewSoundtrack)

### Main Stack:
- Rust 1.62 (can build >= 1.59)
- FLTK 1.3

### Building from Source:
- Building is straightforward using [`cargo`](https://github.com/rust-lang/cargo/)
- - Linux: you might need to install some dependencies (check [this workflow](https://github.com/Monika-After-Story/mas-installer/blob/master/.github/workflows/build.yml) for more details)
- - Windows: for now both `msvc` and `gnu` toolchains are supported, but `msvc` is more preferable
- For all supported features check `Cargo.toml`
- - `include_license` includes `static/license.md` into the binary during building (enabled by default)
- - To disable all default features, use the `--no-default-features` flag in cargo
