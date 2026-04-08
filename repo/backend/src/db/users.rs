use sqlx::{MySqlPool, Row};
use shared::models::User;

pub async fn find_by_username(pool: &MySqlPool, username: &str) -> Option<User> {
    let row = sqlx::query(
        "SELECT id, username, password_hash, display_name, email, preferred_locale, created_at, updated_at
         FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await
    .ok()?;

    row.map(|r| User {
        id: r.get("id"),
        username: r.get("username"),
        password_hash: r.get("password_hash"),
        display_name: r.get("display_name"),
        email: r.get("email"),
        preferred_locale: r.get("preferred_locale"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    })
}

pub async fn find_by_id(pool: &MySqlPool, id: i64) -> Option<User> {
    let row = sqlx::query(
        "SELECT id, username, password_hash, display_name, email, preferred_locale, created_at, updated_at
         FROM users WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .ok()?;

    row.map(|r| User {
        id: r.get("id"),
        username: r.get("username"),
        password_hash: r.get("password_hash"),
        display_name: r.get("display_name"),
        email: r.get("email"),
        preferred_locale: r.get("preferred_locale"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    })
}

pub async fn create_user(
    pool: &MySqlPool,
    username: &str,
    password_hash: &str,
    display_name: Option<&str>,
    email: Option<&str>,
    locale: &str,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash, display_name, email, preferred_locale)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(username)
    .bind(password_hash)
    .bind(display_name)
    .bind(email)
    .bind(locale)
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn get_user_roles(pool: &MySqlPool, user_id: i64) -> Vec<String> {
    let rows = sqlx::query(
        "SELECT r.name FROM user_roles ur
         JOIN roles r ON r.id = ur.role_id
         WHERE ur.user_id = ?"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.iter().map(|r| r.get("name")).collect()
}

pub async fn assign_role(pool: &MySqlPool, user_id: i64, name: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO user_roles (user_id, role_id)
         SELECT ?, id FROM roles WHERE name = ?
         ON DUPLICATE KEY UPDATE user_id = user_id"
    )
    .bind(user_id)
    .bind(name)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_locale(pool: &MySqlPool, user_id: i64, locale: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET preferred_locale = ? WHERE id = ?")
        .bind(locale)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn list_all_users(pool: &MySqlPool) -> Vec<shared::models::User> {
    sqlx::query("SELECT id, username, password_hash, display_name, email, preferred_locale, created_at, updated_at FROM users ORDER BY id")
        .fetch_all(pool)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| shared::models::User {
            id: r.get("id"),
            username: r.get("username"),
            password_hash: r.get("password_hash"),
            display_name: r.get("display_name"),
            email: r.get("email"),
            preferred_locale: r.get("preferred_locale"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        })
        .collect()
}

pub async fn remove_role(pool: &MySqlPool, user_id: i64, role_name: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE ur FROM user_roles ur JOIN roles r ON r.id = ur.role_id WHERE ur.user_id = ? AND r.name = ?")
        .bind(user_id)
        .bind(role_name)
        .execute(pool)
        .await?;
    Ok(())
}
