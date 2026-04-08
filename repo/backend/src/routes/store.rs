use rocket::{get, routes};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use sqlx::MySqlPool;

use shared::dto::ApiResponse;
use shared::models::{StoreHours, SalesTaxConfig};
use crate::middleware::auth_guard::AuthenticatedUser;

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

#[get("/hours")]
pub async fn get_store_hours(
    pool: &State<MySqlPool>,
) -> Json<ApiResponse<Vec<StoreHours>>> {
    let hours = crate::db::store::get_store_hours(pool.inner()).await;

    Json(ApiResponse {
        success: true,
        data: Some(hours),
        error: None,
    })
}

#[get("/pickup-slots?<date>&<prep_time>")]
pub async fn get_pickup_slots(
    pool: &State<MySqlPool>,
    user: Option<AuthenticatedUser>,
    date: String,
    prep_time: Option<i32>,
) -> Result<Json<ApiResponse<Vec<shared::dto::PickupSlot>>>, (Status, Json<ApiResponse<()>>)> {
    // Derive prep time from the authenticated user's cart items (max across all SPUs).
    // Fall back to the explicit query param, then a safe default of 15 minutes.
    let prep_minutes = if let Some(ref u) = user {
        let cart_id = crate::db::cart::get_or_create_cart(pool.inner(), u.claims.sub)
            .await
            .ok();
        if let Some(cid) = cart_id {
            let max = crate::db::cart::get_max_prep_time_for_cart(pool.inner(), cid).await;
            max.unwrap_or_else(|| prep_time.unwrap_or(15))
        } else {
            prep_time.unwrap_or(15)
        }
    } else {
        prep_time.unwrap_or(15)
    };

    let parsed_date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| {
            (
                Status::BadRequest,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Invalid date format, expected YYYY-MM-DD".into()),
                }),
            )
        })?;

    let store_hours = crate::db::store::get_store_hours(pool.inner()).await;

    let existing_reservations = crate::db::store::get_reservations_for_date(pool.inner(), parsed_date).await;

    let slots = crate::services::pickup::generate_pickup_slots(
        &store_hours,
        parsed_date,
        prep_minutes,
        &existing_reservations,
    );

    Ok(Json(ApiResponse {
        success: true,
        data: Some(slots),
        error: None,
    }))
}

#[get("/tax")]
pub async fn get_tax(
    pool: &State<MySqlPool>,
) -> Result<Json<ApiResponse<SalesTaxConfig>>, (Status, Json<ApiResponse<()>>)> {
    let config = crate::db::store::get_tax_config(pool.inner())
        .await
        .ok_or_else(|| {
            (
                Status::NotFound,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("No active tax configuration found".into()),
                }),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(config),
        error: None,
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_store_hours, get_pickup_slots, get_tax]
}
