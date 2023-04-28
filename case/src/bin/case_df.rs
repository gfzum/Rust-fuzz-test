// 问题：mirai 看不出来 double free 的 bug

fn genvec() -> Vec<u8>{
    let mut s = String::from("a tmp string");
    let ptr = s.as_mut_ptr();
    unsafe{
        let v = Vec::from_raw_parts(ptr, s.len(), s.len());
        // std::mem::forget(s);
        return v;
    }
}

fn main(){
    let v = genvec();
    println!("{:?}", v);
}