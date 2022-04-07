#![no_std]
#![feature(global_asm)]
global_asm!(include_str!("trap.S"));

extern "C" {
    pub fn __alltraps();
    pub fn __restore();
}