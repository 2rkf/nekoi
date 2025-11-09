use crate::{ApiResponse, app_state::AppState};
use axum::{extract::State, http::StatusCode, Json};
use redis::AsyncCommands;

/// Health check endpoint that checks MySQL and Redis connectivity.
pub async fn ping(
    State(state): State<AppState>
) -> Result<Json<ApiResponse>, (StatusCode, Json<ApiResponse>)> {
    let mysql_status = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .map(|_| true)
        .unwrap_or(false);

    let mut redis_conn = state.redis_client.get_multiplexed_async_connection().await;
    let redis_status = if let Ok(ref mut conn) = redis_conn {
        let pong: Result<String, _> = conn.ping().await;
        pong.map(|_| true).unwrap_or(false)
    } else {
        false
    };

    if mysql_status && redis_status {
        Ok(Json(ApiResponse {
            id: None,
            message: Some("Health check passed: All services operational".into()),
            status: StatusCode::OK.as_u16(),
            success: true,
            url: None,
        }))
    } else {
        let msg = if !mysql_status && !redis_status {
            "Health check failed: MySQL and Redis connections unavailable"
        } else if !mysql_status {
            "Health check failed: MySQL connection unavailable"
        } else {
            "Health check failed: Redis connection unavailable"
        }.to_string();

        Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ApiResponse {
                id: None,
                message: Some(msg),
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                success: false,
                url: None,
            }),
        ))
    }
}
