use std::process;

use anyhow::{bail, Context, Result};
use derive_new::new;
use lambda_runtime::{service_fn, LambdaEvent};
use percent_encoding::percent_decode_str;
use serde_json::Value;

use crate::app_service::{AppService, AppServiceProps};

mod app_service;
mod drawer;
mod s3_service;

#[derive(new)]
struct App {
    service: AppService,
}

impl App {
    async fn lambda_handler(&self, event: Value, _: lambda_runtime::Context) -> Result<()> {
        let s3 = &event["Records"][0]["s3"];
        let bucket_name = s3["bucket"]["name"]
            .as_str()
            .context("bucket name doesn't exist.")?;
        let object_key = match s3["object"]["key"].as_str() {
            Some(key) => percent_decode_str(key).decode_utf8()?,
            None => bail!("object key doesn't exist."),
        };

        let props = AppServiceProps {
            bucket_name: bucket_name.to_string(),
            object_key: object_key.to_string(),
        };

        let result = self.service.run(props).await?;
        Ok(result)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_thread_names(true)
        .json()
        .flatten_event(true)
        .init();

    let app: App = {
        let shared_config = aws_config::load_from_env().await;
        let service = AppService::new(shared_config);
        App::new(service)
    };

    let func = service_fn(|req: LambdaEvent<Value>| app.lambda_handler(req.payload, req.context));
    if let Err(err) = lambda_runtime::run(func).await {
        log::error!("{:?}", err);
        process::exit(1);
    }
}
