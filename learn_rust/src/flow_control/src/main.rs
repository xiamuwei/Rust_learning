
// 流程控制
fn main (){
    // if_control();
    loop_control1();
}

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

fn loop_control1(){
    let mut count = 0;
    'counting_up :loop{
        let mut remaining = 10;
        loop{
            println!("remaining = {}", remaining);
            println!("count = {}", count);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1 ;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

// while 