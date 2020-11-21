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
