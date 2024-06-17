use std::time::Duration;

use tokio::{
    sync::broadcast::{Receiver, Sender},
    time,
};

async fn task_camera(i: i32, tx: Sender<i32>, mut rx: Receiver<i32>) {
    let now = chrono::Local::now();
    println!("task{i} start {now:?}");

    let stop_signal = Box::pin(tokio::signal::ctrl_c());
    tokio::pin!(stop_signal);

    loop {
        tokio::select! {
            _ = &mut stop_signal => {
                println!("Got Ctrl+C signal, break loop... task{i}");
                break
            }
            _= rx.recv() => {
                println!("Got cancel, break loop... task{i}");
                break
            }
            _ = time::sleep(Duration::from_secs((5 - i)as u64)) => {
                println!("Finish, break loop... task{i}");
                break
            }
        }
    }
    _ = tx.send(i);
}

#[tokio::main]
async fn main() {
    let (tx, _) = tokio::sync::broadcast::channel(2);
    let mut tasks = vec![];
    for i in 1..=4 {
        println!("{i}");
        let tx = tx.clone();
        let rx = tx.subscribe();
        let task = tokio::spawn(async move { task_camera(i, tx, rx).await });
        tasks.push(task)
    }

    for task in tasks {
        task.await.unwrap()
    }
    println!("====== broadcast task finish ======");
}
