#![no_std]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(const_in_array_repeat_expressions)]
#[macro_use]
extern crate console_support;
global_asm!(include_str!("switch.S"));

use lazy_static::*;

extern crate  alloc;
use alloc::sync::{Arc};
use bus::*;

pub mod loader;
pub mod context;
pub mod switch;
pub mod taskcontroll;
pub mod pid;
pub mod manager;
pub mod processor;

pub use loader::*;
pub use context::*;
pub use switch::*;
pub use taskcontroll::*;
pub use pid::*;
pub use manager::*;
pub use processor::*;
use memory_manager::{
    translated_str,
    translated_refmut,
    frame_none,
    MapPermission,
    MapArea,
    MapType
};

use alloc::vec::Vec;

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- hold current PCB lock
    let mut task_inner = task.acquire_inner_lock();
   // println!("pre task id:{}", task.getpid());
    let task_cx_ptr2 = task_inner.get_task_cx_ptr2();
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB lock

    // push back to ready queue.
    add_task(task);
   
    // jump to scheduling cycle
    schedule(task_cx_ptr2);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** hold current PCB lock
    let mut inner = task.acquire_inner_lock();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ hold initproc PCB lock here
    {
        let mut initproc_inner = INITPROC.acquire_inner_lock();
        for child in inner.children.iter() {
            child.acquire_inner_lock().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB lock here

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** release current PCB lock
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let _unused: usize = 0;
    schedule(&_unused as *const _);
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(
        TaskControlBlock::new(get_app_data_by_name("initproc").unwrap())
    );
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.acquire_inner_lock().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().unwrap();
    // find a child process

    // ---- hold current PCB lock
    let mut inner = task.acquire_inner_lock();
    if inner.children
        .iter()
        .find(|p| {pid == -1 || pid as usize == p.getpid()})
        .is_none() {
        return -1;
        // ---- release current PCB lock
    }
    let pair = inner.children
        .iter()
        .enumerate()
        .find(|(_, p)| {
            // ++++ temporarily hold child PCB lock
            p.acquire_inner_lock().is_zombie() && (pid == -1 || pid as usize == p.getpid())
            // ++++ release child PCB lock
        });
    if let Some((idx, _)) = pair {
        let child = inner.children.remove(idx);
        // confirm that child will be deallocated after removing from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        // ++++ temporarily hold child lock
        let exit_code = child.acquire_inner_lock().exit_code;
        // ++++ release child PCB lock
        *translated_refmut(inner.memory_set.token(), exit_code_ptr) = exit_code;
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB lock automatically
}

pub fn sys_mmap(start:usize,len:usize,port:usize)->isize{
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let  fra_none=frame_none();
    if port&0x7==0 ||fra_none||port&(!0x7)!=0||start%4096!=0 {      
        panic!("invalid mmap");
    }
    else {
        let mut map_perm = MapPermission::U;
            if port&1==1{map_perm |= MapPermission::R;}
            if port&2==2{map_perm |= MapPermission::W;}
            if port&4==4{map_perm |= MapPermission::X;}
        if inner.memory_set.compare(start.into(),  (start+len/8).into()){
            panic!("repeatly mmap");
       }
        let maparea=MapArea::new( start.into(),  (start+len/8).into(),MapType::Framed, map_perm);
        inner.memory_set.push(maparea, None);
        
    }
    (len as isize -1+4096)/4096*4096
}
pub fn sys_unmap(start:usize,len:usize)->isize{
    let task = current_task().unwrap();
    let mut inner = task.acquire_inner_lock();
    let i= inner.memory_set.unmmap(start.into(),  (len-1+4096)/4096);
    i
}
pub struct Task;


impl Busadapter for Task{
    fn handle(&self, service_id: usize, body: [usize;3])->isize{
        match service_id{
            TASK_EXIT_RUNNEXT => 
            { 
                exit_current_and_run_next(body[0] as i32);
                1 
            }
            TASK_SUSPEND_RUNNEXT => 
            { 
                suspend_current_and_run_next();
                1
            }
            TASK_EXIT=>
            {
                println!("[kernel] Application exited with code {}", body[0] as f64);
                exit_current_and_run_next(body[0] as i32);
                panic!("Unreachable in sys_exit!");
            }
            TASK_INIT =>
            {
                add_initproc();
                1
            }
            TASK_RUNFIRST =>
            {
                run_tasks();
                1
            }
            TASK_TOKEN =>
            {
                current_user_token() as isize
            }
            TASK_TRAP_CX =>
            {
                let ret = current_trap_cx();
                1
            }
            TASK_ENVCALL =>
            {
                let cx = current_trap_cx();
                cx.sepc += 4;
                match cx.x[17]{
                    LINUX_GETPID => {
                        cx.x[10] = current_task().unwrap().pid.0;
                        //println!("{}",cx.x[10]);
                    }
                    LINUX_SET_TID_ADDRESS => {
                        cx.x[10] = 0 as usize;
                    }
                    LINUX_WRITE => {
                        let m = Message {
                            node_id: MEMORY_MANAGER,
                            service_id: MEMORY_WRITE,
                            body: [current_user_token(), cx.x[11], cx.x[12]],
                        };
                        cx.x[10] = send(m) as usize;
                    }
                    LINUX_EXIT => {
                        println!("[kernel] Application exited with code {}", cx.x[10] as f64);
                        exit_current_and_run_next(cx.x[10] as i32);
                        panic!("Unreachable in sys_exit!");
                    }
                    LINUX_EXIT_GROUP|135|178 => {
                        cx.x[10]=0;   
                    }
                    LINUX_CLOCK_GETTIME => {
                    //method 1
                        let m = Message {
                            node_id: TIMER,
                            service_id: TIMER_GET,
                            body: [0, 0, 0],
                        };
                       let ms = send(m) as usize;
                        /*  let e:isize;
                        let s:isize;
                        let mut ms:usize;
                        unsafe{ 
                            llvm_asm!("rdtime  $0" : "=r"(s)  :: "volatile");
                        }
                        for _ in 0..100{
                            ms = send(m) as usize;
                        }
                        unsafe{ 
                            llvm_asm!("rdtime  $0" : "=r"(e)  :: "volatile");
                        }
                        println!("time:{}",(e-s)/100);*/
                       let buffer = translated_refmut(current_user_token(), cx.x[11] as *mut TimeSpec);
                       buffer.sec = ms/1000;
                       buffer.nsec = ms%1000*1000000;
                    }
                    FS_WRITEV => {
                    //println!("enter writev:{} {:#x} {}",cx.x[10],cx.x[11],cx.x[12]);
                        let m = Message {
                            node_id: MEMORY_MANAGER,
                            service_id: MEMORY_WRITEV,
                            body: [current_user_token(), cx.x[11], cx.x[12]],
                        };
                        cx.x[10] = send(m) as usize;
                    }         
                    LINUX_FORK => {
                        cx.x[10] = sys_fork() as usize;
                    //println!("fork"); 
                    }
                    LINUX_EXEC => {
                        cx.x[10] = sys_exec(cx.x[10] as *const u8) as usize;
                    //println!("exec");
                    }
                    LINUX_WAITPID => {
                        cx.x[10] =sys_waitpid(cx.x[10] as isize, cx.x[12] as *mut i32) as usize;
                   // println!("waitpid");
                    }
                    LINUX_READ => {
                        let m = Message {
                        node_id: MEMORY_MANAGER,
                        service_id: MEMORY_READ,
                        body: [current_user_token(),cx.x[11], cx.x[12]],
                    };
                    cx.x[10] = send(m) as usize;
                    
                    }
                    LINUX_MAP => {
                        cx.x[10] = sys_mmap(cx.x[10], cx.x[11], cx.x[12]) as usize;
                    }
                    LINUX_UNMAP => {
                        cx.x[10] = sys_unmap(cx.x[10], cx.x[11]) as usize;
                    }
                     _ => {
                        let m = Message {
                            node_id: cx.x[17],
                            service_id: cx.x[13],
                            body: [cx.x[10], cx.x[11], cx.x[12]],
                        };
                        cx.x[10] = send(m) as usize;
                     }
                }
                0
            }
            TASK_GETPID => 
            {
                current_task().unwrap().pid.0 as isize
            }
            TASK_FORK =>
            {
                sys_fork()
            }
            TASK_EXEC =>
            {
                sys_exec(body[0]  as *const u8)
            }
            TASK_WAITPID =>
            {
                sys_waitpid(body[0] as isize, body[1] as *mut i32)
            }
            TASK_LOADAPP =>
            {
                list_apps();
                1
            }
            _ => {println!("TaskNode Unsupported service_id: ");-1
        }
    }
}
}