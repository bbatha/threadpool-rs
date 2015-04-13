#![feature(std_misc)]
use std::sync::Semaphore;
use std::thread;

pub struct ScopedThreadPool {
    sem: std::sync::Semaphore,
}

impl ScopedThreadPool {
    pub fn new(size: isize) -> ScopedThreadPool {
       ScopedThreadPool { sem: std::sync::Semaphore::new(size) }
    }

    pub fn execute<'a, T, F>(&'a self, func: F) -> std::thread::JoinGuard<'a, T>
        where T: Send + 'a, F: FnOnce() -> T, F: Send + 'a
    {
       let guard = self.sem.access();
       thread::scoped(move || {
           let _guard = guard;
           func()
       })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    #[test]
    // TODO find a better way to test than have someone pay attention to the commandline...
    fn it_works() {
        let pool = ScopedThreadPool::new(2);

        let mut guards: Vec<thread::JoinGuard<()>> = Vec::new();

        for _ in (0..3) {
            guards.push(pool.execute(move || {
                thread::sleep_ms(1000);
                println!("Things are happening!");
            }));
        }
    }
}
