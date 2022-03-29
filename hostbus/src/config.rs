pub const FILE_SYSTEM:usize=0;
pub const FS_WRITE: usize = 64;
pub const FS_INIT: usize = 11;
pub const FS_READ:usize=1;
pub const FS_WRITEV: usize = 66;


pub const TASK:usize=2;
pub const TASK_EXIT_RUNNEXT:usize=22;
pub const TASK_SUSPEND_RUNNEXT:usize=23;
pub const TASK_EXIT:usize=24;
pub const TASK_INIT: usize = 25;
pub const TASK_LOADAPP: usize = 26;
pub const TASK_RUNFIRST: usize = 27;
pub const TASK_TOKEN:usize=28;
pub const TASK_TRAP_CX:usize=29;
pub const TASK_ENVCALL:usize=21;
pub const TASK_GETPID:usize=211;
pub const TASK_FORK:usize=212;
pub const TASK_EXEC:usize=213;
pub const TASK_WAITPID:usize=214;

pub const TIMER:usize=1;
pub const TIMER_GET:usize=31;
pub const TIMER_SETNEXT:usize=32;
pub const TIMER_INIT: usize = 33;
pub const TIMER_SET:usize=34;


pub const CLOCK_FREQ: usize = 12500000;

pub const VIRTUALMEMORY_WRITE:usize=3;
pub const VIRTUALMEMORY_WRITEV:usize=6;
pub const VIRTUALMEMORY_READ:usize=4;

pub const LINUX_SET_TID_ADDRESS:usize=96;
pub const LINUX_WRITE:usize=64;
pub const LINUX_EXIT:usize=93;
pub const LINUX_EXIT_GROUP:usize=94;
pub const LINUX_CLOCK_GETTIME:usize=113;
pub const LINUX_GETPID:usize=172;
pub const LINUX_FORK:usize=220;
pub const LINUX_EXEC:usize=221;
pub const LINUX_WAITPID:usize=260;
pub const LINUX_READ:usize=63;
pub const LINUX_MAP:usize=222;
pub const LINUX_UNMAP:usize=215;

pub const CLOCK_WRITE:usize=5;



//task
//pub const USER_STACK_SIZE: usize = 4096 * 2;
//pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
//pub const MAX_APP_NUM: usize = 4;
//pub const APP_BASE_ADDRESS: usize = 0x80400000;
//pub const APP_SIZE_LIMIT: usize = 0x20000;

//virtual_memory
/*pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
pub const MEMORY_END: usize = 0x80b00000;
pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}*/

//process manager
pub const USER_STACK_SIZE: usize = 4096 * 4;
pub const KERNEL_STACK_SIZE: usize = 4096 * 4;
pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
pub const MEMORY_END: usize = 0xff000000;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct IoVec {
    /// Starting address
   pub base: *mut u8,
    /// Number of bytes to transfer
    pub len: usize,
}