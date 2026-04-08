use sqlx::MySqlPool;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HealthReport {
    pub status: String,
    pub timestamp: String,
    pub uptime_secs: u64,
    pub database: ComponentHealth,
    pub services: Vec<ServiceHealthReport>,
    pub background_jobs: Vec<JobStatus>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ComponentHealth {
    pub status: String,
    pub latency_ms: Option<u64>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceHealthReport {
    pub name: String,
    pub status: String,
    pub is_critical: bool,
    pub is_degraded: bool,
    pub circuit_state: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct JobStatus {
    pub name: String,
    pub last_run: Option<String>,
    pub next_run: Option<String>,
    pub is_enabled: bool,
    pub last_error: Option<String>,
}

/// Check database connectivity and measure latency.
pub async fn check_database(pool: &MySqlPool) -> ComponentHealth {
    let start = std::time::Instant::now();
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => ComponentHealth {
            status: "healthy".into(),
            latency_ms: Some(start.elapsed().as_millis() as u64),
            details: None,
        },
        Err(e) => ComponentHealth {
            status: "unhealthy".into(),
            latency_ms: None,
            details: Some(e.to_string()),
        },
    }
}

/// Run a full health check across database and managed services.
///
/// `jobs` is passed in from the caller (who holds a reference to the
/// `BackgroundJobManager`) rather than fetched through the `DegradationManager`
/// stub, which always returned an empty list.
pub async fn full_health_check(
    pool: &MySqlPool,
    degradation: &crate::services::resilience::DegradationManager,
    start_time: NaiveDateTime,
    jobs: Vec<JobStatus>,
) -> HealthReport {
    let now = chrono::Utc::now().naive_utc();
    let uptime = now.signed_duration_since(start_time).num_seconds().max(0) as u64;

    let db = check_database(pool).await;

    let service_statuses = degradation.get_status().await;
    let services: Vec<ServiceHealthReport> = service_statuses
        .into_iter()
        .map(|(name, info)| ServiceHealthReport {
            name,
            status: if info.is_degraded { "degraded".into() } else { "healthy".into() },
            is_critical: info.is_critical,
            is_degraded: info.is_degraded,
            circuit_state: info.circuit_state,
        })
        .collect();

    // Overall status: unhealthy if DB down or any critical degraded
    let any_critical_degraded = services.iter().any(|s| s.is_critical && s.is_degraded);
    let any_degraded = services.iter().any(|s| s.is_degraded);

    let status = if db.status != "healthy" || any_critical_degraded {
        "unhealthy"
    } else if any_degraded {
        "degraded"
    } else {
        "healthy"
    };

    HealthReport {
        status: status.into(),
        timestamp: now.format("%Y-%m-%dT%H:%M:%S").to_string(),
        uptime_secs: uptime,
        database: db,
        services,
        background_jobs: jobs,
    }
}
