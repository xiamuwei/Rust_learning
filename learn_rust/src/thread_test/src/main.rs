
use std::thread;
use std::time::Duration;
fn main() {
    // println!("Hello, world!");
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("{}",i);
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("this is main thread");
    handle.join().unwrap();
}


// 线程之间通讯
