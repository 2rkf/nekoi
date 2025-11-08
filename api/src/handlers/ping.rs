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

    let mut redis_conn = state.redis_client.get_connection().unwrap();
    let redis_status = match redis_conn.ping() {
        Ok(_) => true,
        Err(_) => false,
    };

    if mysql_status && redis_status {
        Ok(Json(ApiResponse {
            id: None,
            message: Some("Pong! MySQL and Redis are healthy.".into()),
            status: StatusCode::OK.as_u16(),
            success: true,
            url: None,
        }))
    } else {
        let msg = format!(
            "Pong, but issues detected: MySQL={}, Redis={}",
            mysql_status, redis_status
        );
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
