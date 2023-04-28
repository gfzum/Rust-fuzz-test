// 这个 *const 居然可以无视生命周期？太神奇了

pub struct Foo{
    data: Vec<u8>,
}
impl Foo{
    pub fn new(data: &[u8]) -> Foo{
        Foo{ data: data.to_vec() }
    }
    pub fn as_ptr(&self) -> *const Foo{
        // return a raw pointer of this struct
        self as *const Foo    
    }
    pub fn test(&self){
        println!("test");
    }
    
}

pub unsafe fn use_raw_ptr(ptr: *const Foo){
    println!("{:?}", (*ptr).data);
}

fn main(){
    let data: Option<&[u8]> = Some(b"lfasd");
    let p = match data{
        Some(data) => {
            let foo = Foo::new(data);
            foo.as_ptr();
            &foo as *const Foo
            // &foo
            // 这里没有真正释放 foo ？
        } 
        // None => std::ptr::null_mut(),
        None => {
            let foo = Foo::new(b"b");
            &foo as *const Foo
        },
    };
    unsafe {(*p).test()};
    println!("{:?}", data);
    unsafe{ use_raw_ptr(p) };
    
}