# termsize

[![CI](https://github.com/softprops/termsize/actions/workflows/ci.yml/badge.svg)](https://github.com/softprops/termsize/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/termsize.svg)](https://crates.io/crates/termsize)

> because terminal size matters

Termsize is a rust crate providing a multi-platform interface for resolving
your terminal's current size in rows and columns. On most unix systems, this is similar invoking the [stty(1)](http://man7.org/linux/man-pages/man1/stty.1.html) program, requesting the terminal size.


## [Documentation](https://softprops.github.com/termsize)

## install

run `cargo add termsize` in your terminal or add the following to your `Cargo.toml` file

```toml
[dependencies]
termsize = "0.1"
```

## usage

Termize provides one function, `get`, which returns a `termsize::Size` struct
exposing two fields: `rows` and `cols` representing the number of rows and columns
a a terminal's stdout supports.

```rust
pub fn main() {
  termsize::get().map(|{ rows, cols }| {
    println!("rows {} cols {}", size.rows, size.cols)
  });
}
```

Doug Tangren (softprops) 2015-2024