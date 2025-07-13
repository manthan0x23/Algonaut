use std::time::Duration;

use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_s3::{
    Client,
    config::{Credentials, Region, SharedCredentialsProvider},
    error::SdkError,
    presigning::PresigningConfig,
    primitives::ByteStream,
};

#[derive(Debug, Clone)]
pub struct AwsS3 {
    client: Client,
    bucket: String,
    cdn_base_url: String,
}

impl AwsS3 {
    pub async fn new(
        region: String,
        access_key: String,
        secret_key: String,
        bucket: String,
        cdn_base_url: String,
    ) -> Result<Self, aws_sdk_s3::Error> {
        let credentials = Credentials::new(access_key, secret_key, None, None, "static");

        let credentials_provider = SharedCredentialsProvider::new(credentials);

        // ✅ FIX: Provide BehaviorVersion when building config
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(region))
            .credentials_provider(credentials_provider)
            .load()
            .await;

        let client = Client::new(&config);

        Ok(Self {
            client,
            bucket,
            cdn_base_url,
        })
    }

    pub async fn upload(
        &self,
        key: String,
        data: Vec<u8>,
        content_type: String,
    ) -> Result<(), SdkError<aws_sdk_s3::operation::put_object::PutObjectError>> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await?;

        Ok(())
    }

    pub async fn generate_presigned_url(
        &self,
        key: String,
        expiry_secs: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let presigned = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(
                PresigningConfig::builder()
                    .expires_in(Duration::from_secs(expiry_secs))
                    .build()?,
            )
            .await?;

        Ok(presigned.uri().to_string())
    }

    pub fn get_cdn_url(&self, key: String) -> String {
        format!("{}/{}", self.cdn_base_url.trim_end_matches('/'), key)
    }
}
