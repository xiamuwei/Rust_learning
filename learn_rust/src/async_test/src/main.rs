use futures::executor::block_on;

fn main() {
    block_on(async_main());

    // let song = block_on(learn_song());
    // block_on(sing_song(song));
    // block_on(dance());

    // // println!("Hello, world!");
    // let future = hello_world();  
    // block_on(future);  // block_on 阻塞当前线程，直到提供的Future运行完成
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
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main(){
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1,f2);
}

async fn dance(){
    println!("dance()");
}
