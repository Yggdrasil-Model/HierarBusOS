#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[macro_use]
extern crate user_lib;

use user_lib::*;

#[no_mangle]

pub unsafe fn get_cycles()->isize{
    let cycles:isize;
    llvm_asm!("rdtime  $0" : "=r"(cycles)  :: "volatile");
    cycles

}

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello world from user mode program!");
    0
}