#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[macro_use]
extern crate user_lib;

use user_lib::*;

#[no_mangle]


#[no_mangle]
pub fn main() -> i32 {
    //let s:&str;
    println!("Hello world from user mode program!");
   // sys_write(1, s.as_bytes());
    0
}