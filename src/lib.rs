#![feature(std_misc)]
use std::sync::Semaphore;
use std::thread;

pub struct ThreadPool {
    sem: std::sync::Semaphore,
}

impl ThreadPool {
    pub fn new(size: isize) -> ThreadPool {
       ThreadPool { sem: std::sync::Semaphore::new(size) }
    }

    pub fn spawn<'a, T, F>(&'a self, func: F) -> std::thread::JoinGuard<'a, T>
        where T: Send + 'a, F: FnOnce() -> T, F: Send + 'a
    {
       let guard = self.sem.access();
       thread::scoped(move || {
           let _guard = guard;
           func()
       })
    }
}

#[test]
fn it_works() {
    let pool = ThreadPool::new(2);
    pool.spawn(move || {
        std::thread::sleep_ms(1000);
        println!("Done!");
    });

    pool.spawn(move || {
        std::thread::sleep_ms(1000);
        println!("Done!");
    });

    pool.spawn(move || {
        std::thread::sleep_ms(1000);
        println!("Done!");
    })
}
