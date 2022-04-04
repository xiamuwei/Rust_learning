# Rust

rust是一门系统级编程语言，被设计为保证内存和线程安全

rust编程语言的目的是创建一个高度安全和并发的软件系统，强调安全性、并发、内存控制。不运行C和C++中会引起系统崩溃、内存泄漏和不安全代码根源的 空指针和悬垂指针

rust使用实现 (implemenet)、特征 (trait) 和结构化类型 (structured type) 而不是类 (class)

rust 做到了内存安全而无需有些编程语言中实现自动垃圾收集器的开销，这是通过所有权/ 借用机制、生命周期、类型系统来达到的



## 安装

+ 更新和卸载

  ```shell
  rustup update // 更新到最新版本
  rustup self uninstall // 卸载rust和rustup
  ```

+ 查看rust版本

  ```shell
  rustc --version
  ```

  

## 前言

rust文件通常以 .rs 扩展名结尾，如hello_world.rs

```rust
fn main (){
    println!("hello world");
}
```

编译并运行文件

```shell
rustc hello_world.rs
./hello_world
# 结果: hello world
```

注：rust是一门预编译语言，这意味着你可以编译一个程序，将编译后的可执行文件给别人，即使他们没有安装rust也可以运行程序

 

+ **Cargo**

cargo 是rust的构建系统和包管理器。

安装rust时会自动下载cargo，可通过以下命令查看cargo版本

```shell
cargo --version
```

使用cargo创建项目

```shell
cargo new hello_world //  hello_world 是rust项目名
```

得到以下图所示的目录结构

![cargo文件结构](E:\learning\Michael\截图\cargo文件结构.jpg)

+ 文件详细说明

  + src/main.rs

    源代码都在src目录下

    将普通项目转换为使用cargo：

    1. 把源文件拷贝到src目录下
    2. 创建Cargo.toml 并填写相应的配置

  + Cargo.toml

    ![cargotoml内容](E:\learning\Michael\截图\cargotoml内容.jpg)

    + package表明下面的语句用来配置一个package。name、version、edition分别表明项目的名称、版本、使用的rust大版本号
    + dependencies是罗列项目依赖。rust把代码所需要的库叫做依赖

  + .gitignore 

    初始化了一个Git仓库



+ 构建并运行Cargo项目
  + cargo run 如果之前编译过，并且源码没有改变，那么就会直接运行二进制文件
  + cargo check 检查代码，确保能够通过编译，但是不产生任何可执行文件。运行比cargo build快很多
  + cargo build [--release]   // 后面的--release参数表示编译正式发布的版本，不然就是开发版本





## 变量

rust默认情况下变量是不可变的，可以使用mut关键字使变量可变

可变变量

不可变变量

```rust
// 1、定义不可变变量,默认情况下
let var1 = 23;
// var1 = 3;  报错如下：cannot assign twice to immutable variable `var1`

// 2、如果想变量可变就使用mut关键字
let mut var2 = 32;
var2 = 34;
// var2 = "str"   报错如下：mismatched types，可变变量不可赋值不同的数据类型的值

// shadow 变量遮罩。相当于重新声明了一个变量，只是变量名之前使用过
let var1 = "str";  
println!("var1 = {}",var1);
```





常量

注：不可变变量和常量的区别



变量遮蔽 (shadow)



## 数据类型

rust是一种**静态类型**的语言，这意味着它必须在编译期知道所有变量的类型。编译器通常可以根据值和使用方式推导出我们想要使用的类型

+ 标量数据类型
  + 整型
  + 浮点型
  + 字符类型 char
  + 布尔类型
  
  ```rust
  fn main() {
      println!("Hello, world!");
  
      // 整型
      let integer:i32 = 10;
      // let integer = 10;  自动类型推导
      println!("{}",integer);
  
      // 浮点型
      let float:f32 = 3.2;
      println!("{}",float);
  
      // 字符型
      let character:char = 'c';
      println!("{}",character);
      
      // 布尔类型
      let boolean:bool = true;
      // let boolean:bool = 0; 布尔类型的值只能是true或者false
      println!("{}",boolean);
  }
  ```
  
