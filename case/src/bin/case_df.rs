// 问题：mirai 看不出来 double free 的 bug

fn read_input() -> String{
    println!("input target string: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().parse().unwrap();

    input
}

// 获得 s 的所有权，s 会在函数结束时被 drop
fn genvec(mut s:String) -> Vec<u8>{
    let ptr = s.as_mut_ptr();
    unsafe{
        let v = Vec::from_raw_parts(ptr, s.len(), s.len());
        // std::mem::forget(s);
        return v;
    }
}

fn main(){
    let input = read_input();
    // let input = "ee7rust".to_string();

    // suffix match
    let suffix = "rust";
    if !input.ends_with(suffix){
        // println!("input string should end with {:?}", suffix);
        return;
    }

    // input should not contain int
    if input.chars().any(|c| c.is_digit(10)){
        // println!("input string should not contain int");
        return;
    }

    // input's first 4 chats should be greater than "test"
    if input.chars().take(4).collect::<String>() <= "test".to_string(){
        // println!("input's first 4 chats should be greater than \"test\"");
        return;
    }

    // first 3 chars' ascii sum >= 300
    let mut sum = 0;
    for c in input.chars().take(3){
        sum += c as u32;
    }
    if sum < 350{
        // println!("first 3 chars' ascii sum should >= 350");
        return;
    }

    let v = genvec(input);
    println!("{:?}", v);
}