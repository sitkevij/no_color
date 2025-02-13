extern crate no_color;

use no_color::*;
/// # Examples
///
/// ## testing environment variable `NO_COLOR` set
///
/// ```sh
/// export NO_COLOR; echo $NO_COLOR; cargo run --example simple
/// ```
///
/// ## testing environment variable `NO_COLOR` unset
///
/// ```sh
/// export NO_COLOR; echo $NO_COLOR; unset NO_COLOR; cargo run --example simple
/// ```
///
/// ## The following code is executed in the simple example
///
/// ```rust
///   println!(
///     "Environment variable NO_COLOR {0} found. Now do something.",
///     {
///         if is_no_color() {
///             "is"
///         } else {
///             "is NOT"
///         }
///     }
///   );
/// ```
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
