// pub struct Foo{
//     data: Vec<u8>,
// }
// impl Foo{
//     pub fn new(data: &[u8]) -> Foo{
//         Foo{ data: data.to_vec() }
//     }
//     pub fn as_ptr(&self) -> &Foo{
//         // return a raw pointer of this struct
//         self
//     }
//     pub fn test(&self){
//         println!("test");
//     }
    
// }

// pub unsafe fn use_raw_ptr(ptr: *const Foo){
//     println!("{:?}", (*ptr).test());
// }

fn main(){
    // let data: Option<&[u8]> = Some(b"a");
    // let p = match data{
    //     Some(data) => {
    //         let foo = Foo::new(data);
    //         // foo.as_ptr()
    //         // &foo as *const Foo
    //         // &foo
    //     } 
    //     // None => std::ptr::null_mut(),
    //     None => {
    //         let foo = Foo::new(b"b");
    //         // &foo
    //     },
    // };
    // // unsafe {(*p).test()};
    // // println!("{:?}", foo);
    // // unsafe{ use_raw_ptr(p) };
}