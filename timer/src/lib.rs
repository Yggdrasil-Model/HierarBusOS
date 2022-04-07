#![no_std]
use console_support::*;
use bus::*;
use riscv::register::time;

extern crate  alloc;
use alloc::sync::{Arc};

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
//const NSEC_PER_MSEC:usize=1000000;

pub fn get_time() -> usize {
    time::read()
    
}

pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}
/*pub fn set_time(){
    let mut inner=TIME.timespecinner.lock();
    inner.sec=get_time_ms()/MSEC_PER_SEC;
    inner.nsec=get_time_ms()%MSEC_PER_SEC*NSEC_PER_MSEC;
}*/
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

/*pub fn sys_clock_gettime(clock: usize,  ts: *mut TimeSpec) ->isize{
    // println!("clock_gettime: clock: {:?}, ts: {:?}", clock, ts);
    let ms=get_time_ms();
      let m= Message {
         mod_id:TASK,
         body:[TASK_TOKEN,0, 0],
     };
     let token = send(m) as usize;
     let m=Message{
         mod_id:CLOCK_WRITE,
         body:[token,ts as usize,ms],
     };
     send(m);
     //println!("clock_gettime: clock: {:?}, ts: {:?}", clock, ts.nsec);
     0
 }
*/
pub struct Timer;
fn init(){
    let mut businner=BUS.acquire_inner_lock();
    businner.register_table.push( Some(Arc::new(Timer)));
    drop(businner);
}
impl Busadapter for Timer {
    fn handle(&self, service_id: usize, body:[usize;3])-> isize{
        match  service_id {
            TIMER_GET=>{get_time_ms() as isize},
            TIMER_SETNEXT=>{set_next_trigger();1}
            TIMER_INIT=>{init();1}
           // LINUX_CLOCK_GETTIME=>{sys_clock_gettime(body[0], body[1] as *mut TimeSpec)}
            _=>panic!("Unsupported  timer service id"),
        }
    }
    
}