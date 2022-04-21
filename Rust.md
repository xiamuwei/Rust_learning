# Rust



rust是一门系统级编程语言，被设计为保证内存和线程安全

rust编程语言的目的是创建一个高度安全和并发的软件系统，强调安全性、并发、内存控制。不运行C和C++中会引起系统崩溃、内存泄漏和不安全代码根源的 空指针和悬垂指针

rust使用实现 (implemenet)、特征 (trait) 和结构化类型 (structured type) 而不是类 (class)

rust 做到了内存安全而无需有些编程语言中实现自动垃圾收集器的开销，这是通过所有权/ 借用机制、生命周期、类型系统来达到的



应用：

1. 传统命令行程序：Rust 编译器可以直接生成目标可执行程序，不需要任何解释程序；
2. web应用：Rust 可以被编译成 WebAssembly，WebAssembly 是一种 JavaScript 的高效替代品；
3. 网络服务器：Rust 用极低的资源消耗做到安全高效，且具备很强的大规模并发处理能力，十分适合开发普通或极端的服务器程序；
4. 嵌入式设备：Rust 同时具有JavaScript 一般的高效开发语法和 C 语言的执行效率，支持底层平台开发。



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




## vscode

rust插件 ：

+ rust-analyzer 与rust插件冲突，建议rust-analyzer

+ crates：帮助编写Cargo.toml 依赖
+ hint：首选项中打开有关hint的选项
+ REST client : 
+ Toml Language Support



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

  + src/lib.rs

    一个程序最多有一个libary crate

  + Cargo.toml
  
    ![cargotoml内容](E:\learning\Michael\截图\cargotoml内容.jpg)
  
    + package表明下面的语句用来配置一个package。name、version、edition分别表明项目的名称、版本、使用的rust大版本号
    + dependencies是罗列项目依赖。rust把代码所需要的库叫做依赖
    + profile ：设置各个阶段的相关信息，比如优化级别：opt-level 优化级别越高，所花费时间越长
  
  + .gitignore 
  
    初始化了一个Git仓库



+ 构建并运行Cargo项目
  + cargo run 如果之前编译过，并且源码没有改变，那么就会直接运行二进制文件
  + cargo check 检查代码，确保能够通过编译，但是不产生任何可执行文件。运行比cargo build快很多
  + cargo build [--release]   // 后面的--release参数表示编译正式发布的版本，不然就是开发版本
  + cargo update 更新依赖项

生成的目标文件在target/debug目录下

如果是--release 的话，则在target/release 目录下







## 注释

+ 普通注释 ，其内容将被编译器忽略掉

  ```rust
  // 单行注释，注释内容直到行尾
  
  /*
  	块注释，注释内容一直到结束分割符
  */
  ```

  

+ ==文档注释 ，其内容将被解析成HTML帮助文档==

  ```rust
  /// 为接下来的项目生成帮助文档，文档注释位于需要文档的项之前
  /// Adds one to the number given.
  ///
  /// # Examples
  ///
  /// ```
  /// let arg = 5;
  /// let answer = my_crate::add_one(arg);
  ///
  /// assert_eq!(6, answer);
  /// ```
  pub fn add_one(x: i32) -> i32 {
      x + 1
  }
  注可以通过 cargo doc 来生成这个文档注释的HTML文档，放在target/doc 目录下
  而cargo doc --open 命令会构建当前crate文档并在浏览器中打开
  
  //! 为注释所属于的项(译注：如crate、模块或函数)生成帮助文档
  
  ```

  



## 变量

rust是==强类型语言==，但具有自动判断变量类型的能力

> 注：
>
> 强类型语言：一旦一个变量被指定了某个数据类型，如果不经过强制转换，那么不可以将其他类型赋给这个变量
>
> 弱类型语言：一个变量可以赋不同数据类型的值
>
> 静态类型语言：编译期进行数据类型检查
>
> 动态类型语言：运行期才做类型检查

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


// 常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值
const A:u32 = 2;
```





常量

```rust
const PI: f64 = 3.14; // 常量

// 全局变量
static LANGUAGE: &'static str = "Rust";  // 静态变量
```

> + 不可变变量和常量的区别
>
> 1. 常量不允许使用mut。常量不仅仅默认不可变，而且自始至终不可变
> 2. 常量使用`const`关键字而不是 let 关键字来声明，而且值的类型必须注明 (const a: i32 = 123;)
> 3. 常量可以在任意作用域内声明，包括全局作用域
> 4. 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值
>
> + 常量与静态变量区别
>
> 1. 静态变量中的值有一个固定的内存地址，使用这个值总会访问相同的地址
> 2. 常量则允许在任何被用到的时候复制其数据
> 3. 静态变量可以是可变的





变量遮蔽 (shadow)

==隐藏是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化==

