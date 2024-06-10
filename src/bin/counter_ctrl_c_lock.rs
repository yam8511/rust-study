use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal::ctrl_c;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    let task = tokio::spawn(async move {
        while running_clone.load(Ordering::Relaxed) {
            let mut counter = counter_clone.lock().await;
            *counter += 1;
            println!("i = {}", *counter);

            sleep(Duration::from_nanos(1)).await;
        }
    });

    let _ = ctrl_c().await;
    println!("Got Ctrl+C signal, waiting for loop to finish...");

    // Set running to false to break the loop
    running.store(false, Ordering::Relaxed);

    // Wait for the loop to finish
    if let Err(e) = task.await {
        eprintln!("Error occurred while waiting for task to finish: {}", e);
    }

    let counter = counter.lock().await;
    println!("Loop finished, exiting. final count = {}", *counter);
}
