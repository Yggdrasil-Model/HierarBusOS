use super::config::*;
use super::controller::*;
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    let m = Message {
        node_id: FILE_SYSTEM,
        service_id: FS_WRITE,
        body: [fd, buffer.as_ptr() as usize, buffer.len()],
    };
    send(m)
 }
 
 pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    //syscall(SYSCALL_READ, [fd, buffer.as_mut_ptr() as usize, buffer.len()])
    let m = Message {
        node_id: FILE_SYSTEM,
        service_id: FS_READ,
        body: [fd, buffer.as_mut_ptr() as usize, buffer.len()],
    };
    send(m)
}

pub fn sys_exit(exit_code: i32) -> ! {
    //syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
    let m = Message {
        node_id: TASK,
        service_id: TASK_EXIT,
        body: [exit_code as usize, 0, 0],
    };
    send(m);
    panic!("not arrive");
}

pub fn sys_yield() -> isize {
   // syscall(SYSCALL_YIELD, [0, 0, 0])
    let m = Message {
        node_id: TASK,
        service_id: TASK_SUSPEND_RUNNEXT,
        body: [0,0, 0],
    };
    send(m)
}

pub fn sys_get_time() -> isize {
    //syscall(SYSCALL_GET_TIME, [0, 0, 0])
    let m = Message {
        node_id: TIMER,
        service_id: TIMER_GET,
        body: [0, 0, 0],
    };
    send(m)
}

pub fn sys_getpid() -> isize {
    //syscall(SYSCALL_GETPID, [0, 0, 0])
    let m = Message {
        node_id: TASK,
        service_id: TASK_GETPID,
        body: [0, 0, 0],
    };
    send(m)
}

pub fn sys_fork() -> isize {
   // syscall(SYSCALL_FORK, [0, 0, 0])
    let m = Message {
        node_id: TASK,
        service_id: TASK_FORK,
        body: [0, 0, 0],
    };
    send(m)
}

pub fn sys_exec(path: &str) -> isize {
    //syscall(SYSCALL_EXEC, [path.as_ptr() as usize, 0, 0])
    let m = Message {
        node_id: TASK,
        service_id: TASK_EXEC,
        body: [path.as_ptr() as usize, 0, 0],
    };
    send(m)
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    //syscall(SYSCALL_WAITPID, [pid as usize, exit_code as usize, 0])
    let m = Message {
        node_id: TASK,
        service_id: TASK_WAITPID,
        body: [pid as usize, exit_code as usize, 0],
    };
    send(m)
}