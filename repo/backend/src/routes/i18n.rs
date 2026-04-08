use rocket::{get, routes};
use rocket::serde::json::Json;
use rocket::http::Status;
use std::collections::HashMap;

use shared::dto::ApiResponse;
use shared::i18n::init_translations;

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Serialize)]
pub struct LocaleInfo {
    pub code: String,
    pub name: String,
}

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

#[get("/translations/<locale>")]
pub async fn get_translations(
    locale: String,
) -> Result<Json<ApiResponse<HashMap<String, String>>>, (Status, Json<ApiResponse<()>>)> {
    let translations = init_translations();

    let map = translations
        .map
        .get(&locale)
        .cloned()
        .ok_or_else(|| {
            (
                Status::NotFound,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(format!("Locale '{}' not found", locale)),
                }),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(map),
        error: None,
    }))
}

#[get("/locales")]
pub async fn get_locales() -> Json<ApiResponse<Vec<LocaleInfo>>> {
    let locales = vec![
        LocaleInfo {
            code: "en".into(),
            name: "English".into(),
        },
        LocaleInfo {
            code: "zh".into(),
            name: "Chinese".into(),
        },
    ];

    Json(ApiResponse {
        success: true,
        data: Some(locales),
        error: None,
    })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_translations, get_locales]
}
