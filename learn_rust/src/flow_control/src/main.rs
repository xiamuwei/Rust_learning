
// 流程控制
fn main (){
    // if_control();
    // loop_control1();
    while_let_control();
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
fn while_control (){
    let mut n = 1;
    while n < 101 {
        if n %15 ==0{
            println!("fizzbuzz");
        }else if n % 3==0{
            println!("buzz");
        }else {
            println!("{}",n);
        }

        n+=1;
    }
}
fn while_control1 (){
    let mut number = 3;
    while number != 0 {
        println!("number = {}",number);
        number -= 1;
    }
}


// for 循环
fn for_control (){
    for i in (1..4).rev(){
        println!("{}",i);
    }   
    for i in (1..=4).rev(){
        println!("{}",i);
    }   


    // 左开右闭
    for i in 1..4{
        println!("{}",i);
    }
    // 两边都包括
    for i in 1..=4{
        println!("{}",i);
    }
}

// match 匹配
fn match_control(){
    let number = 13;
    println!("Tell me about {}",number);

    match number {
        1 => println!("One!"),
        2 | 3 | 13 | 5 | 7 => println!("This is a prime"),
        13..=19 =>println!("A teen"),
        _ => println!("Ain't special"),
    }


    let boolean = true ;
    let binary = match boolean{
        true => 1,
        false => 0,
    };
    println!("{} -> {}",boolean,binary);
}


// 匹配守卫
fn guard_control(){
    let pair:(i32,i32) = (32,64);
    match pair {
        (x,y) if x ==y => println!("this is the same value"),
        (x,y) if x !=y => println!("this is not the same value"),
        _ => println!("no idea"),
    }
}


fn valuetToVariable(){
    let num = 2;
    match num {
        n@1..=8 => println!("num = {}",num),
        n@9 => println!("num = 9"),
        _ => println!("no idea"),
    }
}


fn ifLetControl(){
    let number = Some(7);
    let letter:Option<i32> = None;
    let emoction:Option<i32> = None;

    if let Some(i) = number{
        println!("Matched {:?}!",i);
    }

    // 
    if let Some(i) = letter {
        println!("Matched {:?}!",i);
    }else {
        println!("Dont match a number ,lets go with a letter");
    };


    let i_like_letters = false;

    if let Some(i) = emoction {
        println!("Matched {:?}!",i);
    }else if i_like_letters{
        println!("Didn't match a number. Let's go with a letter!");
    }else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}

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