```rust
fn main (){
    let a = 12;
    let a = "hello";
}
```





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
  
      // 整型 默认i32
      let integer:i32 = 10;
      // let integer = 10;  自动类型推导
      println!("{}",integer);
  
      // 浮点型 默认f64
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
  
  // 没有任何值的元组() 是一种特殊类型，只有一个值，也写成()。该类型被称为单元类型(unit type),而该值被称为单元值。**如果表达式不返回任何其他值，则会隐式返回单元值**
  let unit_type = ();
  
  println!("x = {}, y = {}, z = {}",x,y,z);
  
  // 定义数组
  let arr = [1,2,3];
  // 访问数组元素
  println!("arr 第一个元素 = {}", arr[0])
  
  let arr1:[i32; 5] = [1, 2, 3, 4, 5];
  // 创建一个名为arr2 的数组，size为5，默认值是3
  let arr2:[i32;5] = [3;5] ;
  // 等价于 let arr2 = [3, 3, 3, 3, 3];
  ```
  



类型转换

+ 对于原始数据类型，使用as关键字

  as 运算符类似于C中的强制类型转换，区别在于，**它只能用于原始类型，并且它是安全的**

  在 Rust 中，不同的数值类型是不能进行隐式转换的，可以使用as 进行转换

  ```rust
  let b: i64 = 1i32;
  // 会提示错误如下
  /*
  error[E0308]: mismatched types
   --> src\main.rs:2:18
      |
  2   |     let b: i64 = 1i32;
      |                  ^^^^ expected i64, found i32
  help: change the type of the numeric literal from `i32` to `i64`
  */
  
  使用as进行类型转换
  let b: i64 = 1i32 as i64;
  
  let int_value:i32 = 100;
  let num = char_value as i32;
  println!("{}",num);
  ```

+ 对于复杂类型，如struct、enum等，使用From 和 Into trait实现

  From 和 Into trait结构

  ```rust
  pub trait From<T> {
      fn from(T) -> Self;
  }
  pub trait Into<T> {
      fn into(self) -> T;
  }
  ```

  一般来说，我们应该尽量优先选择实现From< T> 而不是 Into< T>，因为当你实现了 From< T>，这意味着你同时隐式实现了 Into< T> , 但是需要注意的是，反之不会

  **默认规则: 如果类型 U 实现了 From< T>,则T 类型实例调用into方法就可以转换为类型U**
  
  ```rust
  #[derive(Debug)]
  struct Number{
      value :i32,
  }
  impl From<i32> for Number{
      fn from(item:i32)->Self{
          Number{value:item}
      }
  }
  
  fn type_exchange(){
      let num1 = 100;
      let n = Number::from(num1);
  
      // 默认为类型Number实现 From<i32>后，i32可以使用into方法，获取Number类型
      let num2 = 200;
      let n2:Number = n.into();
  }
  ```
  
  



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
    
    
    fn if_control(){
         let n = 5;
         if n < 0{
             print!("{} is negative",n);
         }else if n > 0 {
            print!("{} is positive",n);
         }else {
             print!("{} is zero", n);
         }
    
         // if - else 条件选择是一个表达式，并且所有分支都必须返回相同的类型
         let big_n = 
         if n < 10 && n > -10 {
             print!(", and is a small number, increase ten-fold");
             10*n
         }else {
            println!(", and is a big number, half the number");
            n/2
         };
    
         print!("{} -> {} ", n ,big_n);
    }
    ```

  + match

    match关键字,与switch用法类似.第一个匹配分支会被比对,并且其他所有可能的值必须被穷举覆盖

    ```rust
    fn match_control(){
        let number = 13;
        println!("Tell me about {}",number);
    
        match number { // 必须穷举所有的可能性 
            1 => println!("One!"),
            // 匹配第一个合适的区间
            2 | 3 | 13 | 5 | 7 => println!("This is a prime"),
            13..=19 =>println!("A teen"),
            _ => println!("Ain't special"),
        }
    }
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
    
    
    fn loop_control(){
        // 无限循环
        let mut count = 0u32;
        println!("Let s count until infinity!");
        loop {
            // rust 语言不支持 ++ --
            count += 1;
            if count == 3{
                println!("now is third");
                // 跳出当前循环，后续代码不执行，继续执行下个循环代码
                continue;
            }
            println!("{}",count);
    
            if count == 5 {
                // 结束循环
                break;
            }
    
        }
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
    fn for_control (){
        // rev函数用于反转区间
        for i in (1..4).rev(){
            println!("i = {}",i);
        }
        
        // for循环,包括左边的值，不包括右边的值
        for i in 1..4{
            println!("{}",i);
        }
        /*
        结果
        1
        2
        3
        */
        
        // ..= 表示两端都包含在内的范围
        for i in 1..=4{
            println!("{}",i);
        }
        /*
        结果
        1
        2
        3
        4
        */
    }
    ```

    

  + break 和 continue 标签

    + break ：程序何时停止循环
    + continue ：告诉程序跳过当前循环剩余代码，并转到下一个迭代



## 函数

函数体由 一系列的语句 和 可选的结尾表达式构成

语句和表达式

语句：执行一些操作但不会返回值

表达式：计算并产生一个值

```rust
// rust不关心函数定义在哪里，只需要定义就行。main函数中的代码会按顺序执行
fn main (){
    test(1 , true);  // 此处1 , true 是实参
}

// 在函数签名中，必须声明每个参数的类型
// 对于函数返回值，我们不需要对返回值命名，但要在-> 箭头后面声明它的类型
// 而"不"返回值的函数,实际上会返回一个单元类型 '()'
fn test(a :i32 ,b :bool) -> i32{  // 此处的 a ,b  是形参
    // 函数体
    let mut a = 6; // 语句
    
    // 函数隐式返回最后的表达式
    a + 1  // 表达式
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
```



> 格式化输出
>
> 打印操作由std::fmt 里面所定义的一系列宏来处理，包括：
>
> - `format!`：将格式化文本写到[`字符串`](https://rustwiki.org/zh-CN/rust-by-example/std/str.html)（String）。（译注：`字符串`是返回值不是参数。）
> - `print!`：与 `format!` 类似，但将文本输出到控制台（io::stdout）。
> - `println!`: 与 `print!` 类似，但输出结果追加一个换行符。
> - `eprint!`：与 `print!` 类似，但将文本输出到标准错误（io::stderr）。
> - `eprintln!`：与 `eprint!` 类似，但输出结果追加一个换行符。



## 函数式编程

函数式编程风格通常包含将函数作为参数值或其他函数的返回值，将函数赋值给变量以供之后执行等

闭包通过匿名函数实现

+ 闭包

  闭包是函数指针和上下文context的组合

  Fn是带有不可变上下文的闭包

  FnMut是带有可变上下文的闭包

  FnOnce拥有其上下文的闭包，所有的闭包都实现了FnOnce

  ```rust
  /*
  fn  add_one_v1   (x: u32) -> u32 { x + 1 }
  let add_one_v2 = |x: u32| -> u32 { x + 1 };
  let add_one_v3 = |x|             { x + 1 };
  let add_one_v4 = |x|               x + 1  ;
  */
  
  
  // 注意：闭包会为每个参数和返回值推断一个具体类型，故不需要强制标注参数和返回值的类型
  // 但是如果对同一个闭包使用不同类型则会得到类型错误，因为第一次调用时所使用的类型将被锁定进比保重
  /*
  	let example_closure = |x| x;
  
  	let s = example_closure(String::from("hello"));
  	let n = example_closure(5); // 此处会报错，因为x类型被确定为String
  */
  
  let expensive_closure = |num| {
      println!("calculating slowly ...");
      thread::sleep(Duration::from_secs(2));
      num
  };
  
  // 调用闭包
  fn generate_workout(intensity: u32, random_number: u32) {
      let expensive_closure = |num| {
          println!("calculating slowly...");
          thread::sleep(Duration::from_secs(2));
          num
      };
  
      if intensity < 25 {
          println!(
              "Today, do {} pushups!",
              // num 为形参，intensity所代表的值为实参
              expensive_closure(intensity)
          );
          println!(
              "Next, do {} situps!",
              expensive_closure(intensity)
          );
      } else {
          if random_number == 3 {
              println!("Take a break today! Remember to stay hydrated!");
          } else {
              println!(
                  "Today, run for {} minutes!",
                  expensive_closure(intensity)
              );
          }
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
      let mut x = 1;
      let mut t = ||{x+=1;println!("{}",x);};
      t();
      t();
  } 
  // 结果
  2
  3
  ```

  ```rust
  // move 关键字的使用
  use std::thread;
  
  fn main() {
      let s = "hello";
     
      let handle = thread::spawn(|| {
          println!("{}", s);
      });
  
      handle.join().unwrap();
  }
  
  // 上述代码错误，在子线程中尝试使用当前函数的资源，一定是错误的！因为所有权机制禁止这种危险情况的产生
  fn main() {
      let s = "hello";
     
      let handle = thread::spawn(move || {
          println!("{}", s);
      });
  
      handle.join().unwrap();
  }
  ```

  

+ 迭代器

  迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑

  ```rust
  // 迭代器都是实现了 Iterator trait
  pub trait Iterator{
      type Item;
      // next方法是Iterator 实现者被要求定义的唯一方法。next一次返回迭代器中的一个项，封装在Some中，当迭代器结束时，返回None
      // Iterator trait有一系列不同的由标准库提供默认实现的方法，一些方法在其定义中调用了next方法，这也就是为什么在实现Iterator trait时要求实现next方法的原因
      fn next(&mut self) -> Option<Self::Item>;
  }
  ```

  

​	



## ==所有权==

所有权是rust最为与众不同的特性，它让rust无需垃圾回收器（garbage collector）即可保证内存安全。==需要注意的是所有权是针对堆上的数据来说的==

通过所有权系统管理内存，编译器在**编译时**会根据一系列的规则进行检查。在运行时，所有权系统的任何功能都不会减慢程序



栈与堆

栈：大小固定，后进先出

堆：大小不固定，缺乏组织

![rust 所有权](D:\备份\截图\rust 所有权.png)

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





引用 ：使用一个变量的值但不获得所有权. 多数情况下,我们更希望能够访问数据,同时不获取其所有权

>  &符号就是 **引用**，它们允许你使用值但不获取其所有权
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
    // 创建一个可变引用的前提是 s 变量本身也是可变的，可修改的
    let mut s = String::from("hello rust");
    change_string(&mut s);
}

