//! **no_color** is a rust library for detecting if the NO_COLOR environment
//! variable is set. Check for the existence of the NO_COLOR environment
//! variable, whether it has value or not.
use std::env;

/// const NO_COLOR str
const ENV_VAR_NO_COLOR: &str = "NO_COLOR";

/// Function returns `true` if NO_COLOR environment variable exists, `false` if
///  not.
///
/// note: std::env::var_os function may panic if key is empty, contains an
/// ASCII equals sign '=' or the NUL character '\0', or when the value contains
/// the NUL character. Read more https://doc.rust-lang.org/std/env/fn.var_os.html
pub fn is_no_color() -> bool {
    env::var_os(ENV_VAR_NO_COLOR).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    /// https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
    /// these tests set NO_COLOR environment variable
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
