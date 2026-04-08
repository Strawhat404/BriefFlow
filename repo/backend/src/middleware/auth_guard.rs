use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use sqlx::MySqlPool;

use crate::services::auth::Claims;
use crate::services::session::SessionConfig;

// ---------------------------------------------------------------------------
// AuthenticatedUser -- any logged-in user
// ---------------------------------------------------------------------------

/// Request guard that requires a valid HMAC-signed session cookie.
///
/// This is the sole authentication path — there is no JWT bearer fallback.
/// Every authenticated request must present a `brewflow_session` cookie whose
/// signature is valid, whose DB session exists, and whose `last_activity` is
/// within the 30-minute idle timeout.
pub struct AuthenticatedUser {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match try_cookie_auth(request).await {
            Some(user) => Outcome::Success(user),
            None => Outcome::Error((Status::Unauthorized, "Valid session cookie required")),
        }
    }
}

// ---------------------------------------------------------------------------
// Cookie-based auth helper
// ---------------------------------------------------------------------------

/// Attempt to authenticate via the `brewflow_session` cookie.
///
/// 1. Read the cookie value
/// 2. Verify the HMAC signature
/// 3. Look up the session in MySQL
/// 4. Check idle timeout
/// 5. Touch session (update last_activity)
/// 6. Rotate session ID if needed
/// 7. Load user roles and build Claims
async fn try_cookie_auth(request: &Request<'_>) -> Option<AuthenticatedUser> {
    // Check for the cookie FIRST — no DB or state lookup needed for this step.
    // Returning None here produces 401 without touching the database, which also
    // makes unauthenticated-request tests work without a live DB pool.
    let cookie = request.cookies().get("brewflow_session")?;

    let session_config = request.rocket().state::<SessionConfig>()?;
    let pool = request.rocket().state::<MySqlPool>()?;

    let session_id = crate::services::session::verify_cookie(session_config, cookie.value())?;

    let session = crate::db::sessions::get_session(pool, &session_id).await?;

    // Check idle timeout
    let now = chrono::Utc::now().naive_utc();
    let idle_secs = now
        .signed_duration_since(session.last_activity)
        .num_seconds();
    if idle_secs < 0 || idle_secs as u64 > session_config.idle_timeout_secs {
        // Session expired -- clean it up silently.
        let _ = crate::db::sessions::delete_session(pool, &session.session_id).await;
        return None;
    }

    // Touch session (renews last_activity for idle-timeout tracking)
    let _ = crate::db::sessions::touch_session(pool, &session.session_id).await;

    // Rotate session ID if the rotation interval has elapsed.
    // A new session_id is inserted into the DB and a fresh signed cookie is
    // set on the current request so the client receives it in the response.
    let active_session_id = if crate::services::session::should_rotate(session.rotated_at, session_config) {
        let new_id = crate::services::session::create_session_id();
        if crate::db::sessions::rotate_session(pool, &session.session_id, &new_id)
            .await
            .is_ok()
        {
            let signed = crate::services::session::sign_cookie(session_config, &new_id);
            let new_cookie = rocket::http::Cookie::build(("brewflow_session", signed))
                .http_only(true)
                .same_site(rocket::http::SameSite::Strict)
                .secure(true)
                .path("/")
                .max_age(rocket::time::Duration::seconds(
                    session_config.idle_timeout_secs as i64,
                ));
            request.cookies().add(new_cookie);
            new_id
        } else {
            session.session_id.clone()
        }
    } else {
        session.session_id.clone()
    };
    let _ = active_session_id; // used only for the cookie set above

    // Load user roles
    let roles = crate::db::users::get_user_roles(pool, session.user_id).await;
    let user = crate::db::users::find_by_id(pool, session.user_id).await?;

    let claims = Claims {
        sub: session.user_id,
        username: user.username,
        roles,
        exp: 0, // Not applicable for session-based auth
    };

    Some(AuthenticatedUser { claims })
}

// ---------------------------------------------------------------------------
// StaffGuard -- requires Staff or Admin role
// ---------------------------------------------------------------------------

pub struct StaffGuard {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StaffGuard {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = match AuthenticatedUser::from_request(request).await {
            Outcome::Success(u) => u,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        if user.claims.roles.iter().any(|r| r == "Staff" || r == "Admin") {
            Outcome::Success(StaffGuard { claims: user.claims })
        } else {
            Outcome::Error((Status::Forbidden, "Staff or Admin role required"))
        }
    }
}

// ---------------------------------------------------------------------------
// AdminGuard -- requires Admin role
// ---------------------------------------------------------------------------

pub struct AdminGuard {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = match AuthenticatedUser::from_request(request).await {
            Outcome::Success(u) => u,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        if user.claims.roles.iter().any(|r| r == "Admin") {
            Outcome::Success(AdminGuard { claims: user.claims })
        } else {
            Outcome::Error((Status::Forbidden, "Admin role required"))
        }
    }
}

// ---------------------------------------------------------------------------
// TeacherGuard -- requires Teacher, AcademicAffairs, or Admin role
// ---------------------------------------------------------------------------

pub struct TeacherGuard {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TeacherGuard {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = match AuthenticatedUser::from_request(request).await {
            Outcome::Success(u) => u,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        if user.claims.roles.iter().any(|r| {
            r == "Teacher" || r == "AcademicAffairs" || r == "Admin"
        }) {
            Outcome::Success(TeacherGuard { claims: user.claims })
        } else {
            Outcome::Error((Status::Forbidden, "Teacher, AcademicAffairs, or Admin role required"))
        }
    }
}

