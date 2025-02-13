extern crate no_color;

use no_color::*;
/**
 * test environment variable NO_COLOR export
 *
 * ```sh
 * export NO_COLOR; echo $NO_COLOR; cargo run --example simple
 * ```
 *
 * test environment variable NO_COLOR unset
 * ```sh
 * export NO_COLOR; echo $NO_COLOR; unset NO_COLOR; cargo run --example simple
 * ```
 *
 */
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
