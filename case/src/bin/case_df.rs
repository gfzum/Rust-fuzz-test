// 问题：mirai 看不出来 double free 的 bug

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
    let target = String::from("rust fuzz test");
    println!("target string is {:?}, input your operations: ", target);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().parse().unwrap();

    match input.as_str(){
        "length" => {
            println!("{:?}", target.len());
        },
        "reverse" => {
            println!("{:?}", target.chars().rev().collect::<String>());
        },
        "bytes" => {
            let v = genvec(target);
            println!("{:?}", v);
        }
        _ => {
            println!("unknown operation {:?}", input);
        }
    }
}