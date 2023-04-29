// 这个 *const 居然可以无视生命周期？太神奇了

pub struct Foo{
    data: Vec<u8>,
}

impl Foo{
    pub fn new(data: &[u8]) -> Foo{
        Foo{ data: data.to_vec() }
    }

    // return a raw pointer of this struct
    pub fn as_ptr(&self) -> *const Foo{
        self as *const Foo    
    }
    pub fn test(&self){
        println!("test");
        println!("data is {:?}", self.data)
    }
}

pub unsafe fn use_raw_ptr(ptr: *const Foo, v: Vec<u8>){
    println!("unsafe raw pointer get data {:?}", (*ptr).data);
    assert_eq!(v, (*ptr).data);
    (*ptr).test();
}

fn main(){
    let data: Option<&[u8]> = Some(b"abc");
    let p = match data{
        Some(data) => {
            // let foo = Foo::new(data); 
            let foo = Foo{
                data: data.to_vec(),
            };
            foo.as_ptr()
        } // foo will be dropped here
        None => std::ptr::null(),
    };

    let v: Vec<u8> = data.unwrap().to_vec();
    println!("original data is {:?}", v);
    unsafe{(*p).test()}
    unsafe{ use_raw_ptr(p, v) };
}