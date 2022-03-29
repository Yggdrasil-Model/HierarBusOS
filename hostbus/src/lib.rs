#![no_std]
#![feature(global_asm)]
#![feature(llvm_asm)]
pub mod config;
pub use config::*;

pub mod controller;
pub use controller::*;
use lazy_static::*;
use spin::{Mutex};
pub mod forward;
pub use forward::*;
global_asm!(include_str!("trap.S"));
 
 #[macro_use]
extern crate console_support;
extern crate dynamic_malloc_support;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimeSpec {
    pub sec: usize,
    pub nsec: usize,
}
pub struct Time{
    pub timespecinner:Mutex<TimeSpec> 
}
lazy_static!{
    pub static ref TIME:Time=Time{ 
       timespecinner: Mutex::new(TimeSpec{
         sec: 0,
        nsec: 0,
  })
    };
}