+ 复合数据类型
  + 元组 tuple
  + 数组
  
  ```rust
  // 使用类型推导定义元组，等价于let tuple(i32, &str, bool) = (1,"str",false);
  let tuple = (1,"str",false);
  // 访问元组元素
  println!("tuple第一个元素 = {}",tuple.0);
  // 解构，将元组分解并赋值给三个变量
  let (x ,y ,z )= tuple;
  
  println!("x = {}, y = {}, z = {}",x,y,z);
  
  
  // 定义数组
  let arr = [1,2,3];
  // 访问数组元素
  println!("arr 第一个元素 = {}", arr[0])
  ```
  

> 注：
>
> + 对于原始数据类型，使用as关键字
>
>   as 运算符类似于C中的强制类型转换，区别在于，**它只能用于原始类型，并且它是安全的**
>
>   在 Rust 中，不同的数值类型是不能进行隐式转换的，可以使用as 进行转换
>
>   ```rust
>   let b: i64 = 1i32;
>   // 会提示错误如下
>   /*
>   error[E0308]: mismatched types
>    --> src\main.rs:2:18
>       |
>   2   |     let b: i64 = 1i32;
>       |                  ^^^^ expected i64, found i32
>   help: change the type of the numeric literal from `i32` to `i64`
>   */
>   
>   使用as进行类型转换
>   let b: i64 = 1i32 as i64;
>   ```
>
> + 对于复杂类型，如struct、enum等，使用From 和 Into trait实现
>
>   From 和 Into trait结构
>
>   ```rust
>   pub trait From<T> {
>       fn from(T) -> Self;
>   }
>   pub trait Into<T> {
>       fn into(self) -> T;
>   }
>   ```
>
>   一般来说，我们应该尽量优先选择实现From<T> 而不是 Into<T>，因为当你实现了 From<T>，这意味着你同时隐式实现了 Into<T> , 但是需要注意的是，反之不会
>
>   ```rust
>   fn main() {
>       println!("Hello, world!");
>       let b: Complex = 1.into();
>       println!("{:?}", b);
>   }
>   #[derive(Debug)]
>   struct Complex {
>       re: i32,
>       im: i32
>   }
>   
>   impl From<i32> for Complex{
>       fn from(re: i32) -> Self {
>           Complex{
>               re,
>               im:0
>           }
>       }
>   }
>   ```





## 流程控制

+ 判断语句

  + if 表达式， 可返回值并赋值给一个变量

    ```rust
    let number = 6;
    if number % 4 == 0{
        println!("number is divisible by 4");
    }else if number % 3 == 0 {
        println!("number is divisible by 3");
    }else {
        println!("i dont know");
    }
    
    // 可以将if表达式的结果赋值给一个变量
    let flag = true;
    let res = if flag {5} else {6};
    ```

+ 循环语句

  + loop	

    ```rust
    pub fn loop_function(){
        let mut count = 0 ;
        // 标签counting_up
        'counting_up :loop {
            let mut remaining = 10;
            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    // 退出外层循环
                    break 'counting_up;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("End count = {}", count);
    }
    ```

  + while

    ```rust
    pub fn while_fuction(){
        let mut number = 3;
        while number != 0 {
            println!("number = {}",number);
            number -= 1;
        }
    }
    ```

  + for

    ```rust
    pub fn for_function(){
        // for循环,包括左边的值，不包括右边的值
        // rev函数用于反转区间
        for i in (1..4).rev(){
            println!("i = {}",i);
        }
    }
    ```

    

  + break 和 continue 标签



## 函数

语句和表达式

语句：执行一些操作但不会返回值

表达式：计算并产生一个值

```rust
// 在函数签名中，必须声明每个参数的类型
fn test(a int ,b bool) -> bool{
    // 函数体
    let mut a = 6; // 语句
    a + 1  
}
```



## 函数式编程

函数式编程风格通常包含将函数作为参数值或其他函数的返回值，将函数赋值给变量以供之后执行等

+ 闭包
+ 迭代器





## 所有权

所有权是rust最为与众不同的特性，它让rust无需垃圾回收器即可保证内存安全。==需要注意的是所有权是针对堆上的数据来说的==



栈与堆

栈：大小固定，后进先出

堆：大小不固定，缺乏组织

