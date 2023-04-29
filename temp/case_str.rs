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

fn read_input() -> (String, usize){
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    input= input.trim().parse().unwrap();
    let length = input.len();

    // 获取字符串最后一个数字，如果不是十进制数字，会 panic
    let split_index: usize = input.chars().last().unwrap().to_digit(10).unwrap() as usize; 
    let s: String = input.chars().take(length - 1).collect(); // 输入字符串去掉最后一个数字

    (s, split_index)
}

fn main() {

    // 读入一行字符串，最后一个字符（整数）是分割位置，如 "rust3"
    let (mut s, split_index) = read_input();

    // 限制输入字符串的长度
    let limit_length: usize = 10; 
    if s.len() > limit_length {
        println!("input string too long");
        return;
    }

    // 检查输入字符串是否包含子串
    let substr: &str = "rust";
    if !s.contains(substr) {
        println!("input string does not contain \"{}\"", substr);
        return;
    }

    // 分割字符串，如果分割位置超出字符串长度，会 panic
    println!("split s into 2 slices at index {}", split_index);
    let (a, b) = split_at_mut(&mut s, split_index);

    println!("split part 1 = {:?}", a);
    println!("split part 2 = {:?}", b);   
}