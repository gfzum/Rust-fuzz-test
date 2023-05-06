mod cases;
use cases::{case_str2, case_df, case_uniptr,  case_hello};

pub fn str_test(s: &str) {
    case_str2::test(s);
}

pub fn df_test(s: &str) {
    case_df::test(s);
}

pub fn uniptr_test(s: &str) {
    case_uniptr::test(s);
}

// pub fn cond_test(a: i32) {
//     case_cond::test(a);
// }

pub fn hello_test(s: &str) {
    case_hello::test(s);
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         let s = "input:1234567890";
//         str_test(s);
//         df_test(s);
//         uniptr_test(s);
//     }
// }