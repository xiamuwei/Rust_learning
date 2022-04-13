fn main() {
    // println!("{}",is_divisible_by(14,15));
    capture_function();
}


fn is_divisible_by (lhs:i32, rhs:i32) -> bool{
    if rhs == 0 {
        return false;
    }
    // return true;
    // 这是一个表达式，可以不用return关键字
    lhs % rhs == 0
}


// 一个'不'返回值的函数，实际上会返回一个单元类型()
fn fizzbuzz(n:i32)->(){  // 返回一个空元组类型，即单元类型
    if is_divisible_by(n,15){
        println!("fizzbuzz");
    }else if is_divisible_by(n, 5){
        println!("buzz");
    }else {
        println!("{}",n);
    }   
}

// 当函数返回()类型时，函数签名可以省略返回类型
fn fizzbuzz_to (n:i32){
    if is_divisible_by(n,15){
        println!("fizzbuzz");
    }else if is_divisible_by(n, 5){
        println!("buzz");
    }else {
        println!("{}",n);
    }   
}


fn closure_function(){
    let closure_annotated = |i:i32| -> i32{i+1};
    // 根据调用自动推导输入类型
    let closure_inferred = | i |  i+1;
    let i :i32= 1;
    closure_inferred(i);
    println!("i = {}",i);
}


fn capture_function(){
    // use std::mem;
    // let color = String::from("green");

    // let print = || println!("{}",color);
    // print();
   
    // let _reborrow = &color;
    // print();


    let mut x = 1;
    let mut t = ||{x+=1;println!("{}",x);};
    t();
    t();
}   


