use bus::*;

/*fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]), "{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}*/

/*pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}*/

pub fn sys_write(buffer: &[u8]) -> isize {
    let m=Message{
     mod_id:FILE_SYSTEM,
     body:[FS_WRITE,buffer.as_ptr() as usize, buffer.len()],
 };
 send(m)
 }
 
 pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    //syscall(SYSCALL_READ, [fd, buffer.as_mut_ptr() as usize, buffer.len()])
    let m=Message{
        mod_id:FILE_SYSTEM,
        body:[FS_READ,buffer.as_mut_ptr() as usize, buffer.len()],
    };
    send(m)
}

pub fn sys_exit(exit_code: i32) -> ! {
    //syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
    let m=Message{
        mod_id:TASK,
        body:[TASK_EXIT,exit_code as usize, 0],
    };
    send(m);
    panic!("sys_exit never returns!");
}

pub fn sys_yield() -> isize {
   // syscall(SYSCALL_YIELD, [0, 0, 0])
   let m=Message{
    mod_id:TASK,
    body:[TASK_SUSPEND_RUNNEXT,0, 0],
};
send(m)
}

pub fn sys_get_time() -> isize {
    //yscall(SYSCALL_GET_TIME, [0, 0, 0])
    let m=Message{
        mod_id:TIMER,
        body:[TIMER_GET,0, 0],
    };
    send(m)
}

pub fn sys_getpid() -> isize {
    //syscall(SYSCALL_GETPID, [0, 0, 0])
    let m=Message{
        mod_id:TASK,
        body:[TASK_GETPID,0, 0],
    };
    send(m)
}

pub fn sys_fork() -> isize {
   // syscall(SYSCALL_FORK, [0, 0, 0])
   let m=Message{
    mod_id:TASK,
    body:[TASK_FORK,0, 0],
};
send(m)
}

pub fn sys_exec(path: &str) -> isize {
    //syscall(SYSCALL_EXEC, [path.as_ptr() as usize, 0, 0])
    let m=Message{
        mod_id:TASK,
        body:[TASK_EXEC,path.as_ptr() as usize, 0],
    };
    send(m)
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    //syscall(SYSCALL_WAITPID, [pid as usize, exit_code as usize, 0])
    let m=Message{
        mod_id:TASK,
        body:[TASK_WAITPID,pid as usize, exit_code as usize],
    };
    send(m)
}