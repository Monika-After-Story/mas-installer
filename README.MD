[![CI](https://github.com/Monika-After-Story/mas-installer/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/Monika-After-Story/mas-installer/actions/workflows/ci.yml) [![Build](https://github.com/Monika-After-Story/mas-installer/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/Monika-After-Story/mas-installer/actions/workflows/build.yml)

[![CI](https://github.com/Monika-After-Story/mas-installer/actions/workflows/ci.yml/badge.svg?branch=dev)](https://github.com/Monika-After-Story/mas-installer/actions/workflows/ci.yml)

### Description
Cross-platform installer for [Monika After Story](https://github.com/Monika-After-Story/MonikaModDev)

### Main features:
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

### Main stack:
- Rust 1.62
- FLTK 1.3

### Building from source:
- Building is straightforward using [`cargo`](https://github.com/rust-lang/cargo/)
- - Linux: you might need to install some dependencies (check [this workflow](https://github.com/Monika-After-Story/mas-installer/blob/master/.github/workflows/build.yml) for more details)
- - Windows: for now both `msvc` and `gnu` toolchains are supported, but `msvc` is more preferable
