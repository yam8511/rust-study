use std::{error::Error, time::Duration};

use tokio::time::{self};

fn main() -> Result<(), Box<dyn Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        println!("Hello, world!");

        let task_1 = tokio::spawn(async {
            let mut i = 0;
            loop {
                i += 1;
                println!("AAA => {i}");
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {
                        println!("AAA | Got Ctrl+C signal, waiting for loop to finish...");
                        break
                    }
                    _ = time::sleep(Duration::from_secs(1)) => {}
                }
            }
        });

        let task_2 = tokio::spawn(async {
            let mut i = 0;
            loop {
                i += 1;
                println!("BBB => {i}");
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {
                        println!("BBB | Got Ctrl+C signal, waiting for loop to finish...");
                        break
                    }
                    _ = time::sleep(Duration::from_secs(2)) => {}
                }
            }
        });

        let task_3 = tokio::spawn(async {
            let mut i = 0;
            loop {
                i += 1;
                println!("CCC => {i}");
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {
                        time::sleep(Duration::from_secs(3)).await;
                        println!("CCC | Got Ctrl+C signal, waiting for loop to finish...");
                        break
                    }
                    _ = time::sleep(Duration::from_secs(3)) => {}
                }
            }
        });

        tokio::signal::ctrl_c().await.unwrap(); // get first signal

        tokio::select! { // get second signal
            _ = tokio::signal::ctrl_c() => {
                println!("\nGot Ctrl+C signal again. exit!");
            }
            _ = time::sleep(Duration::from_secs(30)) => {
                println!("timeout 30s")
            }
            _ = async{
                task_1.await.unwrap();
                task_2.await.unwrap();
                task_3.await.unwrap();
            }=>{
                println!("all done")
            }
        }
    });
    println!("finish async world!");
    Ok(())
}
