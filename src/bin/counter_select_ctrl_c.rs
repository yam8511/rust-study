use std::{error::Error, time::Duration};

use tokio::time::{self, sleep};

async fn task_do_something_long(name: String) {
    let now = chrono::Local::now();
    println!("{now} | task start | {name}");
    sleep(Duration::from_secs(2)).await;
    let now = chrono::Local::now();
    println!("{now} | task finish | {name}");
}
fn main() -> Result<(), Box<dyn Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        println!("Hello, world!");

        let mut tasks = vec![];
        let mut i = 0;
        loop {
            i += 1;
            let name = format!("{a} + {a} + {a}", a = i);
            let task = task_do_something_long(name);
            let task = tokio::spawn(async { task.await });
            tasks.push(task);
            println!("AAA => {i}");
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    println!("Got Ctrl+C signal, waiting for loop to finish...");
                    break
                }
                _ = time::sleep(Duration::from_nanos(1)) => {}
            }
        }
        for t in tasks.into_iter() {
            let _ = tokio::spawn(t).await.unwrap();
        }
    });
    println!("finish async world!");
    Ok(())
}
