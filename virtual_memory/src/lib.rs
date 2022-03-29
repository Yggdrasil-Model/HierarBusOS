#![no_std]
#![feature(llvm_asm)]
#![feature(core_intrinsics)]
pub mod address;
pub mod page_table;
pub mod frame_allocator;
pub mod memory_set;


extern crate dynamic_malloc_support;
#[macro_use]
extern crate console_support;

#[macro_use]
extern crate bitflags;
extern crate alloc;
use console_support::sbi::*;

pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum};
pub use frame_allocator::{FrameTracker, frame_alloc,frame_none};
use page_table::translated_iovec;
pub use page_table::{PageTableEntry, translated_byte_buffer,translated_refmut,translated_str};
pub use memory_set::{MemorySet, KERNEL_SPACE, MapPermission,MapArea,MapType};
pub use memory_set::remap_test;
use bus::*;
pub fn init() {
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}

pub struct VirtualMemory;
impl Busadapter for VirtualMemory {
    fn handle(&self,body:[usize;3])->isize{
        let buffers=translated_byte_buffer(body[0], body[1] as *const u8 , body[2]);
        for buffer in buffers {
            print!("{}", core::str::from_utf8(buffer).unwrap());
        }
        1
    }
}

pub struct VirtualMemory2;
impl Busadapter for VirtualMemory2 {
    fn handle(&self,body:[usize;3])->isize{
        let mut c: usize;
            loop {
                c = console_getchar();
                if c == 0 {
                    let m=Message{
                        mod_id:TASK,
                        body:[TASK_SUSPEND_RUNNEXT,0, 0],
                    };
                    send(m);
                    continue;
                } else {
                    break;
                }
            }
            let ch = c as u8;
        let mut buffers=translated_byte_buffer(body[0], body[1] as *const u8 , body[2]);
        unsafe { buffers[0].as_mut_ptr().write_volatile(ch); }
            1
    }
}

pub struct VirtualMemory3;
impl Busadapter for VirtualMemory3{
    fn handle(&self,body:[usize;3])->isize{
        let buffer=translated_refmut(body[0],body[1] as *mut TimeSpec);
       // println!("buffer:{:?}",buffer);
        //let inner=TIME.timespecinner.lock();
            //buffer.sec=inner.sec;
           // buffer.nsec=inner.nsec;
        buffer.sec=body[2]/1000;
        buffer.nsec=body[2]%1000*1000000;
       // println!("buffer:{:?}",buffer);
        0
    }
}

pub struct MemoryWritev;
impl Busadapter for MemoryWritev{
    fn handle(&self,body:[usize;3])->isize{
        let mut ret=0;
            let iovs=translated_iovec(body[0], body[1] as *const IoVec, body[2]);
            for iov in iovs.iter(){
                let buffers=translated_byte_buffer(body[0],iov.base,iov.len);
                ret+=iov.len;
                for buffer in buffers {
                    print!("{}", core::str::from_utf8(buffer).unwrap());
                }
            
            }
             println!("");
            ret as isize
        }
    }
    
    