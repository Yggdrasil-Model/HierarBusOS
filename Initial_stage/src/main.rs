#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(const_in_array_repeat_expressions)]
#![feature(alloc_error_handler)]

extern crate alloc;
use alloc::sync::{Arc};

#[macro_use]
extern crate console_support;
extern crate dynamic_malloc_support;

use riscv::register::sstatus::{set_fs,FS, self};
use riscv::register::{
    mtvec::TrapMode,
    stvec,
    sie, 
};

use dynamic_malloc_support::*;
use bus::*;
use timer::{Timer};
use task_manager::{Task};
use filesystem::{Filesystem};
use memory_manager::{init, MemoryManager};



global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn kernel_trap() -> ! {
    panic!("a trap from kernel!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}


#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    unsafe{set_fs(FS::Initial);}
    if sstatus::read().fs()==FS::Initial{
        println!("set sstatus fs successfully");
    }
    println!("[kernel] Hello, world!");
    heap_allocator::init_heap();
    init();
    let mut businner = BUS.acquire_inner_lock();
    businner.register_table.push( Some(Arc::new(Filesystem)));
    businner.register_table.push( Some(Arc::new(Timer)));
    businner.register_table.push( Some(Arc::new(Task)));
    businner.register_table.push( Some(Arc::new(MemoryManager)));

    drop(businner);
    let m = Message {
        node_id: TASK,
        service_id: TASK_INIT,
        body: [0, 0, 0],
    };
    send(m);
    unsafe {
        stvec::write(kernel_trap as usize, TrapMode::Direct);
        sie::set_stimer();
    }
    let m = Message{
        node_id: TIMER,
        service_id: TIMER_SETNEXT,
        body: [0, 0, 0],
    };
    send(m);
    let m = Message {
        node_id: TASK,
        service_id: TASK_LOADAPP,
        body: [0, 0, 0],
    };
    send(m);
    let m = Message {
        node_id: TASK,
        service_id: TASK_RUNFIRST,
        body: [0, 0, 0],
    };
    send(m);
    panic!("Unreachable in rust_main!");
}