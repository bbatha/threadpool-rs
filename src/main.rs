extern crate threadpool;

use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(2);

    let guard = pool.spawn(move || {
        std::thread::sleep_ms(1000);
        println!("Things are happening!");
    });
    println!("Outside the thread is ok!");
    let guard2 = pool.spawn(move || {
        std::thread::sleep_ms(1000);
        println!("Things are happening!");
    });
    println!("Outside the thread is ok!");
    let guard3 = pool.spawn(move || {
        std::thread::sleep_ms(1000);
        println!("Things are happening!");
    });
    println!("Outside the thread is ok!");
}
