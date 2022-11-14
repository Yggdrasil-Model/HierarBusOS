use spin::{Mutex,MutexGuard};
use alloc::vec;
use alloc::vec::Vec;
use lazy_static::*;
use alloc::sync::{Arc};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Message{
    pub node_id: usize,
    pub service_id: usize,
    pub body: [usize;3],
}

impl Message{
    pub fn new(node_id: usize, service_id: usize, body: [usize;3]) -> Self{
        let m = Self{
            node_id,
            service_id,
            body,
        };
        m
    }
}

pub trait  Busadapter {
    fn handle(&self, service_id: usize, body: [usize;3]) -> isize;
}

pub struct Bus{
    inner: Mutex<BusInner>
 }
pub struct  BusInner{
    pub register_table: Vec<Option<Arc<dyn Busadapter + Send + Sync>>>,
}
unsafe impl Sync for Bus {}

lazy_static!{
    pub static ref BUS:Bus=Bus{ 
        inner: Mutex::new(BusInner{
              register_table:vec![],
        }
    )
};
}
impl Bus {
    pub fn acquire_inner_lock(&self) -> MutexGuard<BusInner> {
        self.inner.lock()
    }
    pub fn register(&self){
    }

    // Assuming only one case, i.e., forwarding to the host bus
    pub fn dispatch(&self, m: Message) -> isize {
        let ret: isize = forward(m.node_id, m.service_id, m.body);  
        ret
    }
}

pub fn send(m: Message) -> isize{
    //println!("this user bus");
    BUS.dispatch(m)
}


fn forward(id: usize, service_id: usize, args: [usize; 3])->isize{
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x13}"(service_id), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
} 
