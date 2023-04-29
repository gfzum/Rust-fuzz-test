
fn test(input: &str){

    panic!();

    // let re = regex::Regex::new(r"^input:[\d-]+$").unwrap();
    // if !re.is_match(input) {
    //     println!("not valid");
    //     return;
    // }
    // let mut s = re.find(input).unwrap().as_str()[6..].to_string();
    // println!("s = {}", s);

    // if s.len() < 2 {
    //     println!("input too short");
    //     return;
    // }

}

fn main() {
    
    
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // let input = input.trim();

    let input = "input:123";


    test(input);



    let p = if let Some(mut pos) = s.chars().position(|c| c.is_digit(10)) {
        &mut pos as *mut _
    }else {
        std::ptr::null()
    };

    unsafe{
        println!("the first digit is on poisition {:?}", *p + 1);
        let pp = s.as_mut_ptr().add(*p);
        (*pp) = b'a';
        println!("change it to a, new s = {}", s);
    };
    

}