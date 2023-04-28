// 问题：现在 mem:uninitialized() 废弃了，直接使用似乎会导致 segmentation fault

// #![feature(vec_into_raw_parts)]

// use std::{mem, slice};
// use std::io::Read;

// struct Bar{ vec: Vec<i32>, }
// impl Bar{
//     pub unsafe fn read_from(src: &mut dyn Read, len: usize) -> Bar{
//         let mut bar = mem::uninitialized::<Bar>();
//         // if len >= 10{
//         //     panic!("too long");
//         // }
//         // let s = slice::from_raw_parts_mut(&mut bar as *mut _ as *mut u8, mem::size_of::<Bar>());
//         // src.read_exact(s).unwrap();
//         bar
//     }
// }

fn main(){
    // let mut v = vec![1, 2, 3, 4, 5];
    // let (p, len, cap) = v.into_raw_parts();
    // let mut u = [p as u64, len as _, cap as _];
    // let bp: *const u8 = &u[0] as *const u64 as *const _;
    // let mut b: &[u8] = unsafe{ slice::from_raw_parts(bp, mem::size_of::<[u64; 3]>()) };
    // let mut bar = unsafe{ Bar::read_from(&mut b, 3) };
    // println!("{:?}", bar.vec);
}