use std::process;

use anyhow::Result;
use lambda_runtime::{handler_fn, Context};
use serde_json::{json, Value};

fn init_logger() {
    use std::io::Write;

    env_logger::builder()
        .format(|buf, record| {
            let ts = buf.timestamp_millis();
            writeln!(
                buf,
                "{ts} {level} [{target}] {args} ({file}:{line})",
                ts = ts,
                level = record.level(),
                target = record.target(),
                args = record.args(),
                file = record.file().unwrap_or("unknown"),
                line = record.line().unwrap_or(0),
            )
        })
        .init();
}

async fn lambda_handler(_: Value, _: Context) -> Result<Value> {
    lambda_opencv_rust_sample::run()?;
    Ok(json!({"key": "value"}))
}

#[tokio::main]
async fn main() {
    init_logger();
    let func = handler_fn(lambda_handler);
    if let Err(err) = lambda_runtime::run(func).await {
        log::error!("{:?}", err);
        process::exit(1);
    }
}