+ 变量与数据交互方式

  + 移动 --> 堆上数据直接复制耗费性能，且同时指向相同内存区域存在二次释放问题，故设计了所有权

    ```rust
    pub fn move_owner(){
        let str1 = String::from("hello");
        // 此时str1的所有权转移到str1，到时候销毁值时不需要str考虑
        let var = str;
        // println!("str1 = {}",str1); //borrow of moved value: `str1`
        println!("var = {}",var);
    }	
    ```

  + 克隆

    如果确实需要深度复制堆上数据，我们可以使用clone 的通用函数

    ```rust
    pub fn clone_owner(){
        let str1 = String::from("hello");
        //
        let var = str1.clone();
        // 此时两个变量都可以正常使用
        println!("str1 = {}",str1); 
        println!("var = {}",var);
    }
    ```

  + 拷贝 -- **对于像整型这样的在编译时已经知道大小的类型都是存储在栈上的，所有拷贝其实际的值是快速的**

    所有的整型类型、布尔类型、浮点类型、字符类型char、当且仅当包含了实现Copy类型时的元组

    ```rust
    pub fn copy_owner(){
        let x = 1;
        let y = x;
        println!("x = {}, y = {}",x,y); 
    }
    ```

    



所有权与函数

将值传递给函数在语义上与给变量赋值相似，向函数传递值可能会发生移动或者复制

函数返回值也可以发生所有权的转移

```rust
pub fn owner_function(){
    // 对于堆上建立的变量，作为函数参数时会转移变量
    let s = String::from("hello world");
    // 字符串 s的所有权转移到函数中，后面无法使用
    take_owner(s);
    // println!("s = {}",s);  //value borrowed here after move
    
    let s1 = String::from("hello rust");
    let s1 = take_and_back_owner(s1);
    println!("s = {}",s1);

    let x = 1;
    make_copy(x);
    println!("x = {}",x);
}   

fn take_owner(str:String){
    println!("str = {}",str);
} // str的所有权已经转移到take_owner函数中，结束时会移出作用域并调用drop方法，释放占用的内存

fn make_copy(x:i64){
    println!("x = {}",x);
} 

// take_and_back_owner 将获取传入的字符串的所有权，并最后返回所有权
fn take_and_back_owner(str:String) -> String{
    println!("take_and_back_owner str ={}",str);
    str
}
```





引用 ：使用一个变量的值但不获得所有权

>  & 符号就是 **引用**，它们允许你使用值但不获取其所有权
>
> *符号是**解引用**，实现跟&相反的操作

```rust
pub fn referencing(){
    let s = String::from("hello rust");
    
    // &s 创建了一个指向值s的引用，但是并不会拥有它。因为并不会拥有这个值，所有当引用停止使用时，它所指向的值也不会被丢弃
    println!("s.len() = {}",calculate_length(&s));
}
// 函数以一个对象的引用作为参数 而不是获取值的所有权
fn calculate_length(str:&String) -> i32{
    str.len() as i32
}
```

引用跟变量一样，默认也是不可变的

可变引用

不可变引用

```rust
pub fn mut_referencing(){
    // 创建一个可变引用的前提是s 变量本身也是可变的，可修改的
    let mut s = String::from("hello rust");
    change_string(&mut s);
}

fn change_string(str:&mut String) {
    str.push_str(",hi");
    println!("str = {}",str);
}
```

为了避免数据竞争，规定引用，在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用



借用：创建一个引用的行为被称为借用





## 切片

slice 引用集合中一段连续的元素序列，而不是引用整个集合

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
```

注意：字符串字面量就是slice

```rust
let s = "Hello, world!";
```







## 结构体

结构体 的每一部分都可以是不同类型

+ 自定义结构体

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 创建结构体实例
// 注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变
let mut user1 = User{
  	email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,  
};

// 如果变量与字段名同名时可以使用简写语法
fn build_user(email: String, username: String) -> User {
    User {
        // 此处email、username的值和变量名一致，故可以省略
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 使用结构体更新语法从其他实例创建实例
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};

```

+ 元组结构体

  ```rust
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  
  fn main() {
      let black = Color(0, 0, 0);
      let origin = Point(0, 0, 0]);
  }
  ```

+ 类单元结构

  ```rust
  struct AlwaysEqual;
  
  fn main() {
      let subject = AlwaysEqual;
  }
  ```

  

方法

方法与函数类似：使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会调用的代码。但它们的第一个参数总是self，它代表调用该方法的结构体实例

关联参数

所有在impl块中定义的函数被称为关联函数，因为它们与impl后面命名的类型相关。

