use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    {
        // oneshot
        let (tx, rx) = tokio::sync::oneshot::channel();
        let task = tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;
            let now = chrono::Local::now();
            println!("task start {now:?}");
            match rx.await {
                Ok(v) => {
                    println!("changed {v:?}")
                }
                Err(e) => {
                    println!("error: {e:?}");
                }
            }
        });

        println!("send 123");
        tx.send(123).unwrap();

        task.await.unwrap();
        println!("====== oneshot task finish ======");
    }

    {
        // watch
        let (tx, mut rx) = tokio::sync::watch::channel(789);
        let task = tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;
            let now = chrono::Local::now();
            println!("task start {now:?}");
            loop {
                match rx.changed().await {
                    Ok(_) => {
                        let v = *rx.borrow();
                        println!("changed {v:?}")
                    }
                    Err(e) => {
                        println!("error: {e:?}");
                        break;
                    }
                }
            }
        });

        println!("send 123");
        tx.send(123).unwrap();
        println!("send 456");
        tx.send(456).unwrap();
        drop(tx);

        task.await.unwrap();
        println!("====== watch task finish ======");
    }

    {
        // multiple sender, single receiver
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let task = tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;
            let now = chrono::Local::now();
            println!("task start {now:?}");
            loop {
                match rx.recv().await {
                    Some(v) => println!("recv => {v}"),
                    None => {
                        println!("no value (tx closed)");
                        break;
                    }
                }
            }
        });

        let tx2 = tx.clone();
        let task3 = tokio::spawn(async move {
            println!("send 567");
            tx2.send(567).unwrap();
            println!("send 890");
            tx2.send(890).unwrap();
            drop(tx2);
        });

        println!("send 123");
        tx.send(123).unwrap();
        println!("send 456");
        tx.send(456).unwrap();
        println!("send 789");
        tx.send(789).unwrap();
        drop(tx);

        task.await.unwrap();
        task3.await.unwrap();
        println!("====== mpsc unbounded_channel task finish ======");
    }

    {
        // multiple sender, single receiver
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let task = tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;
            let now = chrono::Local::now();
            println!("task start {now:?}");
            loop {
                match rx.recv().await {
                    Some(v) => println!("recv => {v}"),
                    None => {
                        println!("no value (tx closed)");
                        break;
                    }
                }
            }
        });

        let tx2 = tx.clone();
        let task3 = tokio::spawn(async move {
            println!("send 567 (will block until received)");
            tx2.send(567).await.unwrap();
            println!("send 890");
            tx2.send(890).await.unwrap();
            drop(tx2);
        });

        println!("send 123");
        tx.send(123).await.unwrap();
        println!("send 456 (will block until received)");
        tx.send(456).await.unwrap();
        println!("send 789");
        tx.send(789).await.unwrap();
        drop(tx);

        task.await.unwrap();
        task3.await.unwrap();
        println!("====== mpsc buffer channel (1) task finish ======");
    }

    {
        // broadcast
        let (tx, mut rx) = tokio::sync::broadcast::channel(2);
        let task = tokio::spawn(async move {
            // sleep(Duration::from_secs(1)).await;
            let now = chrono::Local::now();
            println!("task1 start {now:?}");
            loop {
                match rx.recv().await {
                    Ok(v) => println!("task1 recv => {v}"),
                    Err(e) => match e {
                        tokio::sync::broadcast::error::RecvError::Closed => {
                            println!("task1 tx closed: {e:?}");
                            break;
                        }
                        tokio::sync::broadcast::error::RecvError::Lagged(v) => {
                            println!("task1 Lagged: {v}")
                        }
                    },
                }
            }
        });

        let mut rx = tx.subscribe();
        let task2 = tokio::spawn(async move {
            // sleep(Duration::from_secs(1)).await;
            let now = chrono::Local::now();
            println!("task2 start {now:?}");
            loop {
                match rx.recv().await {
                    Ok(v) => println!("task2 recv => {v}"),
                    Err(e) => match e {
                        tokio::sync::broadcast::error::RecvError::Closed => {
                            println!("task2 tx closed: {e:?}");
                            break;
                        }
                        tokio::sync::broadcast::error::RecvError::Lagged(v) => {
                            println!("task2 Lagged: {v}")
                        }
                    },
                }
            }
        });

        println!("send 123");
        tx.send(123).unwrap();
        println!("send 456");
        tx.send(456).unwrap();
        println!("send 789");
        tx.send(789).unwrap();

        let tx2 = tx.clone();
        let task3 = tokio::spawn(async move {
            println!("send 567");
            tx2.send(567).unwrap();
            drop(tx2);
        });
        drop(tx);

        task.await.unwrap();
        task2.await.unwrap();
        task3.await.unwrap();
        println!("====== broadcast task finish ======");
    }
}
