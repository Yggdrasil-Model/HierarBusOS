pub const FILE_SYSTEM:usize=0;
pub const FS_WRITE: usize = 64;
pub const FS_READ:usize=1;

pub const TASK:usize=2;
pub const TASK_EXIT_RUNNEXT:usize=22;
pub const TASK_SUSPEND_RUNNEXT:usize=23;
pub const TASK_EXIT:usize=24;
pub const TASK_GETPID:usize=211;
pub const TASK_FORK:usize=212;
pub const TASK_EXEC:usize=213;
pub const TASK_WAITPID:usize=214;


pub const TIMER:usize=1;
pub const TIMER_GET:usize=31;
pub const TIMER_SETNEXT:usize=32;


pub const CLOCK_FREQ: usize = 12500000;