```rust
#[derive(Debug)]
struct Rectangle {
    width :i32,
    length :i32,
}
impl Rectangle{
    // 方法：第一个参数必须是&self
    fn area(&self) -> u32{
        (self.length * self.width) as u32
    }
    
    // 关联函数：不以self为第一参数的关联函数，并不是方法
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

}

pub fn struct_func(){
    let rec = Rectangle{
        width: 32,
        length :23,
    };

    println!("rec = {}",rec.area());
}
```









## 枚举和模式匹配

枚举 (enum) 允许你通过列举可能的成员来定义一个类型



Option

空值 (Null) 是一个值，它代表没有值。在有空值的语言中，变量总是这两种状态之一：空值和非空值

但是rust没有控制，而是使用一个枚举Option 来实现

```rust
enum Option<T> {
    Some(T),
    None,
}
```



模式匹配

match允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码

rust模式匹配是穷举式的，这意味着你必须覆盖所有可能的情况

但你可以使用other 覆盖其他所有可能的值，或者通过 _ 可以匹配任意值而不绑定到该值

```rust
pub enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn value_in_cents(coin:Coin) -> i32{
    match coin{
        Coin::Penny => {println!("Lucky penny!"); 1},
        Coin::Nickel => 5 ,
        Coin::Dime => 10 ,
        Coin::Quarter => 25 ,
    }

}

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other 分支的代码通过将其传递给 move_player 函数来使用这个变量
        other => move_player(other),
    }
    
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        
        _ => reroll(),
    }

}

```



if  let  结合if 和 let，来处理只匹配一个模式的值而忽略其他模式的情况，可用于简化match代码

```rust
#![allow(unused)]
fn main() {
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
}

// 等价于
#![allow(unused)]
fn main() {
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
}
```







## 常见集合

集合不同于内建的数组和元组，这些集合指向的数据是存储在堆上的，这意味着数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或者缩小

+ vector ：只能存储相同类型的值

  创建vector

  ```rust
  // 使用<i32> 指明此时vector内存的类型
  let vec:Vec<i32> = Vec::new();
  
  // 一旦插入值rust就可以推断出想要存放的类型
  let v = vec![1,2,3];
  ```

  

  + 增删改查

    + 增

      ```rust
      #![allow(unused)]
      fn main() {
      let mut v = Vec::new();
      
      v.push(5);
      v.push(6);
      v.push(7);
      v.push(8);
      }
      ```

    + 删

      ```rust
      
      #![allow(unused)]
      fn main() {
          {
              let v = vec![1, 2, 3, 4];
      
              // 处理变量 v
      
          } // <- 这里 v 离开作用域并被丢弃
      }
      ```

    + 改

    + 查

      ```rust
      let v = vec![100, 32, 57];
      // 遍历vector中的元素
      for i in &v {
          println!("{}", i);
      }
      ```

      

+ String，字符串

  创建字符串

  ```rust
  pub fn str_test(){
      let mut str = String::new();
  
      // 从字面值创建String
      // 使用to_string 函数创建
      let str1 = "hello world".to_string();
      println!("str = {}, str1 = {}",str, str1);
  
      // 使用 from 函数创建
      let str2 = String::from("hello rust");
      
      println!("str = {}, str1 = {} ,str2 = {}",str, str1,str2);
  }
  ```

  + 附加字符串： push_str 和 push

    ```rust
    // 使用push_str 方法来附加字符串切片到原始字符串
    let mut str3 = String::from("test");
    str3.push_str(" push_str");
    println!("str3 = {}",str3);
    
    // 使用push 将一个字符char 追加到string值中
    let mut str4 = "happy".to_string();
    str4.push('U');
    println!("str4 = {}",str4);
    ```

  + 拼接字符串：+ 运算符 和 format! 宏

    ```rust
    // 此处传入的参数为 &str4 , 是因为+ 相当于调用了add函数，而add函数被调用时，只能将&str和String相加，而不能将两个String值相加。此处的str4 发生了解引用强制转换，使得&String被强转被&str
    let str5 = str3 + &str4;
    println!("str5 = {}",str5);
    
    // 使用 format! 同时拼接多个字符串
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s = format!("{}-{}-{}", s1, s2, s3);
    ```

    

+ hash map

  HashMap<k,v> 类型存储了一个键类型 k 对应一个值类型 v 的映射。通过一个哈希函数来实现映射，将决定如何将键和值放入内存中

  ```rust
  use std::collections::HashMap;
  
  let mut scores = HashMap::new();
  
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  ```

  

## 错误处理

