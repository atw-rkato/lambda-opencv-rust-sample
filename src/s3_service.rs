use anyhow::Result;
use aws_sdk_s3::types::ByteStream;
use aws_types::SdkConfig;
use bytes::Bytes;

pub struct S3Service {
    client: aws_sdk_s3::Client,
}

impl S3Service {
    pub fn new(shared_config: &SdkConfig) -> Self {
        let client = aws_sdk_s3::Client::new(shared_config);
        Self { client }
    }

    pub async fn get_object(
        &self,
        bucket_name: impl Into<String>,
        object_key: impl Into<String>,
    ) -> Result<Bytes> {
        let get_object_output = self
            .client
            .get_object()
            .bucket(bucket_name)
            .key(object_key)
            .send()
            .await?;

        log::info!("{:?}", get_object_output);

        let bytes = get_object_output.body.collect().await?;
        Ok(bytes.into_bytes())
    }

    pub async fn upload_image<T>(
        &self,
        bucket_name: impl Into<String>,
        object_key: impl Into<String>,
        body: T,
        ext: &str,
    ) -> Result<()>
    where
        ByteStream: From<T>,
    {
        let put_object_output = self
            .client
            .put_object()
            .bucket(bucket_name)
            .key(object_key)
            .content_type(format!("image/{ext}"))
            .body(ByteStream::from(body))
            .send()
            .await?;

        log::info!("{:?}", put_object_output);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{error, fs};

    use crate::drawer;
    use crate::s3_service::S3Service;

    #[tokio::test]
    #[ignore]
    async fn put_object() -> Result<(), Box<dyn error::Error>> {
        let shared_config = aws_config::load_from_env().await;
        let s3_service = S3Service::new(&shared_config);

        let img = fs::read("img/jellyfish.jpg")?;
        let img_contours = drawer::draw_contours(&img, "jpg")?;
        s3_service
            .upload_image(
                "lambda-opencv-rust-input",
                "jellyfish-contours.jpg",
                img_contours,
                "jpg",
            )
            .await?;
        Ok(())
    }
}
