use std::{error::Error, time::Duration};

use log::LevelFilter;
use tokio::time::{self};

static MY_LOGGER: MyLogger = MyLogger { path: "aaa.log" };

struct MyLogger {
    path: &'static str,
}

impl log::Log for MyLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[zuolar]{} {} | {}`",
                self.path,
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

fn main() -> Result<(), Box<dyn Error>> {
    // env_logger::init();
    log::set_logger(&MY_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Info);

    log::debug!("env logger init {} {}", "aa", 123);

    let rt = tokio::runtime::Runtime::new()?;

    log::debug!("start tokio runtime");
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
    log::info!("finish async world!");
    Ok(())
}
