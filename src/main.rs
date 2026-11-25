// use std::mem::size_of;


// fn main() {
//     // println!("Hello, world!");

//     // let mut name = String::from("Rust");

//     // name.push_str(" Programming"); 
//     // println!("{}",name);
//     // println!("Size of name: {}", size_of::<String>());
//     // println!("Size of name i8 : {}", size_of::<i8>());
//     // println!("Size of name i16 : {}", size_of::<i16>());
//     // println!("Size of name i32 : {}", size_of::<i32>());
//     // println!("Size of name i64 : {}", size_of::<i64>());
//     // println!("Size of name u8 : {}", size_of::<u8>());
//     // println!("Size of name u16 : {}", size_of::<u16>());
//     // println!("Size of name u32 : {}", size_of::<u32>());
//     // println!("Size of name u64 : {}", size_of::<u64>());
//     // println!("Size of name f32 : {}", size_of::<f32>());    
//     // println!("Size of name f64 : {}", size_of::<f64>());
//     // println!("Size of name bool : {}", size_of::<bool>());
//     // println!("Size of name char : {}", size_of::<char>());


    
// }


use std::mem::size_of;

fn main() {
    let name = String::from("Rust");
    let text: &str = "Rust";

    println!("String size: {}", size_of::<String>());
    println!("&str size: {}", size_of::<&str>());

    println!("String length: {}", name.len());
    println!("String capacity: {}", name.capacity());

    println!("Text length: {}", text.len());

}