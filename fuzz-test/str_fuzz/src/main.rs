#[macro_use]
extern crate afl;

use case_lib;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            case_lib::str_test(s);
        }
    });
}