fn change_string(str:&mut String) {
    str.push_str(",hi");
    println!("str = {}",str);
}
```

为了避免数据竞争，规定引用，在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用

```rust
let a = &str6;
let b = &str6;
let c = &mut str6;
println!("{}, {},{}", a, b,c);

let d = &mut str7;
let e = &mut str7;
println!("{}, {}", d, e);
```





借用：创建一个引用的行为被称为借用





## 数组

```rust
fn complex_type(){
    // 数组
    let arr1:[i32;5] = [1,2,3,4,5];
    let arr2:[i32;5] = [3;5] ;

    // 遍历数组
    for i in arr2 {
        println!("i = {}",i)
    }

}
// 结果
i = 3
i = 3
i = 3
i = 3
i = 3
```



## 元组

```rust
fn complex_type(){  
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
```



## 切片

slice 引用集合中一段连续的元素序列，而不是引用整个集合

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];  // .. 是rust的循环语法，&代表的是引用符号
    let world = &s[6..11];
}
```

注意：字符串字面量就是slice

```rust
let s = "Hello, world!";
```



## 常见集合

集合不同于内建的数组和元组，这些集合指向的数据是存储在堆上的，这意味着数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或者缩小

+ vector ：只能存储相同类型的值. 一个挨着一个地存储一系列数量可变的值

  创建vector

  ```rust
  // 使用<i32> 指明此时vector内存的类型, Vector是泛型实现的
  let vec:Vec<i32> = Vec::new();
  
  // 一旦插入值rust就可以推断出想要存放的类型
  let v = vec![1,2,3];
  ```

  + 增删改查

    + 增

      ```rust
      // 使用Vec::new函数创建Vector, Vector是泛型实现的
      let mut v :Vec<i32> = Vec::new();
      // 使用vec! 宏来创建vector
      let mut v2 = vec![1,2,3,4,5];
      // 增加
      v.push(2);
      v.push(3);
      v.push(4);
      
      // 将两个Vec拼接起来
      v.append(&mut v2);
      
      ```

    + 删

      ```rust
      v.pop(); // 将最后一个数丢掉
      v.clear(); // 清空vector
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

  UTF-8 编码

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
  fn hashMap_test(){
      // 创建一个hashmap
      let mut map: HashMap<i32, i32> = HashMap::new();
      // 增加
      map.insert(1, 100);
      map.insert(2, 101);
      map.insert(3, 102);
  
      // 删
  
      // 改变
          // 覆盖
          map.insert(1, 200);
          // 只有键没有对应值时插入
          map.entry(5).or_insert(500);
          // 根据旧键更新值
          let x = map.entry(1).or_insert(100);
          *x += 1;
      // 遍历
      for (i,k) in map{
          println!("map[{}] = {}",i,k);
      }
  }
  ```

  



## 结构体

结构体 的每一部分都可以是不同类型。struct 需要命名各个部分数据以便能清除的表明其值的意义。由于这些名字，结构体比元组更灵活：不需要依赖顺序来指定或访问实例中的值

**方法**（method）与函数类似：它们使用 `fn` 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文），==并且它们第一个参数总是 `self`，它代表调用该方法的结构体实例==

+ 自定义结构体

  ```rust
  #[derive(Debug)]
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  
  
  fn main (){
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
      
      // Debug 是一个trait，允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调式代码时能看到它的值。我们在定义结构体时显示加上 #[derive(Debug)]注解。
      // 在{}中加入:? 指示符告诉println! 我们想要使用Debug的输出格式。Debug是一个trait，它允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调试代码时能看到它的值
      println!("user1 = {:?}",user1);
      
  }
  ```

+ 元组结构体

  ```rust
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  
  fn main() {
      let black = Color(0, 0, 0);
      let origin = Point(0, 0, 0);
  }
  ```

+ 类单元结构

  ```rust
  // 一个没有任何字段的结构体被称为类单元结构体，因为它们类似于(), 即unit类型
  struct AlwaysEqual;
  
  fn main() {
      let subject = AlwaysEqual;
  }
  ```
  
  

方法

方法与函数类似：使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会调用的代码。

但方法是在struct (或enum，trait对象)的上下文中定义；但它们的第一个参数总是self，它代表调用该方法的结构体实例。

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
    
    // 使用结构体:: 语法调用这个关联函数
    let sq = Rectangle::square(3);
}
```









## 枚举和模式匹配

枚举 (enum) 允许你通过列举可能的成员来定义一个类型

枚举可以用来代替结构体: 每个成员可以处理不同类型和数量的数据

```rust
enum Message {
    Quit,  // 没有关联任何数据,类单元结构体
    Move{x :i32, y :i32},  // 一个结构体
    Write(String),  // 一个字符串
    ChangeColor(i32, i32, i32), // 三个i32
}

// 总体来说还是一个类型,可以直接实现这个Message类型的方法来处理这些不同的类型
impl Message {
    fn call(&self){}
}

fn main(){
    let q = Message::Quit;
    let m = Message::Move{x: 12, y: 24};
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0,255,255);
    
    m.call();
}
```





Option

Option 是标准库中定义的另一个枚举，描述某个值可能存在(某种类型)或者不存在的情况。位于prelude中(预导入模块)

空值 (Null) 是一个值，它代表没有值。在有空值的语言中，变量总是这两种状态之一：空值和非空值

但是==rust没有空值，而是使用一个枚举Option 来实现==

```rust
enum Option<T> {
    Some(T),
    None,
}

let some_number = Some(5);
let some_string = Some("a string");
// 如果使用None 而不是Some，需要告诉Rust Option<T> 是什么类型，因为编译器只通过None值无法推断出Some成员保存的值的类型
let absent_number: Option<i32> = None;
```



模式匹配

match允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码

**rust模式匹配是穷举式的**，这意味着你必须覆盖所有可能的情况

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
    

    let letter:Option<i32> = None;
    let emoction:Option<i32> = None;
    if let Some(i) = letter {
        println!("Matched {:?}!",i);
    }else {
        println!("Dont match a number ,lets go with a letter");
    };
}
```

while let 

