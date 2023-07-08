[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![rand](https://img.shields.io/badge/rand-v0.8.5-orange?style=flat-square)](https://crates.io/crates/rand)
[![clap](https://img.shields.io/badge/clap-v4.3.0-orange?style=flat-square)](https://crates.io/crates/clap)

# rspw (rust-password)
> **Command Line Argument Tool for generating random passwords.**
>> **Developed using Rust and crate [rand](https://github.com/rust-random/rand) & [clap](https://github.com/clap-rs/clap)**


## Install using [cargo](https://doc.rust-lang.org/cargo/)
```bash
$ cargo install rspw
```

## Usage
Length can be chosen from 6-64:
```bash
$ rspw -l 32
> 6Og8mOtGQyfwyA8SxfdasXX3gvGfiiaT
```
Special characters with `-s` flag:
```bash
$ rspw -l=12 -s
> 7b%TfAn~qIdS
```
Password can be attached to the clipboard for 30 s (Linux):
```bash
$ rspw -l=64 -s -c
> Password will be copied on the clipboard, clears in 30 seconds.
```

## TODO
See [main](src/main.rs), feel free to contribute.
