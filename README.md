[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![rand](https://img.shields.io/badge/rand-v0.8.5-orange?style=flat-square)](https://crates.io/crates/rand)
# rspw (rust-password)

> **Command Line Argument Tool for generating random passwords.**
>> **Developed using Rust and crate [rand](https://github.com/rust-random/rand).**

## Install using [cargo](https://doc.rust-lang.org/cargo/)
```bash
$ cargo install rspw
```
## Usage
Default generates a password with length 8:
```bash
$ rspw gen 
> dK7VgAz9
```
Length can be chosen from 6-64:
```bash
$ rspw gen 32
> kGk4z2oZ2RXkwrbY68Ru4TJtAk0Rz2nq
```

## TODO
See [main](src/main.rs), feel free to contribute.