use redis::{Client as RedisClient, Commands};
use std::time::{SystemTime, UNIX_EPOCH};

/// A thread-safe rate limiter that tracks and enforces request quotas per day.
#[derive(Clone)]
pub struct RateLimiterStore {
    redis_client: RedisClient,
    request_per_day: u32,
}

/// Represents the status of a rate limit check, including allowance status and quota information.
pub struct RateLimitStatus {
    /// Whether the current request is allowed under the rate limit.
    pub is_allowed: bool,
    /// Seconds until the next allowed request (only present when rate limited).
    pub retry_after: Option<u64>,
    /// The total request limit per time window.
    pub limit: u32,
    /// Remaining requests in the current time window.
    pub remaining: u32,
    /// Seconds until the current rate limit window resets.
    pub reset_after: u64,
}

impl RateLimiterStore {
    /// Creates a new `RateLimiterStore` with the specified requests per day limit.
    pub fn new(redis_client: RedisClient, request_per_day: u32) -> Self {
        Self {
            redis_client,
            request_per_day,
        }
    }

    /// Checks if a request is allowed under the rate limit, updating the usage count.
    pub fn check(&self, user: String, extend: bool) -> RateLimitStatus {
        let mut conn = self.redis_client.get_connection().unwrap();
        let limit = if extend {
            self.request_per_day * 10
        } else {
            self.request_per_day
        };

        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let day_start = now_secs / 86400 * 86400;
        let ttl = 86400 - (now_secs - day_start);
        let redis_key = format!("ratelimit:{}:{}", user, day_start);
        let usage: u32 = conn.incr(&redis_key, 1).unwrap_or(1);

        if usage == 1 {
            let _: () = conn.expire(&redis_key, ttl as i64).unwrap();
        }

        if usage <= limit {
            RateLimitStatus {
                is_allowed: true,
                retry_after: None,
                limit,
                remaining: limit - usage,
                reset_after: ttl,
            }
        } else {
            RateLimitStatus {
                is_allowed: false,
                retry_after: Some(ttl),
                limit,
                remaining: 0,
                reset_after: ttl,
            }
        }
    }
}
