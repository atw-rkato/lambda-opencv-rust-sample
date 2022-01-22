use anyhow::Result;
use aws_sdk_s3::ByteStream;
use bytes::Bytes;

pub struct S3Service {
    client: aws_sdk_s3::Client,
}

impl S3Service {
    pub fn new(shared_config: &aws_config::Config) -> Self {
        let client = aws_sdk_s3::Client::new(&shared_config);
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

    pub async fn put_object(
        &self,
        bucket_name: impl Into<String>,
        object_key: impl Into<String>,
        body: Vec<u8>,
    ) -> Result<()> {
        let put_object_output = self
            .client
            .put_object()
            .bucket(bucket_name)
            .key(object_key)
            .body(ByteStream::from(body))
            .send()
            .await?;

        log::info!("{:?}", put_object_output);

        Ok(())
    }
}
