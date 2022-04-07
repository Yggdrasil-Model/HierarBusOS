#![no_std]
#![feature(llvm_asm)]
pub mod config;
pub use config::*;

pub mod controller;
pub use controller::*;

 
 #[macro_use]
extern crate console_support;

extern crate dynamic_malloc_support;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimeSpec {
    pub sec: usize,
    pub nsec: usize,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IoVec {
    /// Starting address
   pub base: *mut u8,
    /// Number of bytes to transfer
    pub len: usize,
}