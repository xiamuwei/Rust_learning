// use std::io::stdin;
use std::io::prelude::*;
use std::io::Read;
use std::fs::{self,File,OpenOptions};
fn main() {
    wrtie_file_stream();
    // 获取命令行参数
    // let args = std::env::args();
    // for i in args {
    //     println!("{}",i);
    // }

    // // 命令行输入
    // let mut buf = String::new();
    // stdin().read_line(&mut buf).expect("failed to read line.");
    // println!("Your input line is \n{}",buf);
}

// 读取文件
// 一次性读取文件
fn read_file_once(){
    // 使用read_to_string 完成对文本文件的读取
    let content = fs::read_to_string("E:\\depository\\Rust_learning\\learn_rust\\src\\io-test\\src\\test").unwrap();
    println!("{}",content);

    // 二进制文件读取
    let content =  fs::read("E:\\depository\\Rust_learning\\learn_rust\\src\\io-test\\src\\test").unwrap();
    println!("{:?}",content);
}

// 使用文件流读取，适合大文件
fn read_file_stream(){
    let mut buf:[u8;5]= [0;5];
    //等价于 let mut buf= [0u8;5]; 
    //std::fs 模块中的 File 类是描述文件的类，可以用于打开文件，再打开文件之后，我们可以使用 File 的 read 方法按流读取文件的下面一些字节到缓冲区（缓冲区是一个 u8 数组），读取的字节数等于缓冲区的长度
    let mut file = fs::File::open("E:\\depository\\Rust_learning\\learn_rust\\src\\io-test\\src\\test").unwrap();
    file.read(&mut buf).unwrap();
    println!("{:?}", buf);
    // std::fs::File 的 open 方法是"只读"打开文件，并且没有配套的 close 方法，因为 Rust 编译器可以在文件不再被使用时自动关闭文件

}

// 文件写入
// 一次性写入
fn write_file_once(){
    fs::write("E:\\depository\\Rust_learning\\learn_rust\\src\\io-test\\src\\test.txt", "contents: C").unwrap();
}

// 流式写入
fn wrtie_file_stream(){
    let mut file = File::create("E:\\depository\\Rust_learning\\learn_rust\\src\\io-test\\src\\test1.txt").unwrap();
    file.write(b"buf: &[u8]").unwrap();

    // 因为File类中不存在append静态方法
    // 所以 可以使用OpenOptions  选择append，read，write权限等
    let mut file = OpenOptions::new().append(true).open("E:\\depository\\Rust_learning\\learn_rust\\src\\io-test\\src\\test.txt").unwrap();
    file.write(b"test2OpenOptions").unwrap();

    // 以读写权限打开一个文件
    let mut file = OpenOptions::new().read(true).write(true).open("E:\\depository\\Rust_learning\\learn_rust\\src\\io-test\\src\\test").unwrap();
    file.write(b"test3OpenOptions").unwrap();
}
