fn main() {
    println!("Hello, world!");
    let str = longest("str12");
    println!("str = {}",str);
}


// fn longest(x:&str, y :&str) -> &str{
fn longest(x:&str) -> &str{
    let y = " str"; // 所有的字符串字面值都拥有'static 生命周期
    if x.len() < y.len(){
        x 
    }else {
        y
    }
}

fn longest_lfe<'a>(x:&'a str ,y :&'a str) -> &'a str{

    if x.len() < y.len(){
        x 
    }else {
        y
    }
} 