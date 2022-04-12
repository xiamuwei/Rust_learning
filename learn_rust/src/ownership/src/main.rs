fn main() {
    println!("Hello, world!");

    let str1 = String::from("hello rust");
    move_owner(str1);
    // println!("{}",str1); //borrow of moved value: `str1`

    // 克隆
    let str2 = String::from("hello rust"); 
    let str3 = str2.clone();
    println!("str2 = {}, str3 = {}",str2,str3);

    // 拷贝
    let num = 13;
    stack_variable(num);
    println!("num = {}",num);


    let str4 = String::from("back");
    let str5 = move_owner_back(str4);


    // 借用
    let mut str6 = String::from("test value");
    get_value(&str6); // 不可变引用
    println!("{}",str6);

    let mut str7 = String::from("test value");
    get_value_mut(&mut str7);

    let a = &str6;
    let b = &str6;
    // let c = &mut str6;
    // println!("{}, {},{}", a, b,c);

    let d = &mut str7;
    // let e = &mut str7;
    // println!("{}, {}", d, e);

}

// 所有权移动
fn move_owner(str:String){
    println!("{}",str);
}

// 函数也可以通过返回值发生所有权转移
fn move_owner_back(str :String)->String{
    str
}

// 针对栈数据
fn stack_variable(num :i32){
    println!("{}",num);
}




// 不获取所有权,只获取变量值
fn get_value(s:&String)->i32{
    s.len() as i32
}   
// 不获取所有权,只获取变量值
fn get_value_mut(s:&mut String){
    s.push_str(" hello");
    println!("{}",s);
}   




