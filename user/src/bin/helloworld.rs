#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let mut result = 0;
    for i in 0..100 {
        println!(
            "{} {} {} {} {} {} {} {} {} {}",
            i, i, i, i, i, i, i, i, i, i
        );
    }
    println!("hello world {}", result);
    return 0;
}