rust将错误区分成两个主要类型：可恢复错误 和 不可恢复错误

可恢复错误通常代表向用户报告错误和重试操作是合理情况，比如未找到文件。不可恢复错误通常是bug的同义词，比如尝试访问超过数组结尾的位置。

rust相应的有 可恢复错误 Result<T,E> , 和不可恢复错误 panic 。panic!宏代表一个程序无法处理的状态，并停止执行而不是使用无效或不正确的值继续处理；Result枚举代表操作可能会在一种可能恢复的情况下失败

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```



失败时panic的简写：unwrap 和 except

如果 `Result` 值是成员 `Ok`，`unwrap` 会返回 `Ok` 中的值。如果 `Result` 是成员 `Err`，`unwrap` 会为我们调用 `panic!`

类似于 `unwrap` 的方法它还允许我们选择 `panic!` 的错误信息：`expect`

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```



传播错误

当编写一个其实现会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理，这被称为 传播错误

```rust
#![allow(unused)]
fn main() {
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
}
```







## 泛型

泛型通过在编译时候进行泛型代码的单态化来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程

+ 在函数定义中使用泛型

  ```rust
  fn largest<T>(list: &[T]) -> T {
      let mut largest = list[0];
  
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
  
      largest
  }
  
  fn main() {
      let number_list = vec![34, 50, 25, 100, 65];
  
      let result = largest(&number_list);
      println!("The largest number is {}", result);
  
      let char_list = vec!['y', 'm', 'a', 'q'];
  
      let result = largest(&char_list);
      println!("The largest char is {}", result);
  }
  ```

+ 在结构体定义中使用泛型

  ```rust
  struct Rectangle<T>{
      width : T,
      length : T,
  }
  
  fn main (){
  	let rec = Rectangle{
          width : 23,
          length : 34,
      }
  };
  ```

+ 枚举定义中的泛型

  ```rust
  enum Result<T,E>{
      Ok(T),
      Err(E),
  }
  ```

+ 方法定义中的泛型

  ```rust
  struct Point<T> {
      x: T,
      y: T,
  }
  
  // 必须在impl声明后面T，这样就可以在Point<T> 上实现的方法中使用它。在impl之后声明泛型T，这样rust就知道Point的尖括号中类型是泛型而不是具体类型
  impl<T> Point<T> {
      fn x(&self) -> &T {
          &self.x
      }
  }
  
  fn main() {
      let p = Point { x: 5, y: 10 };
  
      println!("p.x = {}", p.x());
  }
  ```

  



## trait

trait类似于其他语言中常被称为接口的功能

定义trait

```rust
pub trait Summary{
    fn summarize(&self) -> String; 
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为类型实现trait
impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```



trait作为参数，限制传入参数类型

```rust
//  传入的参数不是具体的类型，只要是任何是实现了Summary trait的类型就行
// 返回值也可以指定必须是实现了某个 实现Summary trait 的类型
pub fn notify(item: impl Summary)  -> impl Summary{
    println!("Breaking news! {}", item.summarize());
    
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```



trait bound

+ trait bound语法

```rust
pub fn notify(item1: impl Summary, item2: impl Summary) {
    
// 等价于
pub fn notify<T: Summary>(item1: T, item2: T) {
```

+ 通过 + 指定多个 trait bound 

```rust
pub fn notify(item: impl Summary + Display) {

// 等价于
pub fn notify<T: Summary + Display>(item: T) {
```

+ 通过where简化 trait bound

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    
// 等价于
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```





## 生命周期

rust中的每一个引用都有其生命周期，也就是引用保持有效的作用域。大部分时候生命周期是隐含的并可以推断的

主要目标就是避免悬垂引用

```rust
&i32 // 引用
&'a i32  // 带显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```

```rust
// 单个引用的生命周期标注本身是没有意义的
// 生命周期'a 的实际生命周期是 x和y两个生命周期中较小的那个
pub fn live_date(){
    let str1 = "hello";
    let str2 = "world";
    longest_test(str1, str2);
}

fn longest_test<'a> (x : &'a str, y : &'a str) -> &'a str {
    if x.len() < y .len(){
        x 
    }else {
        y
    }
}

// 从函数返回的引用时，返回的类型的生命周期参数需要与其中一个参数的生命周期匹配
// 如果返回的引用没有指向任何参数，那么它就会指向引用函数内创建的值。这就是悬垂引用：该值在函数结束时候就走出了作用域 -->  此时如果想要可用，请直接返回值而不是引用 	
```

