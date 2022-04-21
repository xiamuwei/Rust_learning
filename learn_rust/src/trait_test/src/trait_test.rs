trait T1{
    fn test(&self);
}


trait T2 :T1{
    fn exec(&self);
}


struct Person{}

impl T1 for Person{
    fn test(&self){
        println!("this is T test");
    }
}

impl T2 for Person{
    fn exec(&self){
        println!("this is T2 exec");
    }
}

fn test_trait(){
    let p = Person{};
    p.exec();
    p.test();
}