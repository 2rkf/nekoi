use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use bytes::Bytes;
use moka::future::Cache;
use sqlx::MySqlPool;
use std::env;
use std::sync::Arc;
use redis::Client as RedisClient;

use crate::middlewares::rate_limit::RateLimiterStore;
use crate::services::image_service::ImageService;

/// Shared application state containing database pool, cache, and services.
#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub redis_client: RedisClient,
    pub cache: Arc<Cache<String, Bytes>>,
    pub image_service: Arc<ImageService>,
    pub rate_limiter: RateLimiterStore,
    pub s3_bucket: String,
    pub s3_client: Arc<Client>,
}

/// Creates a new shared AppState.
pub async fn create_state(
    pool: MySqlPool,
    redis_client: RedisClient,
    base_url: String,
    bucket: String,
    access_key_id: String,
    secret_access_key: String,
) -> Result<AppState> {
    let endpoint = env::var("AWS_ENDPOINT").expect("Missing 'AWS_ENDPOINT'");
    let region_str = env::var("AWS_REGION").expect("Missing 'AWS_REGION'");
    let region = aws_config::Region::new(region_str);

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region.clone())
        .endpoint_url(endpoint)
        .credentials_provider(Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "nekoi",
        ))
        .load()
        .await;
    let s3_client = Arc::new(Client::new(&config));

    let image_service = Arc::new(ImageService::new(
        base_url,
        s3_client.clone(),
        bucket.clone(),
    ));

    let cache = Arc::new(
        Cache::builder()
            .max_capacity(10000)
            .time_to_live(std::time::Duration::from_secs(3600))
            .build(),
    );

    let rate_limiter = RateLimiterStore::new(redis_client.clone(), 1000);

    Ok(AppState {
        pool,
        redis_client,
        cache,
        image_service,
        rate_limiter,
        s3_bucket: bucket.clone(),
        s3_client,
    })
}