生命周期在 函数/方法的参数中：输入生命周期； 函数/ 方法的返回值中：输出生命周期

生命周期省略的三个规则：

1. 每个引用类型都有自己的生命周期
2. 如果只有1个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期参数
3. 如果有多个输入生命周期参数，但其中一个是&self 或者&mut self（方法），那么self的生命周期会被赋给所有的输出生命周期参数



借用检查器：比较作用域来确保所有的借用都是有效的





## 测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


//输出结果
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

注意：fn行之前的`#[test]` ：这个属性表名这是一个测试函数



```rust
//运行单个指定测试，通过名字指定
$ cargo test one_hundred

//过滤运行多个测试。这运行了所有名字中带有 add 的测试，也过滤掉了其他不符合条件的测试
$ cargo test add

//忽略某些测试。使用 ignore 属性来标记耗时的测试并排除他们
#[test]
#[ignore]
fn expensive_test() {
    // 需要运行一个小时的代码
}
```











## IO

+ 获取命令行参数 -- `std::env::args`

使用rust标准库提供的函数，`std::env::args`，这个函数返回一个传递给命令行参数的迭代器。

```rust
use std::{env, string};

fn main() {
    println!("Hello, world!");

    // 获取命令行参数 ，使用rust标准库提供的函数，也就是 std::env::args
    let args: Vec<String> = env::args().collect();

    println!("{:?}",args); // ["target\\debug\\minigrep.exe", "mingrep"] 命令行参数收集到一个Vector中
}

// 注意：Vector的第一个值是"target\\debug\\minigrep.exe" ，它是我们二进制文件的名称
```

+ 读取文件内容 -- `std::fs` 

  使用fs::read_to_string 接收filename，打开文件，返回包含其内容的Result< String >

```rust
use std::{env, string,fs};

fn main() {
    println!("Hello, world!");
    // 获取命令行参数 ，使用rust标准库提供的函数，也就是 std::env::args
    let args: Vec<String> = env::args().collect();

    println!("{:?}",args); // ["target\\debug\\minigrep.exe", "mingrep"] 命令行参数收集到一个Vector中

    let filename = &args[1];
    println!("filename = {} ",filename);

    let contents = fs::read_to_string(filename).expect("something went wrong reading the");
    println!("contents = {}",contents);
}
```











## 智能指针



+ Box< T> 

  Box允许将一个值放在堆上而不是栈上，留在栈上的则是指向堆数据的指针

  + Deref trait

    将智能指针当作常规引用处理

  + Drop trait

    运行清理代码

+ Rc< T>

  引用计数智能指针

+ RefCell< T>

  内部可变性







## 并发





## 面向对象







## 高级特性







## package \ crate \module

+ crate的类型

  将相关功能组合到一个作用域内，便于在项目间进行共享，防止冲突

  + binary
  + library

+ package

  + 包含一个Cargo.toml，描述如何构建这些Crates
  + 只能包含0~1个 library crate
  + 可以包含任何数量的binrary crate
  + 但必须至少包含一个crate (library或者binary都可以)

+ 定义module来控制作用域和私有性

  + 在一个crate内，将代码进行分组

  + 如何定义module

    mod 关键字 {}

    可以嵌套，也可以包含其他项(struct，enum，常量，trait，函数等)的定义

+ 路径

  + 相对路径
  + 绝对路径

+ use关键字

  绝对路径和相对路径可以帮助我们找到指定的函数，但用起来也非常的麻烦，每次都要写一大长串路径。Rust为我们提供了use关键字。在很多语言中都有import关键字，这里的use就有些类似于import。

  use 可以使用绝对路径或者相对路径

  ```rust
  mod front_of_house {
      pub mod hosting {
         	pub fn add_to_waitlist(){} 
      }
  }
  
  use crate::front_of_house::hosting; // 绝对路径
  
  use front_of_house::hosting; // 相对路径
  
  pub fn eat_at_restaurant(){
      // 一般引用到所需函数的上级目录(指定到父级)，避免与本地可能有同名函数，无法区别
      // struct、enum、其他: 指定完整路径(指定到本身)
      hosting::add_to_waitlist();
  }
  ```

  以上的代码可以拆分到不同文件

+ as关键字

  为引入的路径指定本地的别名

  ```rust
  use std::io::Result as IOResult;
  ```

  