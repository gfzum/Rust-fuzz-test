#[macro_use]
extern crate afl;

use case_lib::cond_test;

fn main() {
    fuzz!(|data: &[u8]| {
        // convert data to i32 and pass to cond_test
        if let Ok(a) = std::str::from_utf8(data) {
            if let Ok(a) = a.parse::<i32>() {
                cond_test(a);
            }
        }
    });
}
