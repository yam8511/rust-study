use std::io;

use chrono::Local;
// Import necessary libraries
use tokio::time::{self, Duration};
use tracing::*;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::writer::Tee;
use tracing_subscriber::{self, fmt::time::FormatTime};

// 用来格式化日志的输出时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}

#[macro_export]
macro_rules! write_log {
    ($file_name:expr, $closure:expr) => {
        let file_appender = tracing_appender::rolling::daily(".", $file_name);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        // 创建一个不调用init()方法的Subscriber，从TRACE级别开始记录
        let sc = tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .with_writer(Tee::new(non_blocking.clone(), io::stdout)) // 写入文件，将覆盖上面的标准输出
            .with_ansi(false) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
            .with_level(true)
            .with_target(true)
            .with_timer(LocalTimer)
            .finish(); // 初始化并将SubScriber设置为全局SubScriber

        tracing::subscriber::with_default(sc, || {
            // 即便全局Subscriber是WARN级别的，但这条记录仍然会被记录
            $closure();
        });
    };
}

#[tokio::main]
async fn main() {
    // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、设置日志的时间格式
    // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

    // 设置全局Subscriber，从WARN级别开始记录
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_writer(io::stdout) // 写入标准输出
        .event_format(format)
        .init(); // 初始化并将SubScriber设置为全局SubScriber

    let mut tasks = vec![];

    // Create 8 tasks using tokio
    for i in 1..=8 {
        let task = tokio::spawn(async move {
            let now = chrono::Local::now();
            write_log!(format!("task_{i}.log"), || {
                tracing::info!("This will be logged to stdout");
            });
            println!(
                "ex first = {}",
                Local::now().signed_duration_since(now).num_milliseconds()
            );

            let mut count = 0;
            loop {
                count += 1;
                let now = chrono::Local::now();

                write_log!(format!("task_{i}.log"), || {
                    tracing::info!("task_{i} count: {count}");
                });
                println!(
                    "ex_{count} = {}ms",
                    Local::now().signed_duration_since(now).num_milliseconds()
                );

                // println!("Task {i} count: {count}");
                time::sleep(Duration::from_secs(1)).await;
            }
        });
        tasks.push(task);
    }

    // Await all tasks
    for task in tasks {
        task.await.unwrap();
    }
}
