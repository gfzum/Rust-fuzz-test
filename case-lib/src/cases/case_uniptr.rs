fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().parse().unwrap();

    input
}

pub fn test(input: &str) {
    
    // let input = read_input();
    // let input = input.as_str();

    // let input = "input:---";

    let re = regex::Regex::new(r"^input:[\d-]+$").unwrap();
    if !re.is_match(input) {
        // println!("not valid");
        return;
    }
    let mut s = re.find(input).unwrap().as_str()[6..].to_string();
    println!("s = {}", s);

    if s.len() < 2 {
        // println!("input too short");
        return;
    }

    let p = if let Some(pos) = s.chars().position(|c| c.is_digit(10)) {
        &pos as *const _
    }else {
        std::ptr::null()
    };

    unsafe{
        // println!("the first digit is on poisition {:?}", (*p) + 1);
        let pp = s.as_mut_ptr().add(*p);
        (*pp) = b'a';
        // println!("change it to a, new s = {}", s);
    };
}