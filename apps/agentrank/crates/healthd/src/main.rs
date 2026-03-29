//! Exit 0 if PostgreSQL and Redis are reachable; non-zero otherwise.

use agentrank_data_plane::{check_postgres, check_redis, database_url, pg_pool, redis_url};
use std::process::ExitCode;
use tracing::{error, info};

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let database_url = match database_url() {
        Ok(u) => u,
        Err(e) => {
            error!(error = %e, "DATABASE_URL is required");
            return ExitCode::from(1);
        }
    };

    let redis_url = match redis_url() {
        Ok(u) => u,
        Err(e) => {
            error!(error = %e, "REDIS_URL is required");
            return ExitCode::from(1);
        }
    };

    let pool = match pg_pool(&database_url).await {
        Ok(p) => p,
        Err(e) => {
            error!(error = %e, "failed to connect to PostgreSQL");
            return ExitCode::from(1);
        }
    };

    if let Err(e) = check_postgres(&pool).await {
        error!(error = %e, "PostgreSQL health check failed");
        return ExitCode::from(1);
    }

    if let Err(e) = check_redis(&redis_url).await {
        error!(error = %e, "Redis health check failed");
        return ExitCode::from(1);
    }

    if let Ok(u) = std::env::var("QDRANT_URL") {
        if !u.trim().is_empty() {
            match agentrank_vector::connect().await {
                Ok(client) => {
                    if let Err(e) = agentrank_vector::health_check(&client).await {
                        error!(error = %e, "Qdrant health check failed");
                        return ExitCode::from(1);
                    }
                }
                Err(e) => {
                    error!(error = %e, "QDRANT_URL set but Qdrant unreachable");
                    return ExitCode::from(1);
                }
            }
        }
    }

    info!("postgres and redis healthy (and qdrant if QDRANT_URL set)");
    ExitCode::SUCCESS
}
