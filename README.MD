[![CI](https://github.com/Monika-After-Story/mas-installer/actions/workflows/ci.yml/badge.svg)](https://github.com/Monika-After-Story/mas-installer/actions/workflows/ci.yml) [![Build](https://github.com/Monika-After-Story/mas-installer/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/Monika-After-Story/mas-installer/actions/workflows/build.yml)

### Description
Cross-platform installer for [Monika After Story](https://github.com/Monika-After-Story/MonikaModDev)

### Main features:
- Cross-platform (Windows, MacOS, Linux)
- Small executables size
- Fast
- Dynamic version fetching
- Music during installation

### Main stack:
- Rust 1.25
- FLTK 1.3

### Building from source:
- Building is straightforward using [`cargo`](https://github.com/rust-lang/cargo/)
- - on Linux you might need to install some dependencies (check [this workflow](https://github.com/Monika-After-Story/mas-installer/blob/master/.github/workflows/build.yml) for more details)
- - on Windows you can use both `msvc` and `gnu` (`gnu` results in a bigger size though)
