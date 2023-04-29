use std::io::stdin;
use std::slice;

fn split_at_mut(slice: &mut str, index: usize) -> (&mut str, &mut str) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr() as *mut u8;

    unsafe {
        (
            std::str::from_utf8_unchecked_mut(slice::from_raw_parts_mut(ptr, index)),
            std::str::from_utf8_unchecked_mut(slice::from_raw_parts_mut(ptr.add(index), len - index)),
        )
    }
}

fn main() {

    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    input= input.trim().parse().unwrap();

    // 限制输入字符串的长度
    let limit_length: usize = 10; 
    if input.len() > limit_length {
        println!("input string too long");
        return;
    }

    // 检查输入字符串是否包含子串
    let substr: &str = "rust";
    if !input.contains(substr) {
        println!("input string does not contain \"{}\"", substr);
        return;
    }

    // 分割字符串，如果分割位置超出字符串长度，会 panic
    let split_index = 5;
    println!("split s into 2 slices at index {}", split_index);
    let (a, b) = split_at_mut(&mut input, split_index);

    println!("split part 1 = {:?}", a);
    println!("split part 2 = {:?}", b);   
}