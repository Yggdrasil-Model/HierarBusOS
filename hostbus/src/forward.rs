

use crate::config::*;
use crate::controller::*;
use riscv::register::mstatus::set_fs;
//use lazy_static::*;
use riscv::register::sstatus::{Sstatus, self, SPP,FS};
#[derive(Copy, Clone)]
#[repr(C)]
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
    sie,
    sepc,
};




pub fn init() {
    set_kernel_trap_entry();
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

pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer(); }
}
#[no_mangle]
pub fn mess_handler() ->! {
    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();
    let spec=sepc::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
           /* cx.sepc += 4;
            let m=Message{
                mod_id:cx.x[17],
                body:[cx.x[10], cx.x[11], cx.x[12]],
            };
            cx.x[10] =send(m) as usize;*/
            //cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            let m=Message{
                mod_id:TASK,
                body:[TASK_ENVCALL,0, 0],
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
            let exitcode=-2;
            let m=Message{
                mod_id:TASK,
                body:[TASK_EXIT_RUNNEXT,exitcode as usize,0],
            };
           send(m);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, core dumped. bad addr = {:#x}",spec);
            let exitcode=-3;
            let m=Message{
                mod_id:TASK,
                body:[TASK_EXIT_RUNNEXT,exitcode as usize,0],
            };
           send(m);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            //set_next_trigger();
            //suspend_current_and_run_next();
            let m=Message{
                mod_id:TIMER,
                body:[TIMER_SETNEXT,0,0],
            };
           send(m);
           let m=Message{
            mod_id:TASK,
            body:[TASK_SUSPEND_RUNNEXT,0,0],
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
    let m=Message{
        mod_id:TASK,
        body:[TASK_TOKEN,0, 0],
    };
    let user_satp = send(m) as usize;
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
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