```rust

fn while_let_control(){

    let mut optional = Some(0);
    loop {
        match optional{
            Some(i) => {
                if i > 9{
                    println!("Greater than 9, quit!");
                    optional = None;
                }else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            // 当解析失败时退出
            _ => {break;},
        }
    }


	// 可以使用while let简化上述代码
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }

}
```





模式匹配的其他用法

```rust
// 使用 _ 来匹配剩下所有情况，不过它从不绑定任何变量

// 通过 | 来实现多个模式匹配
match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}

// 通过 ..= 来实现匹配模式范围。(范围只允许数字或者char值，因为编译器会在编译时检查范围不为空)
match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}


// 通过 @ 允许我们在创建一个存储值的变量的同时，测试其值是否匹配模式
enum Message {
    Hello {id :i32},
}

let msg = Message::Hello{ id :5 };

match msg {
    // 判断id是否在 3~7 这个范围之间的同时将值赋给id_variable
    Message::Hello {id :id_variable @ 3..=7 } => {
        println!("Found an id in range :{}", id_variable);
    },
    Message::Hello {id : 10..=12 } => {
        println!("Found an id in another range ");
    }
    Message::Hello {id } => {
         println!("Found some other id: {}", id);
    },
}
fn valuetToVariable(){
    let num = 2;
    match num {
        n@1..=8 => println!("num = {}",num),
        n@9 => println!("num = 9"),
        _ => println!("no idea"),
    }
}
```





匹配守卫

```rust
//一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```











## 泛型

泛型通过在编译时候进行泛型代码的单态化来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

这意味着在使用泛型时**没有运行时开销**；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。这个单态化过程正是 Rust 泛型在运行时**极其高效**的原因。

+ 在函数定义中使用泛型

  ```rust
  // 如同函数中使用参数，需要在函数签名中声明参数名一样。当在函数签名中使用一个类型函数时，必须在使用它之前就声明它。类型参数声明位于函数名称与参数列表中间的尖括号 <> 中
  fn largest<T>(list: &[T]) -> T {
      let mut largest = list[0];
  
      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }
  
      largest // 表达式
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
  
  // 意味着Point<f32>类型会有一个方法 `distance_from_origin`，而其他 `T` 不是 `f32` 类型的 `Point<T>` 实例则没有定义此方法。这个方法计算点实例与坐标 (0.0, 0.0) 之间的距离，并使用了只能用于浮点型的数学运算符
  impl Point<f32> {
      fn distance_from_origin(&self) -> f32 {
          (self.x.powi(2) + self.y.powi(2)).sqrt()
      }
  }
  
  fn main() {
      let p = Point { x: 5, y: 10 };
  
      println!("p.x = {}", p.x());
  }
  ```
  
  



## ==trait==

trait类似于其他语言中常被称为接口的功能

定义trait

```rust
pub trait Summary{
    fn summarize(&self) -> String; 
    // Summary trait的定义，带有一个summarize_default  方法的默认实现，不需要在每个类型的每个实现中都定义
    fn summarize_default (&self) -> String {  
        String::from("(Read more...)")
    }
    
    
    // 默认实现允许调用相同trait中的其他方法，哪怕这些方法没有默认实现。	
    fn summarize_author(&self) -> String;
	// 使用时，只需要在实现trait时定义summarize_author即可
    fn summarize_test(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    
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
pub fn notify(item1: impl Summary, item2: impl Summary) {}
    
// 等价于
pub fn notify<T: Summary>(item1: T, item2: T) {}
```

+ 通过 + 指定多个 trait bound 

```rust
pub fn notify(item: impl Summary + Display) {}

// 等价于
pub fn notify<T: Summary + Display>(item: T) {}
```

+ 通过where简化 trait bound

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
    
// 等价于
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```



```rust
// rust不允许类型间继承，但是支持组合，然而trait之间以好像继承的形式实则是用组合的本质来组织trait, 子 trait相当于超集合， 而父trait相当于子集合，也就是说如果你要impl 一个子 trait, 那么也必须同时Impl其父trait， 因为他们是组合不是继承哟，切记！敏捷软件开发认为： 组合优于继承，这也是工程界的共识经验

trait Person {
    fn name(&self) -> String;
}
 
//Person现在是Student的父集， 而Student称为Person的子集。
//所以rust要求impl子集的同时也必须impl父集。
trait Student: Person {
    fn university(&self) -> String;
}
 
trait Programmer {
    fn fav_language(&self) -> String;
}
 
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
 
 
struct Foo {};
impl CompSciStudent for Foo {
     fn git_username(&self) -> String {
         String::from("CompSciStudent ")
     }
}
 
//注意在impl CompSciStudent trait的时候，必须同时impl trait Programmer and Student and Person.
impl Programmer for Foo {
      fn fav_language(&self) -> String {
           String::from("Programmer ")
      }
}
impl Student for Foo {
      fn university(&self) -> String {
          String::from("Student")
      }
}
 
impl Person for Foo {
     fn name(&self) -> String {
         String::from("Person ") 
     }
}
 
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}
```







孤儿原则：

不能为外部类型实现外部trait。这条规则保证了其他人编写的代码不会破坏你的代码，反之亦然，没有这条规则的话，两个crate可以分别对相同类型实现相同的trait，而rust将无从得知应该使用哪一个实现



## ==生命周期==

也是一种泛型

因为引用是只是获取值但不获取所有权, 无法确保所引用的值没有被销毁,为了防止引用一个已经被销毁的对象,引入了生命周期概念

rust中的每一个引用都有其生命周期，也就是**引用保持有效的作用域**,  分析引用对象的生命周期来防止引用一个已经不可用的对象。大部分时候生命周期是隐含的并可以推断的

==主要目标就是避免悬垂引用==

```rust
/*
以 单引号 ' 开头,后接一个字母
位置在引用符号 & 的后面,使用空格将标注和类型分开
详细例如下:
*/

&i32 // 引用
&'a i32  // 带显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```

```rust
// 单个引用的生命周期标注本身是没有意义的
// 生命周期'a 的实际生命周期是 x和y两个生命周期中较小的那个
// 生命周期注解并不改变任何引用的生命周期的长短。生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期
pub fn live_date(){
    let str1 = "hello";
    let str2 = "world";
    // 当函数在外边被调用时,具体的引用(即实参)被传递给longest_test函数后,泛型生命周期'a 所替代的具体生命周期是x和y之中生命周期较小的哪一个.那就意味着我们返回的引用值,在x和y中较短的那个生命周期结束之前都是保持有效的
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
2. 如果只有一个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期参数
3. 如果有多个输入生命周期参数，但其中一个是&self 或者&mut self（方法），那么self的生命周期会被赋给所有的输出生命周期参数

以上三条规则在没有显示标注生命周期的情况下,来确定引用的生命周期.其中第一条规则用于输入生命周期, 后面两条规则用于输出生命周期. 如果编译器使用了三条规则后仍然无法确定生命周期的引用,就会编译失败,此时需要显示生命周期



借用检查器：比较作用域来确保所有的借用都是有效的

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
// 将 r 的生命周期标记为 'a
// 将 x 的生命周期标记为 'b
// 可以看到 'b 块比 'a 小
// 在编译时, 借用检查器会比较这两个生命周期的大小
// 发现 r 拥有生命周期 'a 但它引用了一个拥有生命周期 'b 的对象
// 此时程序被拒绝编译, 因为生命周期 'b 比生命周期 'a 要小
// 也就是被引用的对象比它的引用者存在的时间更短。
```









