use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    let task = tokio::spawn(async move {
        loop {
            let mut counter = counter_clone.lock().await;
            *counter += 1;
            println!("i = {}", *counter);
            drop(counter);

            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    println!("Got Ctrl+C signal, waiting for loop to finish...");
                    break
                }
                _ = time::sleep(Duration::from_nanos(1)) => {}
            }
        }
    });

    task.await.expect("Task panicked");

    let counter = counter.lock().await;
    println!("Loop finished, exiting. final count = {}", *counter);
}
