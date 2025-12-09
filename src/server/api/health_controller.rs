use axum::http::StatusCode;
use axum::Extension;
use axum::Json;
use chrono::Utc;
use tracing::{error, warn};

use crate::server::dtos::health_dto::{
    DatabaseHealth, HealthResponse, HealthStatus, RedisHealth, ServiceHealthDetails,
};
use crate::server::services::Services;
use crate::server::{get_app_version, get_uptime_seconds};

pub async fn health_endpoint(
    Extension(services): Extension<Services>,
) -> (StatusCode, Json<HealthResponse>) {
    // Perform health checks with timing
    let db_health = check_database_health(&services).await;
    let redis_health = check_redis_health(&services).await;

    // Determine overall status
    let overall_status = determine_overall_status(&db_health, &redis_health);

    // Build response
    let response = HealthResponse {
        status: overall_status,
        timestamp: Utc::now(),
        uptime_seconds: get_uptime_seconds(),
        version: get_app_version().to_string(),
        environment: format!("{:?}", services.config.cargo_env).to_lowercase(),
        services: ServiceHealthDetails {
            database: db_health,
            redis: redis_health,
        },
    };

    // Determine HTTP status code
    let http_status = match overall_status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK, // Still operational
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    (http_status, Json(response))
}

async fn check_database_health(services: &Services) -> DatabaseHealth {
    match services.database.health_check().await {
        Ok(response_time) => {
            let (pool_active, pool_max) = services.database.pool_stats();

            DatabaseHealth {
                status: HealthStatus::Healthy,
                response_time_ms: response_time,
                pool_active,
                pool_max,
            }
        }
        Err(e) => {
            error!("Database health check failed: {}", e);
            DatabaseHealth {
                status: HealthStatus::Unhealthy,
                response_time_ms: 0.0,
                pool_active: 0,
                pool_max: 5,
            }
        }
    }
}

async fn check_redis_health(services: &Services) -> RedisHealth {
    match services.redis.health_check().await {
        Ok(response_time) => RedisHealth {
            status: HealthStatus::Healthy,
            response_time_ms: response_time,
        },
        Err(e) => {
            error!("Redis health check failed: {}", e);
            RedisHealth {
                status: HealthStatus::Unhealthy,
                response_time_ms: 0.0,
            }
        }
    }
}

fn determine_overall_status(db: &DatabaseHealth, redis: &RedisHealth) -> HealthStatus {
    // Count unhealthy services
    let unhealthy_count = [&db.status, &redis.status]
        .iter()
        .filter(|&&s| *s == HealthStatus::Unhealthy)
        .count();

    match unhealthy_count {
        0 => HealthStatus::Healthy,
        1 => {
            // One service down - degraded but operational
            warn!("System in degraded state: one service unhealthy");
            HealthStatus::Degraded
        }
        _ => {
            // Both critical services down
            error!("System unhealthy: multiple services down");
            HealthStatus::Unhealthy
        }
    }
}