静态生命周期

```rust
'static
/*
是一个特殊的生命周期,它代表整个程序的持续时间
例如,所有的字符串字面值都拥有'static 生命周期.因为字符串字面值是直接存储在二进制程序当中,所以它总是可用的
而在我们给引用指定static的生命周期时，就要思考清楚是否需要在程序整个生命周期内都要存活，否则容易引起内存安全问题
*/
fn main() {
    println!("Hello, world!");
    let str = longest("str12");
    println!("str = {}",str);
}

fn longest(x:&str) -> &str{
    let y = " str"; // 所有的字符串字面值都拥有'static 生命周期,故此处顺利执行
    if x.len() < y.len(){
        x 
    }else {
        y
    }
}
```











## ==智能指针==

智能指针(smart pointer)：是一类数据结构。为了可以方便的控制对象的生命周期，安全的使用动态内存，引入智能指针的概念。智能指针的行为类似常规指针，但拥有额外的元数据和功能，重要的区别是它负责自动释放所指向的对象。智能指针和普通引用的区别之一就是所有权的不同。智能指针拥有资源的所有权，而普通引用只是对所有权的借用.

智能指针通常使用结构体实现。==智能指针区别于常规结构体的显著体征在于实现了Deref 和Drop trait==

**Deref trait**提供了解引用的能力；

**Drop trait** 提供了自动析构的能力，允许我们自定义当智能指针离开作用域时运行的代码



+ Box< T> 

  Box允许将一个值放在堆上而不是栈上，留在栈上的则是指向堆数据的指针

  

+ Rc< T>

  注意：Rc< T> 只能用于单线程场景

  引用计数智能指针 ：意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理

+ RefCell< T>

  注意：RefCell< T>只能用于单线程场景
  
  ==内部可变性==
  
  

+ Deref trait

  将智能指针当作常规引用处理

+ Drop trait

  运行清理代码

  > rust不允许我们主动调用Drop trait的drop方法(rust不允许我们显示调用drop，因为rust仍然会在main的结尾对值自动调用drop，会导致一个double free错误，因为rust会尝试清理两次相同的值)；
  >
  > 当我们希望在作用域结束之前就强制释放变量的话，我们可以使用标准库提供的 std::mem::drop





原子引用计数Arc类型是一种智能指针，它能够让你以线程安全的方式在线程间共享不可变数据。

原子引用计数确保在对类型的所有引用都结束之前，它不会被丢弃

==需要注意的是，`Arc`只能包含不可变数据==。这是因为如果两个线程试图在同一时间修改被包含的值，`Arc`无法保证避免**数据竞争**。如果你希望修改数据，你应该在`Arc`类型内部封装一个互斥锁保护（Mutex guard）

