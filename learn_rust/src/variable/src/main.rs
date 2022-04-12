use std::convert::From;


fn main() {
    
    let str1:String = String::from("this is test");
    let str2:&str = "this is happy";

    // 切片
    let slice = &str1[..4];
    println!("slice = {}", slice);

    // local_type()
    // complex_type()
}

fn local_type(){
     /*
    基本数据类型
    + 整型
    + 浮点型
    + 字符型
    + 布尔类型 
    */
    let int_value:i32 = 100;
    let float_value:f64 = 3.14;
    let char_value:char = 'C';
    let bool_value:bool = false;
    println!("int_value = {}, float_value = {}, char_value = {}, bool_value = {}", 
    int_value,float_value,char_value,bool_value);

    let num = char_value as i32;
    let c = 100u8 as char;
    println!("{}",num);
    println!("{}",c);
}

fn complex_type(){
    // 数组
    let arr1:[i32;5] = [1,2,3,4,5];
    let arr2:[i32;5] = [3;5] ;

    for i in arr1 {
        println!("i = {}",i)
    }
    
    // 元组
    let tuple_value = (1,"test",3.14);
    // 较短的元组可以直接使用调试方式打印
    println!("{:?}",tuple_value);
    // 可以使用元组下标来访问具体的值
    println!("{}",tuple_value.0);

    // 解构元组
    let (x,y,z) = tuple_value;
    println!("x = {}\ny = {}\nz = {}",x,y,z);
   
}

// From 和 Into trait
#[derive(Debug)]
struct Number{
    value :i32,
}
impl From<i32> for Number{
    fn from(item:i32)->Self{
        Number{value:item}
    }
}
// impl Into<i32> for Number{
//     fn into(self)->i32{
//         self.value
//     }
// }

fn type_exchange(){
    let num1 = 100;
    let n = Number::from(num1);

    // 默认为类型Number实现 From<i32>后，i32可以使用into方法，获取Number类型
    let num2 = 200;
    let n2:Number = n.into();
}