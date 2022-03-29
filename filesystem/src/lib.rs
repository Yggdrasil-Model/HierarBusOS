#![no_std]
use bus::*;
extern crate  alloc;
use alloc::sync::{Arc};
const FD_STDOUT: usize = 1;
use alloc::vec::Vec;

extern crate console_support;


pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
         match fd {
             FD_STDOUT => {
              let m=Message{
                mod_id:TASK,
                body:[TASK_TOKEN,0, 0],
            };
            let token = send(m) as usize;
            let m=Message{
                mod_id:VIRTUALMEMORY_WRITE,
                body:[token,buf as usize,len],
            };
            send(m);
            len as isize     
        },
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

pub fn sys_writev(fd: usize, iov_ptr: *const IoVec, iov_count: usize)->isize{
    match fd{
        FD_STDOUT=>{let m=Message{
            mod_id:TASK,
            body:[TASK_TOKEN,0, 0],
        };
        let token = send(m) as usize;
        let m=Message{
            mod_id:VIRTUALMEMORY_WRITEV,
            body:[token,iov_ptr as usize,iov_count],
        };
        send(m)   
    },
        _ => {
            panic!("Unsupported fd in sys_write!");
        }

    }

}

pub fn sys_read(buf: *const u8, len: usize) -> isize {
    assert_eq!(len, 1, "Only support len = 1 in sys_read!");
    let m=Message{
        mod_id:TASK,
        body:[TASK_TOKEN,0, 0],
    };
    let token = send(m) as usize;
    let m=Message{
        mod_id:VIRTUALMEMORY_READ,
        body:[token,buf as usize,len],
    };
    send(m)
}

pub struct Filesystem;
fn init(){
    let mut businner=BUS.acquire_inner_lock();
    businner.register_table.push( Some(Arc::new(Filesystem)));
    drop(businner);
}
impl Busadapter for Filesystem {
    fn handle(&self,body:[usize;3])->isize{
        match body[0]{
        FS_WRITE=>{sys_write(FD_STDOUT, body[1] as *const u8, body[2])}
        FS_INIT=>{init();1}
        FS_READ=>{sys_read(body[1] as *const u8, body[2])}
        FS_WRITEV=>{sys_writev(FD_STDOUT,body[1] as *const IoVec, body[2])}
        _ => {panic!("Unsupported syscall_id: ");}
    }
    }
}

