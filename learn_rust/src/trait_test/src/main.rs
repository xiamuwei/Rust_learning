use std::fmt::*;
fn main() {
    // println!("Hello, world!");
    let new = NewArticle{
        headlines:String::from("headlines"),
        location:String::from("location"),
        author:String::from("author"),
        content:String::from("content"), 
    };
    println!("{}",new.summarize());
    println!("{}",new.summarize_default());
}


// 定义trait 
pub trait Summary{
    fn summarize(&self) -> String;

    fn summarize_default (&self)-> String{
        String::from("this is a test")
    }
}

// trait bound
fn notify(item1:impl Summary,item:impl Summary){}
fn notify1<T:Summary>(item1:T,item2:T){}
// + 绑定多个trait
fn notify3(item1:impl Summary + Display){}
fn notify4<T:Summary+Display>(item1:T){}
// where 简化trait bound
fn some_function<T:Display+Clone,U:Clone+Debug> (t:T,u:T){}
fn some_function1<T,U>(t:T, u:U)
where  T:Display+Clone,
       U:Clone+Debug
{}


// 为类型实现trait
pub struct NewArticle{
    pub headlines:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

impl Summary for NewArticle{
    fn summarize(&self) -> String{
        format!("{} by {} ({})",self.headlines,self.author,self.location)
    }
}

pub struct Tweet {
    pub username :String,
    pub content :String,
    pub reply:bool,
    pub retweet:bool,
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("{} :{}",self.username,self.content)
    }
}