#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[macro_use]
extern crate user_lib;

use user_lib::*;


#[no_mangle]
pub unsafe fn main() -> i32 {
    let mut cycles:isize;
    let mut e:isize;
    llvm_asm!("rdtime  $0" : "=r"(cycles)  :: "volatile");
   for _ in 0..1000{
       getpid();
   }
    llvm_asm!("rdtime  $0" : "=r"(e)  :: "volatile");
    println!("{}",(e-cycles)/1000);
    0
}