use std::time::Duration;

use tokio::{select, signal::ctrl_c, spawn, time::sleep};

async fn task() {
    let mut a = 0;
    loop {
        a += 1;
        println!("{a}");
        sleep(Duration::from_secs(1)).await
    }
}

#[tokio::main]
async fn main() {
    select! {
        _ = task() => {},
        _= sleep(Duration::from_secs(6))=>{}
    }
    println!("after select, task will be cancelled");

    let forever_task = spawn(async { task().await });
    select! {
        _ = forever_task => {},
        _= sleep(Duration::from_secs(6))=>{}
    }
    println!("pending, task inside spawn still running");
    ctrl_c().await.unwrap();
    println!("finish")
}
