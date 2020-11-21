extern crate no_color;

use no_color::*;

fn main() {
	if is_no_color() {
		println!("{0}", "Not ok to print color terminal chars");
	} else {
		println!("{0}", "Ok to print color terminal chars");
	}
}
