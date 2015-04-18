#![feature(std_misc)]
#![feature(scoped)]
use std::sync::Semaphore;
use std::thread;

pub struct ScopedThreadPool<'a> {
    sem: std::sync::Semaphore,
    guards: std::vec::Vec<thread::JoinGuard<'a, ()>>,
}

impl<'a> ScopedThreadPool<'a> {
    pub fn new(size: u32) -> ScopedThreadPool<'a> {
        ScopedThreadPool {
            sem: std::sync::Semaphore::new(size as isize), /* as safe here? */
            guards: Vec::new(),
        }
    }

    pub fn execute<F>(&'a mut self, func: F)
        where F: FnOnce() -> (), F: Send + 'a
    {
        let pool_guard = self.sem.access();
        let join_guard = thread::scoped(move || {
            let _pg = pool_guard;
            func()
        });
        self.guards.push(join_guard);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    #[test]
    // TODO find a better way to test than have someone pay attention to the commandline...
    fn it_works() {
        let mut pool = ScopedThreadPool::new(2);

        for _ in (0..3) {
           pool.execute(move || {
                thread::sleep_ms(1000);
                println!("Things are happening!");
            });
        }
    }
}