[Arc使用](https://zhuanlan.zhihu.com/p/300971430)



## 错误处理

rust将错误区分成两个主要类型：可恢复错误 和 不可恢复错误

可恢复错误通常代表向用户报告错误和重试操作是合理情况，比如未找到文件。不可恢复错误通常是bug的同义词，比如尝试访问超过数组结尾的位置。

rust相应的有 **可恢复错误 Result<T,E> **, 和**不可恢复错误 panic **。panic!宏代表一个程序无法处理的状态，并停止执行而不是使用无效或不正确的值继续处理；Result枚举代表操作可能会在一种可能恢复的情况下失败

```rust
// T 和 E 是泛型类型参数。T 代表成功时返回的Ok成员中的数据的类型，而E 代表失败时返回的Err成员中的错误的类型
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



为了应对panic，展开或者中止调用栈

默认情况查下，当panic发生：

- 程序展开调用栈 (工作量大)
  - rust沿着调用栈往回走
  - 清理每个遇到的函数中的数据
- 或立即中止调用栈
  - 不进行清理，直接停止程序
  - 内存需要OS进行清理

Cargo.toml 中设置profile.release 是否展开或者立即中止

![image-20220310181518258](C:\Users\Administrator\AppData\Roaming\Typora\typora-user-images\image-20220310181518258.png)



失败时panic的简写：unwrap 和 except

如果 `Result` 值是成员 `Ok`，`unwrap` 会返回 `Ok` 中的值。如果 `Result` 是成员 `Err`，`unwrap` 会为我们调用 `panic!`

类似于 `unwrap` 的方法它还允许我们选择 `panic!` 的错误信息：`expect`。except允许在调用panic!  时自定义错误信息，而不是像unwrap 使用默认的panic! 信息

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
        Err(e) => return Err(e), // 如果打开文件成功，返回Ok值；如果失败，则返回err
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
}


// 传播错误的简写 ：? 运算符
// Result 值之后的? 被定义为与上面所示的match表达式一样的功能。如果Result的值是Ok，这个表达式将会返回Ok中的值而程序继续执行。如果值是Err,Err中的值将作为整个函数的返回值，就像使用了return关键字，这样错误就会被传播给了调用者
// 只有在返回Result 或者其他实现了FromResidual的类型的函数中使用? 运算符
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```



## 测试

当运行多个测试时，Rust默认使用线程来并行运行。这意味着测试会更快的运行完毕，所以你可以更快的得到代码能否工作的反馈。因为测试是在同时运行的，你应该确保测试不能相互依赖，或依赖任何共享的状态，包括依赖共享的环境，比如当前工作目录或者环境变量。

如果你不希望测试并行运行，或者想要更加精确的控制线程的数量，可以传递 `--test-threads` 参数和希望使用线程的数量给测试二进制文件。例如

```rust
$ cargo test -- --test-threads=1
// 这里将测试线程设置为 `1`，告诉程序不要使用任何并行机制。这也会比并行运行花费更多时间，不过在有共享的状态时，测试就不会潜在的相互干扰了。
```





```rust
#[cfg(test)]
mod tests {
    // #[test]属性标明哪些函数是测试
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
// `test result: ok.` 意味着所有测试都通过了。`1 passed; 0 failed` 表示通过或失败的测试数量。因为之前我们并没有将任何测试标记为忽略，所以摘要中会显示 `0 ignored`。我们也没有过滤需要运行的测试，所以摘要中会显示`0 filtered out`。`0 measured` 统计是针对性能测试的
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





+ 单元测试

  概念：单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口

+ 集成测试

  概念：集成测试对于你的库来说则完全是外部的。它们与其他外部代码一样，通过相同的方式使用你的代码，只测试公有接口 而且每个测试都有可能会测试多个模块





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


// 获取命令行参数
let args = std::env::args();
for i in args {
    println!("{}",i);
}

// 命令行输入
let mut buf = String::new();
stdin().read_line(&mut buf).expect("failed to read line.");
println!("Your input line is \n{}",buf);
```

+ 读取文件内容 -- `std::fs` 

  使用fs::read_to_string 接收filename，打开文件，返回包含其内容的Result< String >

```rust
use std::{env, string, fs};

// 读取文件
// 一次性读取文件 适合小文件
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
```



+ 文件内容写入

  ```rust
  use std::fs;
  
  
  // 一次性写入
  fn main() {
      // 会直接覆盖原始内容
      fs::write("D:\\text.txt", "FROM RUST PROGRAM")
          .unwrap();
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
  ```
  
  



## 并发

+ thread::spawn 创建一个新线程

  ```rust
  use std::thread;
  use std::time::Duration;
  
  fn main() {
      // 返回值类型是JoinHandle
      thread::spawn(|| {
          for i in 1..10 {
              println!("hi number {} from the spawned thread!", i);
              thread::sleep(Duration::from_millis(1));
          }
      });
  
      for i in 1..5 {
          println!("hi number {} from the main thread!", i);
          thread::sleep(Duration::from_millis(1));
      }
  }
  ```

  

+ join 等待所有线程结束

  ```rust
  use std::thread;
  use std::time::Duration;
  
  fn main() {
      let handle = thread::spawn(|| {
          for i in 1..10 {
              println!("hi number {} from the spawned thread!", i);
              thread::sleep(Duration::from_millis(1));
          }
      });
  	
      // 如果将 handle.join().unwrap();放在此处,则主线程会等待直到新建线程执行完毕之后才开始执行for 循环
      
      for i in 1..5 {
          println!("hi number {} from the main thread!", i);
          thread::sleep(Duration::from_millis(1));
      }
  
      handle.join().unwrap();
      // 通过调用handle的join方法会阻塞当前线程直到handle所代表的线程结束
      
  }
  ```

  

+ channel在线程之间通讯

  ```rust
  use std::{sync::mpsc, thread};
  
  fn main (){
      // mspc 是多个生产者,单个消费者(multiple producer, single consumer)
      // tx 作为发送者(transmitter)    rx 作为接收者(receiver)
      let ( tx, rx ) = mpsc::channel();
  
      thread::spawn(move || {
          let val = String::from("hi");
          // 通道的发送端有一个 send方法 用来获取需要放入通道的值,send方法返回一个Result<T,E>类型,所以如果接收端已经被丢弃了,将没有发送值的目标,所以发送操作会返回错误
          tx.send(val).unwrap();
      });
  
      // 通道的接收端有两个有用的方法:recv和try_recv. recv 会阻塞主线程执行直到从通道中接受一个值. try_recv 不会阻塞,相反它会立刻返回一个Result<T,E>:ok 值包含可用的信息,而err值代表此时没有任何消息
      let received = rx.recv().unwrap();
      println!("Got: {}", received);
  }
  ```

  

+ mutex 共享状态

  互斥器(mutex) 其只允许一个线程访问某些数据。

  ```rust
  use std::sync::{Arc, Mutex};
  use std::thread;
  
  
  fn main() {
      // 使用关联函数new来创建一个Mutex<T>。
      // Mutex 是一个智能指针
      let m = Mutex::new(5);
  
      {
          // 使用lock方法获取锁，以访问互斥器中的数据。这个调用会阻塞当前线程，直到我们拥有锁为止
          let mut num = m.lock().unwrap();
          *num = 6;
      }
  
      println!("m = {:?}", m);
  }
  
  
  
  fn main() {
      // Arc是一个安全用于并发环境的原子引用计数类型，替代了Rc智能指针
      let counter = Arc::new(Mutex::new(0));
      let mut handles = vec![];
  
      for _ in 0..10 {
          let counter = Arc::clone(&counter);
          let handle = thread::spawn(move || {
              let mut num = counter.lock().unwrap();
  
              *num += 1;
          });
          handles.push(handle);
      }
  
      for handle in handles {
          handle.join().unwrap();
      }
  
      println!("Result: {}", *counter.lock().unwrap());
  }
  ```

  

+ Send 和 Sync trait 

  通常并不需要手动实现Send 和Sync trait，因为由Send 和Sync 的类型组成的类型，自动就是Send 和 Sync的。因为他们是标记trait，甚至不需要实现任何方法。他们只是用来加强并发相关的不可变性。

  手动实现trait涉及到编写不安全的代码

  Send trait允许在线程间转移所有权
  
  > Send trait表明实现了Send 的类型值的所有权可以在线程间传送
  
  Sync允许多线程访问
  
  > Sync trait 表明一个实现了Sync 的类型可以安全的在多个线程中拥有其值的引用
  
  

## 面向对象

+ 封装

  使用 `pub` 关键字来决定模块、类型、函数和方法是公有的，而默认情况下其他一切都是私有的

+ 继承

  默认实现

+ 多态

  在 Rust 中，通过特性（trait）实现多态





## 高级特性

1. 不安全Rust ：用于当需要舍弃Rust某些保证并负责手动维护
2. 高级trait ：与trait 相关的关联类型，默认类型参数，完全限定语法，超 trait和 new type 模式
3. 高级类型 ： 关于newtype 模式的更多内容，类型别名，never类型和动态大小类型
4. 高级函数和闭包 ： 函数指针和返回闭包
5. 宏



+ 不安全rust
  + 解引用裸指针
  + 调用不安全的函数或者方法
  + 访问或可变静态变量
  + 实现不安全trait
  + 访问 union 的字段

+ 高级trait

  + 关联trait
  + 默认类型参数

+ 高级类型

  + 类型别名

+ 宏

  宏可以接收不同数量的参数。宏最后以展开的方式来生成比你所手写出的更多代码

  + 声明宏 -- 匹配对应模式然后以另一部分的代码替换当前代码

    ```rust
    // #[macro_export] 注解说明宏应该是可用的。如果没有该注解，这个宏不能被引入作用域
    #[macro_export]
    macro_rules! vec {
        // 最外层的 $() ，捕获了符合括号内模式的值以用于替换后的代码
        // $()内的$x:expr ,其匹配rust的任意表达式，并将该表达式记作 $x
        // $()之后的逗号说明一个可有可无的逗号分隔符可以出现在 $() 所匹配的代码之后
        // 紧随逗号之后的* 说明该模式匹配多个或者零个* 之前的任何模式
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    
    // let vec:Vec<u32> = vec![1,2,3];
    
    // 上述宏等价于,以下述代码替换vec![1,2,3]
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_cev
    ```
  
  + 过程宏 -- 接收rust代码作为输入，在这些代码上进行操作，然后产生另一些代码作为输出
  
    + 自定义派生
    
      #[derive] 宏，用于struct 和 enum，可以为其指定随derive属性添加的代码 	
    
    + 类属性
    
      属性宏可以用于任意条目，例如函数
    
    + 类函数



## 数据库

1. 添加依赖 到 Cargo.toml 

   ```rust
   [dependencies]
   mysql = "*"
   chrono = "0.4"  // 时间处理库：用来处理日期和时间列
   ```

2. 编写代码

   ==查询时间？==

   ```rust
   use mysql::*;
   use mysql::prelude::*;
   
   fn main(){
       //设置连接字符串
       let url="mysql://root:123456@localhost:3306/rust_test";
       let opts = Opts::from_url(url).unwrap();
       //创建连接池
       // 注意：老版本是直接传url字符串即可，新版本要求必须转换为opts类型
       let pool= Pool::new(opts).unwrap();
       // Pool::new(url).unwrap();
   
       //连接数据库
       let mut conn=pool.get_conn().unwrap();
   
       //数据库操作
       //1.查询user表
       //方式1：流式查询  数据逐行读取，数据不会存储在内存中
       conn.query_iter("Select name,age,gender from person").unwrap()
       .for_each(|row|{
           let r:(String,i32,String)=from_row(row.unwrap());
           println!("name={},age={},gender={}",r.0,r.1,r.2);
       });
   
       //方式2：将数据集取出存储在Vec中
       let res:Vec<(String,i32,String)>=conn.query("Select name,age,gender from person").unwrap();
       for r in res{
           println!("name={},age={},gender={}",r.0,r.1,r.2);
       }
       
       //方式3：将数据转换成Struct
       struct Person{
           name:String,
           age:i32,
           gender:String
       }
       let res=conn.query_map("Select name,age,gender  from person",
               |(name,age,gender)|Person{
                   name,
                   age,
                   gender
                   }
               ).expect("QUERY FAILED");
       for person in res{
           println!("name={},age={},gender={}",person.name,person.age,person.gender);
       }
   }
   ```

   





## Rust Web

TCP 是一个底层协议，它描述了信息如何从一个 server 到另一个的细节，不过其并不指定信息是什么。HTTP 构建于 TCP 之上，它定义了请求和响应的内容

+ TCP

  + 客户端

    ```rust
    // 客户端
    // std::io::prelude 引入作用域来获取读写流所需的特定trait
    use std::io::{Error,Read,Write};
    use std::net::{TcpListener,TcpStream};
    use std::thread;
    use std::time;
    
    fn handle_client(mut stream:TcpStream) -> Result<(),Error>{
        let mut buf = [0;512];
        for _ in 0..1000 {
            let bytes_read = stream.read(&mut buf)?;
            if bytes_read == 0 {
                return Ok(());
            }
    
            stream.write(&buf[..bytes_read])?;
            thread::sleep(time::Duration::from_secs(1))
        }
    
        Ok(())
    }
    
    
    fn main () -> std::io::Result<()>{
        // 使用bind函数监听地址
        // bind 函数返回Result<T,E>
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        let mut thread_vec : Vec<thread::JoinHandle<()>> = Vec::new();
    
        // incoming函数是个迭代器
        for stream in listener.incoming(){
            let stream = stream.expect("failed!");
            let handle = std::thread::spawn(move || {
                handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error));
            });
    
            thread_vec.push(handle);
        }
    
        for handle in thread_vec{
            handle.join().unwrap();
        }
        
        Ok(())
    }
    ```

    

  + 服务端

    ```rust
    // 服务端
    use std::io::{self,prelude::*,BufReader,Write};
    use std::net::TcpStream;
    use std::str;
    
    fn main() -> std::io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        for _ in 0..10 {
            let mut input = String::new();
            // 从控制台读取输入
            io::stdin().read_line(&mut input).expect("failed to read from stdin");
            // 写数据到stream流中
            stream.write(input.as_bytes()).expect("failed to write to stream");
            
            let mut reader = BufReader::new(&stream);
    
            let mut buffer :Vec<u8> = Vec::new();
    
            reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
    
            println!("{}",str::from_utf8(&buffer).expect("Could not write buffer as string"));
            println!("");
        }
    
        Ok(())
    }
    ```

    

+ HTTP

  + 客户端

    ```rust
    
    ```
  
  
  
  + 服务端
    
      ```rust
      
    ```
    
    
  
    



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

  使用use 引入mod时候，依然遵循私有性原则，mod中的私有方法等仍然无法访问

  ```rust
  mod front_of_house {
      pub mod hosting {
         	pub fn add_to_waitlist(){} 
      }
  }
  
  use crate::front_of_house::hosting; // 绝对路径
  
  use front_of_house::hosting; // 相对路径
  
  use std::collections::HashMap;
  
  pub fn eat_at_restaurant(){
      // 一般引用到所需函数的上级目录(指定到父级)，避免与本地可能有同名函数，无法区别
      // struct、enum、其他: 指定完整路径(指定到本身)
      hosting::add_to_waitlist();
      let mut map = HashMap::new();
  }
  ```

  以上的代码可以拆分到不同文件

+ as关键字

  为引入的路径指定本地的别名

  ```rust
  use std::io::Result as IOResult;
  ```




使用第三方库时，使用extern crate

```rust
extern crate my_library;
// 通过 extern crate 引入的 crate 或者类库，需要通过两个 :: 双引号逐层将需要引用的module模块，直到调用的功能函数为止
fn main() {
    println!("Hello in English: {}", my_library::english::greetings::hello());
    println!("Goodbye in English: {}", my_library::english::farewells::goodbye());

    println!("Hello in Chinese: {}", my_library::chinese::greetings::hello());
    println!("Goodbye in Chinese: {}", my_library::chinese::farewells::goodbye());
}

pub mod greetings;
pub mod farewells;
pub use self::greetings::hello;
pub use self::farewells::goodbye;
//self 指的是当前模块,super 指的是上级模块


extern crate my_library as sayings;
use sayings::chinese::greetings as ch_greetings;
use sayings::chinese::farewells::*;
// 使用嵌套路径，清理大量use语句
use sayings::english::{self, greetings as en_greetings, farewells as en_farewells};
// 上面一句等价于
use sayings::english;
use sayings::english::greetings as en_greetings;
use sayings::english::farewells as en_farewells;




// pub(crate) use 和 pub use区别：
pub use T 导出了T可以被其他crate使用；使用pub(crate) use T 只把T导到当前的crate，其他crate访问不了
```







## Rust异步编程

[异步编程文档](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)



多线程虽然比多进程节省资源，但依然存在昂贵的系统内核调度代价

> 异步编程：消息的发送方不需要等待消息返回就可以处理其它事情
>
> 同步编程：消息的发送方要等待消息返回才能继续处理其它事情
>
> 
>
> + 除了concurrent并发模型以外的，其他并发模型：
>   + OS线程
>     + 无需改变编程模型，线程间同步困难，性能开销大
>     + 线程池可以降低一些成本，但难以支撑大量IO绑定的工作
>   + Event-driven编程
>     + 与回调函数一起用，可能高效
>     + 非线性的控制流，数据流和错误传播难以追踪
>   + Coroutines
>     + 类似线程，无需改变编程模型
>     + 类似async，支持大量任务
>     + 抽象掉了底层细节(这对系统编程、自定义运行时的实现很重要)
>   + Actor编程
>     + 将所有并发计算划分为actor，消息通信易出错
>     + 可以有效的实现actor模型，但许多实际问题没解决(例如流控制，重试逻辑)



async编程，是一种并发(concurrent)编程模型

允许你在少数系统线程上运行大量的并发任务。即不使用多线程，但能达到多线程的效果

虽然Rust本身就支持async编程，但很多应用依赖于社区的库：标准库提供了最基本的特性、类型和功能，例如Future trait。async/await 语法直接被Rust编译器支持。futures crate提供了许多实用类型、宏和函数。它们可以用于任何异步应用程序。异步代码、IO和任务生成的执行由“async runtime”提供，例如Tokio和async-std。大多数async应用程序和一些async crate都依赖于特定的运行时





通过async/await语法，看起来和同步编程差不多

> + rust中的async
>   + Future是惰性的
>     + 只有poll时才能取得进展；被丢弃的furute就无法取得进展了
>   + Aysnc 是零成本的
>     + 使用async，可以无需堆内存分配，和动态调度，对性能大好，且允许在受限环境使用async
>   + 不提供内置运行时，可以通过tokio等实现 
>     + 运行时由社区提供
>   + 单线程、多线程均支持
>     + 但优缺点不同

> + rust中的async 和线程(thread)
>   + OS线程thread
>     + 适用于少量任务，有内存和CPU开销，且线程生成和线程间切换非常昂贵
>     + 线程池可以降低一些成本
>     + 允许重用同步代码，代码无需大改，无需特定编程模型
>     + 有些系统支持修改线程优先级
>   + Async
>     + 显著降低内存和CPU开销
>     + 同等条件下，支持比线程多几个数量级的任务(少量线程支撑大量任务)
>     + 可执行文件大(需要生成状态机，每个可执行文件捆绑一个异步运行时)



async/await 

+ async
  + async把一段代码转换为一个实现了Future trait的状态机
    + 虽然在同步方法中调用阻塞函数(async 转化的函数)会阻塞整个线程，但阻塞的Future将放弃对线程的控制，从而允许其他Future来运行。
    
      futures::executor::block_on 阻挡当前线程，直到提供的Future运行完成
    
    + 在async fn中，.await 不会阻塞当前线程，而是异步的等待Future的完成(如果该Future目前无法取得进展，就允许其他任务运行)
    
    + rust不允许在trait中声明async 函数

语法：

```rust
// 异步函数语法
async fn do_something(){}
// 返回的是Future， Future需要由一个执行者来运行
// 上述代码等价于
fn do_something() -> impl Future<Output=()>{
    async {
        println!("Hello");
    }
}

// futures::executor::block_on;
// block_on 阻塞当前线程，直到提供的Future运行完成


fn main() {
    let song = block_on(learn_song());
    // block_on 阻塞当前线程，直到提供的Future运行完成
    block_on(sing_song(song));
    block_on(dance());
}

async fn hello_world(){ // 返回futures::Future类型
    println!("hello world");
}

struct Song{}

async fn learn_song()->Song{
    println!("learn_song()");
    Song{}
}
async fn sing_song(song:Song){
    println!("sing_song()");
}

async fn learn_and_sing(){
    // 在async fn中，可以使用.await 来等待另一个实现Future trait的完成
    // 与block_on 不同， .await不会阻塞当前线程，而是异步的等待Future的完成（如果该Future目前无法取得进展，就允许其他任务运行）
    let song = learn_song().await;
    sing_song(song).await;
}

async fn dance(){
    println!("dance()");
}
```

+ Future trait

  rust async编程的核心，Future是一种异步计算，它可以产生一个值，实现Future的类型表示目前可能还不可用的值

  Future代表一种你可以检验其是否完成的操作：Future可以通过调用poll函数来取得进展。

  poll函数会调用Future尽可能接近完成。如果Future完成了，就发送你会poll::Ready(result)，其中result就是最终的结果；如果Future还无法完成：就返回poll::Pending，并当Future准备好取得更多进展时调用一个waker的wake()函数

  > wake函数：当wake()函数被调用时，当执行器将驱动Future再次调用poll函数，以便Future能取得更多进展。
  >
  > 没有wake函数，执行器就不知道特定的Future何时能取得进展（就得不断的poll）
  >
  > 通过wake函数，执行器就确切的知道哪些Future已经准备好进行poll的调用 	 

  针对Future，你唯一能做的就是使用poll来敲它，直到一个值掉出来

  ![image-20220421200345761](Rust.assets/image-20220421200345761.png)



+ Waker  类型的作用

  Future在第一次poll的时候通常无法完成任务，所以Future需要保证在准备好取得更多进展后，可以再次被poll

  每次Future被poll，它都是作为一个任务的一部分

  任务（task）就是被提交给执行者的顶层的Future

  Waker提供了wake()方法，它可以被用来告诉执行者：相关的任务应该被唤醒

  当wake()被调用，执行者知道Waker所关联的任务已经准备好取得更多进展，Future应该被再次poll

  Waker实现了clone()，可以复制和存储



+ Executor

  Future是惰性的，除非驱动它们来完成，否则就什么都不做



async/.await 是rust的特殊语法，在发生阻塞时，它让放弃当前线程的控制权成为可能，这就允许在等待操作完成的时候，允许其他代码取得进展

async fn 和async blocks 都返回实现了Future trait的值。

async体和其他future都是惰性的，在真正运行之前什么都不做

使用.await 是最常见的运行future的方式

对future使用.await 就会尝试驱动器运行至完成

如果Future被阻塞：它会放弃当前线程的控制权；当可取的更多进展时，执行器会捡起这个Future并恢复执行，最后由 .await 完成解析

```rust
async fn foo()->u8 {5}

fn bar() -> impl Future<Output = u8> {
    async {
        let x :u8= foo().await;
        x+5
    }
}
```





注：rust没有反射的能力，因此其无法在运行时获取类型信息



## 问题

+ rust cargo build一直出现 Blocking waiting for file lock on package cache？

  命令行在当前工程目录下输入 `where cargo`, 得到cargo路径：如C:\Users\Administrator\.cargo\ ，进进入C:\Users\Administrator\.cargo\，ls -al 查看隐藏文件，如果 .package-cache  存在，则删除

  

+ cargo build` 时报错：`couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll'], set the 'LIBCLANG_PATH' environment variable to a path where one of these files can be found (invalid: [])

  在Windows 上
  访问 https://www.rust-lang.org/tools/install 页面并按照说明安装 Rust。

  安装需要C++ 的构建工具（C++ build tools） 
  访问 https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/ 获取。
  安装时，确保已选择 “C++ build tools”，并包括 Windows 10 SDK 和英文语言包。

  



+  vscode 链接远程主机，然后开发

  [vscode链接远程主机](https://www.bilibili.com/read/cv15857143/)

+ 明明在`Cargo.toml`中的`dependencies`已经引入了`crate`，如：`cargo add xxxx`，但是在代码中引入`crate`，就是会提示如下错误：unresolved import

  ```bash
  # 此时可以试着，在项目根目录下面，运行如下命令：
  cargo clean
  cargo build
  ```

  

+ \#[inline] 内联

  编译器用内联函数的函数代码 替换 函数调用，从而提高程序运行速度，但是需要占用更多内存。

  程序代码请求将函数作为内联函数时，[编译器](https://so.csdn.net/so/search?q=编译器&spm=1001.2101.3001.7020)并不一定会满足这种要求，编译器可能认为该函数过大 或 注意到 函数调用了自己（**内联函数不能递归**）。