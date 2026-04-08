use rocket::{get, post, put, delete, routes};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use sqlx::MySqlPool;

use shared::dto::{ApiResponse, UserInfo};
use crate::middleware::auth_guard::AdminGuard;

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize)]
pub struct AssignRoleRequest {
    pub role: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateStoreHoursRequest {
    pub hours: Vec<StoreHourEntry>,
}

#[derive(Debug, serde::Deserialize)]
pub struct StoreHourEntry {
    pub day_of_week: u8,
    pub open_time: String,
    pub close_time: String,
    pub is_closed: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateTaxRequest {
    pub tax_name: String,
    pub rate: f64,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateProductRequest {
    pub name_en: String,
    pub name_zh: String,
    pub description_en: String,
    pub description_zh: String,
    pub category: Option<String>,
    pub image_url: Option<String>,
    pub base_price: f64,
    pub prep_time_minutes: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateProductRequest {
    pub name_en: Option<String>,
    pub name_zh: Option<String>,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub category: Option<String>,
    pub image_url: Option<String>,
    pub base_price: Option<f64>,
    pub prep_time_minutes: Option<i32>,
    pub is_active: Option<bool>,
}

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

#[get("/users")]
pub async fn list_users(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
) -> Json<ApiResponse<Vec<UserInfo>>> {
    let users = crate::db::users::list_all_users(pool.inner()).await;
    let mut user_infos = Vec::new();
    for u in users {
        let roles = crate::db::users::get_user_roles(pool.inner(), u.id).await;
        user_infos.push(UserInfo {
            id: u.id,
            username: u.username,
            display_name: u.display_name,
            roles,
            preferred_locale: u.preferred_locale,
        });
    }
    Json(ApiResponse {
        success: true,
        data: Some(user_infos),
        error: None,
    })
}

#[post("/users/<id>/roles", data = "<body>")]
pub async fn assign_role(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
    id: i64,
    body: Json<AssignRoleRequest>,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    crate::db::users::assign_role(pool.inner(), id, &body.role)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to assign role".into()),
                }),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        error: None,
    }))
}

#[delete("/users/<id>/roles/<role>")]
pub async fn remove_role(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
    id: i64,
    role: String,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    crate::db::users::remove_role(pool.inner(), id, &role)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to remove role".into()),
                }),
            )
        })?;
    Ok(Json(ApiResponse {
        success: true,
        data: None,
        error: None,
    }))
}

#[put("/store-hours", data = "<body>")]
pub async fn update_store_hours(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
    body: Json<UpdateStoreHoursRequest>,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    for entry in &body.hours {
        crate::db::store::update_store_hours(
            pool.inner(),
            entry.day_of_week,
            &entry.open_time,
            &entry.close_time,
            entry.is_closed,
        )
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to update store hours".into()),
                }),
            )
        })?;
    }
    Ok(Json(ApiResponse {
        success: true,
        data: None,
        error: None,
    }))
}

#[put("/tax", data = "<body>")]
pub async fn update_tax(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
    body: Json<UpdateTaxRequest>,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    crate::db::store::update_tax_config(pool.inner(), &body.tax_name, body.rate)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to update tax config".into()),
                }),
            )
        })?;
    Ok(Json(ApiResponse {
        success: true,
        data: None,
        error: None,
    }))
}

#[post("/products", data = "<body>")]
pub async fn create_product(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
    body: Json<CreateProductRequest>,
) -> Result<Json<ApiResponse<shared::dto::ProductListItem>>, (Status, Json<ApiResponse<()>>)> {
    let spu_id = crate::db::products::create_spu(
        pool.inner(),
        &body.name_en,
        &body.name_zh,
        Some(body.description_en.as_str()),
        Some(body.description_zh.as_str()),
        body.category.as_deref(),
        body.base_price,
        body.prep_time_minutes,
    )
    .await
    .map_err(|_| {
        (
            Status::InternalServerError,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Failed to create product".into()),
            }),
        )
    })?;

    let item = shared::dto::ProductListItem {
        spu_id,
        name_en: body.name_en.clone(),
        name_zh: body.name_zh.clone(),
        description_en: Some(body.description_en.clone()),
        description_zh: Some(body.description_zh.clone()),
        category: body.category.clone(),
        image_url: body.image_url.clone(),
        base_price: body.base_price,
        prep_time_minutes: body.prep_time_minutes,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(item),
        error: None,
    }))
}

#[put("/products/<id>", data = "<body>")]
pub async fn update_product(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
    id: i64,
    body: Json<UpdateProductRequest>,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    let existing = crate::db::products::get_spu(pool.inner(), id)
        .await
        .ok_or_else(|| {
            (
                Status::NotFound,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Product not found".into()),
                }),
            )
        })?;

    let name_en = body.name_en.as_deref().unwrap_or(&existing.name_en);
    let name_zh = body.name_zh.as_deref().unwrap_or(&existing.name_zh);
    let desc_en = body.description_en.as_deref().or(existing.description_en.as_deref());
    let desc_zh = body.description_zh.as_deref().or(existing.description_zh.as_deref());
    let category = body.category.as_deref().or(existing.category.as_deref());
    let base_price = body.base_price.unwrap_or(existing.base_price);
    let prep_time = body.prep_time_minutes.unwrap_or(existing.prep_time_minutes);

    crate::db::products::update_spu(
        pool.inner(), id, name_en, name_zh, desc_en, desc_zh, category, base_price, prep_time,
    )
    .await
    .map_err(|_| {
        (
            Status::InternalServerError,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Failed to update product".into()),
            }),
        )
    })?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        error: None,
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        list_users,
        assign_role,
        remove_role,
        update_store_hours,
        update_tax,
        create_product,
        update_product
    ]
}
