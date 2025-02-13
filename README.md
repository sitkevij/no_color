# no_color

**no_color** is a zero dependency rust library for NO_COLOR environment variable detection.

[![Crates.io](https://img.shields.io/crates/v/no_color?style=flat-square)](https://crates.io/crates/no_color)
[![Crates.io](https://img.shields.io/crates/d/no_color?style=flat-square)](https://crates.io/crates/no_color)
[![GitHub repo size](https://img.shields.io/github/repo-size/sitkevij/no_color)](https://github.com/sitkevij/no_color)
[![main](https://github.com/sitkevij/no_color/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sitkevij/no_color/actions/workflows/ci.yml?branch=main)
[![docs.rs](https://img.shields.io/docsrs/no_color)](https://docs.rs/no_color/0.2.0/no_color/)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/sitkevij/no_color/blob/main/LICENSE)

## About

From [no-color.org](https://no-color.org/):

An increasing number of command-line software programs output text with ANSI color escape codes by
default. While some developers and users obviously prefer seeing these colors, many users don’t.
Unfortunately, every new piece of software seems to have a different way of disabling colored text
output and some software has no way at all.

Accepting the futility of trying to reverse this trend, an informal standard is hereby proposed:

> All command-line software which outputs text with ANSI color added should check for the presence
> of a NO_COLOR environment variable that, when present (regardless of its value), prevents the
> addition of ANSI color.

By adopting this standard, users that prefer to have plain, non-colored text output can set one
environment variable in their shell to have it automatically affect all supported software.

## Usage

no_color is a library crate which works with [Cargo](http://crates.io).

### Add to Cargo.toml

To use, add the following to your `Cargo.toml` dependencies section:

```toml
[dependencies]
no_color = "0.2"
```

### Implementing Code

Suggested updates to your rust code as below:

- Note as of edition 2018 [extern is not needed](https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html)
  to import a crate.

- See [examples/simple.rs](examples/simple.rs) for reference.

```rust
extern crate no_color;

use no_color::*;

fn main() {
    println!(
        "Environment variable NO_COLOR {0} found. Now do something.",
        {
            if is_no_color() {
                "is"
            } else {
                "is NOT"
            }
        }
    );
}

```
