//! # no_color
//!
//! [![Crates.io](https://img.shields.io/crates/v/no_color?style=flat-square)](https://crates.io/crates/no_color)
//! [![Crates.io](https://img.shields.io/crates/d/no_color?style=flat-square)](https://crates.io/crates/no_color)
//! [![GitHub repo size](https://img.shields.io/github/repo-size/sitkevij/no_color)](https://github.com/sitkevij/no_color)
//! [![main](https://github.com/sitkevij/no_color/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sitkevij/no_color/actions/workflows/ci.yml?branch=main)
//! [![docs.rs](https://img.shields.io/docsrs/no_color)](https://docs.rs/no_color/0.2.0/no_color/)
//! [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/sitkevij/no_color/blob/main/LICENSE)
//!
//! **no_color** is a zero dependency rust library for detecting if the NO_COLOR environment
//! variable is set.
//!
//! Note only the existence of the NO_COLOR environment variable is required to be valid, whether
//! the environment variable has value or not.
//!
//! ## Usage
//!
//! no_color is a library crate which works with [Cargo](http://crates.io).
//!
//!  ### Add to Cargo.toml
//!     
//! To use, add the following to your `Cargo.toml` dependencies section:
//!
//! ```toml
//! [dependencies]
//! no_color = "0.2"
//! ```
//!
//! or use cargo add:
//!
//! ```sh
//! $ cargo add no_color
//! ```
use std::env;

/// const NO_COLOR str
const ENV_VAR_NO_COLOR: &str = "NO_COLOR";

/// Function returns `true` if NO_COLOR environment variable exists, `false` if
///  not.
///
/// note: std::env::var_os function may panic if key is empty, contains an
/// ASCII equals sign '=' or the NUL character '\0', or when the value contains
/// the NUL character. Read more <https://doc.rust-lang.org/std/env/fn.var_os.html>
pub fn is_no_color() -> bool {
    env::var_os(ENV_VAR_NO_COLOR).is_some()
}

#[cfg(test)]
/// <https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab>
/// these tests set NO_COLOR environment variable
mod tests {
    use super::*;

    /// test_env_var_no_color_true_with_value()
    #[test]
    fn test_env_var_no_color_true_with_value() {
        env::set_var(ENV_VAR_NO_COLOR, "1");
        assert!(is_no_color(), "{}", true);
    }

    #[test]
    fn test_env_var_no_color_true_without_value() {
        env::set_var(ENV_VAR_NO_COLOR, "");
        assert!(is_no_color(), "{}", true);
    }
}
