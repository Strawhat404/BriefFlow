use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    pub auth: AuthState,
    pub locale: String,
    pub cart_count: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthState {
    /// Signed `brewflow_session` cookie value returned by the login endpoint.
    /// Attached as `Cookie: brewflow_session=<value>` on every API request.
    pub session_cookie: Option<String>,
    pub user: Option<UserInfo>,
    pub is_authenticated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub roles: Vec<String>,
    pub preferred_locale: String,
}

impl AppState {
    pub fn current_locale(&self) -> &str {
        if self.locale.is_empty() {
            "en"
        } else {
            &self.locale
        }
    }

    pub fn is_staff(&self) -> bool {
        self.auth
            .user
            .as_ref()
            .map(|u| u.roles.iter().any(|r| r == "Staff" || r == "Admin"))
            .unwrap_or(false)
    }

    pub fn is_admin(&self) -> bool {
        self.auth
            .user
            .as_ref()
            .map(|u| u.roles.iter().any(|r| r == "Admin"))
            .unwrap_or(false)
    }

    pub fn is_teacher(&self) -> bool {
        self.auth
            .user
            .as_ref()
            .map(|u| {
                u.roles
                    .iter()
                    .any(|r| r == "Teacher" || r == "AcademicAffairs" || r == "Admin")
            })
            .unwrap_or(false)
    }

    pub fn set_auth(&mut self, session_cookie: String, user: UserInfo) {
        self.locale = user.preferred_locale.clone();
        self.auth = AuthState {
            session_cookie: Some(session_cookie),
            user: Some(user),
            is_authenticated: true,
        };
    }

    pub fn logout(&mut self) {
        self.auth = AuthState::default();
    }
}
