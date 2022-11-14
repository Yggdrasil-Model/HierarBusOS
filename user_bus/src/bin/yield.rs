#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{getpid, yield_,exit,wait,fork};

#[no_mangle]
pub fn main() -> i32 {
    /*println!("Hello, I am process {}.", getpid());
    for i in 0..5 {
        yield_();
        println!("Back in process {}, iteration {}.", getpid(), i);
    }*/
    for _ in 0..6 {
        let pid = fork();
        if pid == 0 {
            println!("pid {} ", getpid());
            yield_();
            println!("pid {} OK!", getpid());
            exit(0);
        }
    }

    let mut exit_code: i32 = 0;
    for _ in 0..6 {
        assert!(wait(&mut exit_code) > 0);
        assert_eq!(exit_code, 0);
    }
    println!("yield pass.");
    0
}