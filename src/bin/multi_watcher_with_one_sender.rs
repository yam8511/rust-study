use std::{collections::BTreeMap, sync::Arc, time::Duration};

use tokio::{
    select,
    signal::ctrl_c,
    sync::{
        broadcast::{self, Receiver},
        Mutex,
    },
    time::sleep,
};

#[tokio::main]
async fn main() {
    let xml_map = Arc::new(Mutex::new(BTreeMap::new()));
    let (otx, _ctx) = broadcast::channel(1);
    // let otx = Arc::new(otx);
    let otx2 = otx.clone();
    tokio::spawn(async move {
        ctrl_c().await.unwrap();
        otx2.send(()).unwrap();
    });

    let state = xml_map.clone();
    let mut ctx = otx.subscribe();
    let task3 = tokio::spawn(async move {
        let key = "aaa".to_string();
        let mut i = 0;
        let (tx, mut _rx) = tokio::sync::broadcast::channel(1);
        let tx = Arc::new(tx);
        {
            let mut map = state.lock().await;
            map.insert(key.clone(), tx.clone());
        }
        loop {
            i += 1;
            println!("send {i}");
            if let Err(e) = tx.send(i) {
                println!("send error: {e:?}");
            }

            select! {
                _ = ctx.recv() => {
                    break;
                }
                _ = sleep(Duration::from_secs(1)) => {}
            }
        }
    });

    let state = xml_map.clone();
    let mut ctx = otx.subscribe();
    let task = tokio::spawn(async move {
        // sleep(Duration::from_secs(1)).await;
        let now = chrono::Local::now();
        println!("task1 start {now:?}");
        let key = "aaa".to_string();

        loop {
            let mut rx2: Receiver<i32> = {
                let map = state.lock().await;
                let tx = map.get(&key);
                if let Some(tx) = tx {
                    println!("task1 subscribe");
                    tx.subscribe()
                } else {
                    println!("task1 no tx can subscribe");
                    continue;
                }
            };

            loop {
                select! {
                    result = rx2.recv()=> {
                        match result {
                            Ok(v) => println!("task1 recv => {v}"),
                            Err(e) => match e {
                                tokio::sync::broadcast::error::RecvError::Closed => {
                                    println!("task1 rx closed: {e:?}");
                                    break;
                                }
                                tokio::sync::broadcast::error::RecvError::Lagged(v) => {
                                    println!("task1 Lagged: {v}");
                                    continue;
                                }
                            },
                        }
                    }
                    _ = sleep(Duration::from_secs(10)) => {
                        println!("task1 recv timeout");
                        break;
                    }
                    _ = ctx.recv() => {
                        return;
                    }
                }
                sleep(Duration::from_secs(2)).await
            }
        }
    });

    let state = xml_map.clone();
    let mut ctx = otx.subscribe();
    let task2 = tokio::spawn(async move {
        // sleep(Duration::from_secs(2)).await;
        let now = chrono::Local::now();
        println!("task2 start {now:?}");
        let key = "aaa".to_string();

        loop {
            let mut rx2: Receiver<i32> = {
                let map = state.lock().await;
                let tx = map.get(&key);
                if let Some(tx) = tx {
                    println!("task2 subscribe");
                    tx.subscribe()
                } else {
                    println!("task2 no tx can subscribe");
                    continue;
                }
            };

            loop {
                select! {
                    result = rx2.recv()=> {
                        match result {
                            Ok(v) => println!("task2 recv => {v}"),
                            Err(e) => match e {
                                tokio::sync::broadcast::error::RecvError::Closed => {
                                    println!("task2 rx closed: {e:?}");
                                    break;
                                }
                                tokio::sync::broadcast::error::RecvError::Lagged(v) => {
                                    println!("task2 Lagged: {v}");
                                    continue;
                                }
                            },
                        }
                    }
                    _ = sleep(Duration::from_secs(10)) => {
                        println!("task2 recv timeout");
                        break;
                    }
                    _ = ctx.recv() => {
                        return;
                    }
                }
                sleep(Duration::from_secs(2)).await
            }
        }
    });

    task.await.unwrap();
    task2.await.unwrap();
    task3.await.unwrap();
    println!("====== broadcast task finish ======");
}
