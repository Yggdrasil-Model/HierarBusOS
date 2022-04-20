use spin::{Mutex, MutexGuard};
use riscv::register::*;
use  core::fmt;
//SpinNoIrqLock
#[derive(Default)]
pub struct  Lock<T>(Mutex<T>);
pub struct LockGuard<'a,T> {
    guard: Option<MutexGuard<'a,T>>,
}

impl <T> Lock<T> {
    pub fn new(obj:T) -> Self {
        Self(Mutex::new(obj))
    }
    pub fn lock(&self) -> LockGuard<'_,T>{
      unsafe {
        sstatus::clear_sie();
      }  
        LockGuard {
            guard: Some(self.0.lock()),
        }
    }
}
impl <'a,T> Drop for LockGuard<'a,T> {
    fn drop(&mut self) {
        self.guard.take();
        unsafe {
            sstatus::set_sie();
        }       
    }
}

impl <'a,T> core::ops::Deref for LockGuard<'a,T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.guard.as_ref().unwrap().deref()
    }
}

impl <'a,T> core::ops::DerefMut for LockGuard<'a,T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
      self.guard.as_mut().unwrap().deref_mut()
  }
}

impl <T:fmt::Debug> fmt::Debug for Lock<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guard = self.lock();
        write!(f, "Mutex {{ data: {:?},  }}",&*guard)
    }
}
