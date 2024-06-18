use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::{
    sync::{
        broadcast::{Receiver, Sender},
        Mutex,
    },
    time::sleep,
};

async fn task_camera(
    i: i32,
    tx: Sender<i32>,
    mut rx: Receiver<i32>,
    counter: Arc<Mutex<i32>>,
    frame_map: Arc<Mutex<HashMap<String, i32>>>,
) {
    let now = chrono::Local::now();
    println!("task{i} start {now:?}");

    let stop_signal = Box::pin(tokio::signal::ctrl_c());
    tokio::pin!(stop_signal);

    let mut count = 0;
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
            _ = sleep(Duration::from_nanos(1)) => {}
        }
        let mut num = counter.lock().await;
        *num += 1;
        let num = *num;
        count += 1;
        {
            let mut map = frame_map.lock().await;
            map.insert(format!("CAM{}", i), count);
        }
        println!("Finish, add 1 and sum = {num}... task{i} = {count}");
        if num == 100 {
            println!("Finish, break loop... task{i}");
            break;
        }
    }
    _ = tx.send(i);
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let frame_map = Arc::new(Mutex::new(HashMap::new()));

    let (tx, _) = tokio::sync::broadcast::channel(1);
    let mut tasks = vec![];
    for i in 1..=4 {
        println!("{i}");
        let tx = tx.clone();
        let rx = tx.subscribe();
        let counter = counter.clone();
        let frame_map = frame_map.clone();
        let task = tokio::spawn(async move { task_camera(i, tx, rx, counter, frame_map).await });
        tasks.push(task)
    }

    for task in tasks {
        task.await.unwrap()
    }

    let map = frame_map.lock().await;
    for (key, val) in map.iter() {
        println!("{} => {}", key, val)
    }
    println!("====== broadcast task finish ======");
}
