use chrono::NaiveDateTime;
use sqlx::{MySqlPool, Row};

/// A row from the `sessions` table.
#[derive(Debug, Clone)]
pub struct SessionRow {
    pub session_id: String,
    pub user_id: i64,
    pub last_activity: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub rotated_at: NaiveDateTime,
}

/// Insert a new session.
pub async fn create_session(
    pool: &MySqlPool,
    session_id: &str,
    user_id: i64,
    user_agent: Option<&str>,
    ip_addr: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO sessions (session_id, user_id, user_agent, ip_address, last_activity, created_at, rotated_at)
         VALUES (?, ?, ?, ?, NOW(), NOW(), NOW())",
    )
    .bind(session_id)
    .bind(user_id)
    .bind(user_agent)
    .bind(ip_addr)
    .execute(pool)
    .await?;

    Ok(())
}

/// Look up a session by its ID.
pub async fn get_session(pool: &MySqlPool, session_id: &str) -> Option<SessionRow> {
    let row = sqlx::query(
        "SELECT session_id, user_id, last_activity, created_at, rotated_at
         FROM sessions WHERE session_id = ?",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await
    .ok()?;

    row.map(|r| SessionRow {
        session_id: r.get("session_id"),
        user_id: r.get("user_id"),
        last_activity: r.get("last_activity"),
        created_at: r.get("created_at"),
        rotated_at: r.get("rotated_at"),
    })
}

/// Update `last_activity` to NOW() for the given session.
pub async fn touch_session(pool: &MySqlPool, session_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE sessions SET last_activity = NOW() WHERE session_id = ?")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Rotate a session: change its ID and update `rotated_at`.
pub async fn rotate_session(
    pool: &MySqlPool,
    old_session_id: &str,
    new_session_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE sessions SET session_id = ?, rotated_at = NOW() WHERE session_id = ?",
    )
    .bind(new_session_id)
    .bind(old_session_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a single session.
pub async fn delete_session(pool: &MySqlPool, session_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE session_id = ?")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Remove all sessions whose `last_activity` is older than `idle_timeout_secs`
/// seconds ago.  Returns the number of deleted rows.
pub async fn cleanup_expired_sessions(
    pool: &MySqlPool,
    idle_timeout_secs: u64,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM sessions WHERE last_activity < DATE_SUB(NOW(), INTERVAL ? SECOND)",
    )
    .bind(idle_timeout_secs as i64)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
