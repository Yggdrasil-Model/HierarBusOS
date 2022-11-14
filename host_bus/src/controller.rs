extern crate alloc;

use core::borrow::BorrowMut;

use spin::{Mutex,MutexGuard};
use alloc::{vec};
use alloc::vec::Vec;
use lazy_static::*;
use alloc::sync::{Arc};
use riscv::register::sstatus::{Sstatus, self, SPP};
use riscv::register::{
    mtvec::TrapMode,
    stvec,
    scause::{
        self,
        Trap,
        Exception,
        Interrupt,
    },
    stval,
    sepc,
};
use super::config::*;
use user_bridge::*;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Message{
    pub node_id: usize,
    pub service_id: usize,
    pub body: [usize;3],
}

impl  Message{
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
    inner:Mutex<BusInner>,
    
 }

 #[repr(C)]
 pub struct BusInner{
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
    pub fn register(&self,node:Arc<dyn Busadapter + Send + Sync>){
        self.inner.lock().register_table.push( Some(node));
    }
    pub fn dispatch(&self,m: Message)->isize{
        let inner = self.acquire_inner_lock();
        let mut ret: isize = -1;
        if m.node_id >= inner.register_table.len(){
            println!("wrong node id!{}", m.node_id);
        }
        else if let Some(s) = &inner.register_table[m.node_id]{
        let point = s.clone();
        drop(inner);
        ret = point.handle(m.service_id, m.body);
        }
        ret
    }
    
}

pub fn send(m: Message) -> isize {
    BUS.dispatch(m)
}

pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub kernel_satp: usize,
    pub kernel_sp: usize,
    pub trap_handler: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) { self.x[2] = sp; }
    pub fn app_init_context(
        entry: usize,
        sp: usize,
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            kernel_satp,
            kernel_sp,
            trap_handler,
        };
        cx.set_sp(sp);
        cx
    }
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}


#[no_mangle]
pub fn mess_handler() ->! {
    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();
    let spec=sepc::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            let m = Message {
                node_id: TASK,
                service_id: TASK_ENVCALL,
                body: [0, 0, 0],
            };
            send(m);
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) |
        Trap::Exception(Exception::InstructionFault) |
        Trap::Exception(Exception::InstructionPageFault) |
        Trap::Exception(Exception::LoadFault) |
        Trap::Exception(Exception::LoadPageFault) => {
            println!("[kernel] {:?} in application in application, bad addr = {:#x} {:#x } core dumped.",scause.cause(),spec,stval);
            let exitcode =-2;
            let m = Message {
                node_id: TASK,
                service_id: TASK_EXIT_RUNNEXT,
                body: [exitcode as usize, 0, 0],
            };
            send(m);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, core dumped. bad addr = {:#x}",spec);
            let exitcode=-3;
            let m = Message {
                node_id: TASK,
                service_id: TASK_EXIT_RUNNEXT,
                body: [exitcode as usize, 0, 0],
            };
            send(m);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            //set_next_trigger();
            //suspend_current_and_run_next();
            let m = Message {
                node_id: TIMER,
                service_id: TIMER_SETNEXT,
                body: [0,0,0],
            };
            send(m);
            let m = Message {
                node_id: TASK,
                service_id: TASK_SUSPEND_RUNNEXT,
                body: [0,0,0],
            };
            send(m);
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let m = Message {
        node_id: TASK,
        service_id: TASK_TOKEN,
        body: [0, 0, 0],
    };
    let user_satp = send(m) as usize;
    /*extern "C" {
        fn __alltraps();
        fn __restore();
    }*/
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    
    unsafe {
        //llvm_asm!("fence.i" :::: "volatile");
        llvm_asm!("jr $0" :: "r"(restore_va), "{a0}"(trap_cx_ptr), "{a1}"(user_satp) :: "volatile");
    }
    panic!("Unreachable in back_to_user!");
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}