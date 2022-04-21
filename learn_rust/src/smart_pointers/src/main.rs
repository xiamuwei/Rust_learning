use std::thread;
use std::time::Duration;

fn main() {
    let foo:Vec<i32> = Vec::new();

    thread::spawn(||{
        thread::sleep(Duration::from_millis(20));
    });
}




