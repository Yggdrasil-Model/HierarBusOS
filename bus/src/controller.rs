extern  crate alloc;
use spin::{Mutex,MutexGuard};
use alloc::vec;
use alloc::vec::Vec;
use lazy_static::*;
use alloc::sync::{Arc};
use bridge::*;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Message{
    pub mod_id:usize,
    pub body:[usize;3],
}

impl  Message{
    pub fn new(mod_id:usize,body:[usize;3])->Self{
        let  m=Self{
            mod_id,
            body,
        };
        m
    }
}

pub trait  Busadapter {
    fn handle(&self,body:[usize;3])->isize;
}

pub struct Bus{
    inner:Mutex<BusInner>
 }
 pub struct  BusInner{
     pub register_table:Vec<Option<Arc<dyn Busadapter + Send + Sync>>>,
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
    pub fn register(&self)->usize{
        let mut inner=self.acquire_inner_lock();
        inner.register_table.push( None);
        inner.register_table.len()-1
    }
    pub fn dispatch(&self,m:Message)->isize{
        let  inner=self.acquire_inner_lock();
        let mut ret:isize=0;
        let id=(m.mod_id-256) as isize;
        if id<0{
            ret=forward(m.mod_id,m.body);
        }
        else{
            if id >=inner.register_table.len()  as isize{
                panic!("wrong module id!");
            }
            if let  Some(s)=&inner.register_table[id as usize]{
            let point=s.clone();
            drop(inner);
            ret=point.handle(m.body);
        }
        }
        ret
    }
    
}

pub fn send(m:Message)->isize{
    BUS.dispatch(m)
}