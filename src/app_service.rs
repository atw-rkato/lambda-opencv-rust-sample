use std::ffi::OsStr;
use std::path::Path;

use anyhow::{Context, Result};
use aws_types::SdkConfig;

use crate::drawer;
use crate::s3_service::S3Service;

pub struct AppServiceProps {
    pub bucket_name: String,
    pub object_key: String,
}

pub struct AppService {
    s3_service: S3Service,
}

impl AppService {
    pub fn new(shared_config: SdkConfig) -> Self {
        Self {
            s3_service: S3Service::new(&shared_config),
        }
    }

    pub async fn run(
        &self,
        AppServiceProps {
            bucket_name,
            object_key,
        }: AppServiceProps,
    ) -> Result<()> {
        let uploaded_file = self
            .s3_service
            .get_object(&bucket_name, &object_key)
            .await?;

        let ext = Path::new(&object_key)
            .extension()
            .and_then(OsStr::to_str)
            .context("file extension not found.")?;

        log::info!("draw contours start.");
        let img = drawer::draw_contours(uploaded_file.as_ref(), ext)?;
        log::info!("draw contours end.");

        self.s3_service
            .upload_image(format!("{bucket_name}-output"), &object_key, img, ext)
            .await?;

        Ok(())
    }
}
