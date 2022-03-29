#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(const_in_array_repeat_expressions)]
#![feature(alloc_error_handler)]

extern crate  alloc;
use alloc::sync::{Arc};

#[macro_use]
extern crate console_support;
extern crate dynamic_malloc_support;

use  riscv::register::sstatus::{set_fs,FS, self};
use dynamic_malloc_support::*;
use bus::*;
use timer::{Timer};
use process::{Task};
use filesystem::{Filesystem};
use virtual_memory::{init, VirtualMemory,VirtualMemory2,VirtualMemory3,MemoryWritev};



global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));


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
    let mut businner=BUS.acquire_inner_lock();
    businner.register_table.push( Some(Arc::new(Filesystem)));
    businner.register_table.push( Some(Arc::new(Timer)));
    businner.register_table.push( Some(Arc::new(Task)));
    businner.register_table.push( Some(Arc::new(VirtualMemory)));
    businner.register_table.push( Some(Arc::new(VirtualMemory2)));
    businner.register_table.push( Some(Arc::new(VirtualMemory3)));
    businner.register_table.push( Some(Arc::new(MemoryWritev)));
   
    drop(businner);
    let m=Message{
        mod_id:TASK,
        body:[TASK_INIT,0, 0],
    };
    send(m);
    forward::init();
    forward::enable_timer_interrupt();
    let m=Message{
        mod_id:TIMER,
        body:[TIMER_SETNEXT,0, 0],
    };
    send(m);
    let m=Message{
        mod_id:TASK,
        body:[TASK_LOADAPP,0, 0],
    };
    send(m);
   let m=Message{
        mod_id:TASK,
        body:[TASK_RUNFIRST,0, 0],
    };
    send(m);
    panic!("Unreachable in rust_main!");
}