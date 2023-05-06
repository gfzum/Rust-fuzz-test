pub fn test(a: i32) {
    // let a: i32 = get_args();
    if a < 100 {
        let a1 = a % 11; let a2 = a % 101;
        let b1 = a % 43; let b2 = a % 149;
        let v = a1 * b1 + a2 * b2;
        if  a1 > a2 && b1 < b2
            && v > a1 + a2 && v < b1 + b2 {
            panic!()
        }
    } else {
        if a1 < a2 && b1 > b2
            && v < a1 * a2 && v > b1 * b2 {
            panic!()
        }
    }
}