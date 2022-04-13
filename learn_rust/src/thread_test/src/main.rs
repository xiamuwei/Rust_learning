
use std::time::Duration;
use std::sync::{mpsc,Arc, Mutex};
use std::thread;
use std::rc::Rc;
fn main() {
    mutex_test();

    thread::spawn(||{
        thread_test();
    });
    for i in 0..10{
        println!("this is a main test");
    }
}

fn thread_test(){
    println!("this is a test for thread");
}

// 定义线程
// println!("Hello, world!");f
fn thread_new(){
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


    let num = 10;   
    thread::spawn(move ||{
        println!("num = {}",num);
    });
}


// 线程之间通讯
fn thread_chat(){
    // 使用move关键字，强制闭包获取其使用的环境值的所有权
    let (tx, rx) = mpsc::channel();
    thread::spawn(move ||{
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });

    let rec = rx.recv().unwrap();
    println!("Got :{}",rec);
}


fn thread_chat1(){
    // 实现线程通讯，使用mspc, 返回元组
    let (tx,rx) = mpsc::channel();

    // 使用线程发送消息
    thread::spawn(move ||{
        tx.send("happy").unwrap();
    });

    let rec = rx.recv().unwrap();
    println!("{}",rec);
}


// 共享状态
fn mutex_test(){
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num =6 ;
    }
    println!("m = {:?}",m);
}


fn mutex_test_1(){
    let counter =Arc::new( Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle =  thread::spawn( move||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result :{}", *counter.lock().unwrap());
}