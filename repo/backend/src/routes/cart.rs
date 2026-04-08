use rocket::{get, post, put, delete, routes};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use sqlx::MySqlPool;

use shared::dto::{AddToCartRequest, ApiResponse, CartResponse, CartItemDetail};
use crate::middleware::auth_guard::AuthenticatedUser;

#[derive(Debug, serde::Deserialize)]
pub struct UpdateQuantityRequest {
    pub quantity: i32,
}

// ---------------------------------------------------------------------------
// Helper: verify a cart item belongs to the authenticated user's cart
// ---------------------------------------------------------------------------
async fn verify_cart_item_ownership(pool: &MySqlPool, item_id: i64, user_id: i64) -> bool {
    let cart_id = crate::db::cart::get_or_create_cart(pool, user_id).await.unwrap_or(-1);
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM cart_items WHERE id = ? AND cart_id = ?")
        .bind(item_id)
        .bind(cart_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0)
        > 0
}

// ---------------------------------------------------------------------------
// Helper: build a CartResponse from the DB cart items
// ---------------------------------------------------------------------------
async fn build_cart_response(pool: &MySqlPool, user_id: i64) -> Result<CartResponse, String> {
    let cart_id = crate::db::cart::get_or_create_cart(pool, user_id)
        .await
        .map_err(|e| format!("Failed to get cart: {}", e))?;

    let rows = crate::db::cart::get_cart_items(pool, cart_id).await;

    let tax_config = crate::db::store::get_tax_config(pool).await;
    let tax_rate = tax_config.map(|t| t.rate).unwrap_or(0.0);

    let items: Vec<CartItemDetail> = rows
        .iter()
        .map(|r| CartItemDetail {
            id: r.id,
            spu_name_en: r.spu_name_en.clone(),
            spu_name_zh: r.spu_name_zh.clone(),
            sku_code: Some(r.sku_code.clone()),
            options: r.option_labels.clone(),
            quantity: r.quantity,
            unit_price: r.unit_price,
            line_total: r.unit_price * r.quantity as f64,
        })
        .collect();

    let subtotal: f64 = items.iter().map(|i| i.line_total).sum();
    let tax_amount = crate::services::pricing::calculate_tax(subtotal, tax_rate);
    let total = crate::services::pricing::calculate_total(subtotal, tax_amount);

    Ok(CartResponse {
        items,
        subtotal,
        tax_rate,
        tax_amount,
        total,
    })
}

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

#[get("/")]
pub async fn get_cart(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
) -> Result<Json<ApiResponse<CartResponse>>, (Status, Json<ApiResponse<()>>)> {
    let cart = build_cart_response(pool.inner(), user.claims.sub)
        .await
        .map_err(|e| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(e),
                }),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(cart),
        error: None,
    }))
}

#[post("/add", data = "<body>")]
pub async fn add_to_cart(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
    body: Json<AddToCartRequest>,
) -> Result<Json<ApiResponse<CartResponse>>, (Status, Json<ApiResponse<()>>)> {
    let user_id = user.claims.sub;

    // Validate quantity
    if body.quantity < 1 {
        return Err((
            Status::UnprocessableEntity,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Quantity must be at least 1".into()),
            }),
        ));
    }

    // Look up the SPU to get base price
    let spu = crate::db::products::get_spu(pool.inner(), body.spu_id)
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

    // Validate that all submitted option_value_ids belong to this SPU.
    // This prevents a client from mixing options from different products.
    for ov_id in &body.selected_options {
        let ov = crate::db::products::get_option_value_by_id(pool.inner(), *ov_id).await;
        let belongs = match ov {
            Some(ref v) => {
                // Resolve the option_group → SPU linkage: option_group.spu_id must match
                crate::db::products::option_value_belongs_to_spu(pool.inner(), v.id, body.spu_id).await
            }
            None => false,
        };
        if !belongs {
            return Err((
                Status::UnprocessableEntity,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(format!("Option value {} does not belong to this product", ov_id)),
                }),
            ));
        }
    }

    // Enforce required option groups: every group with is_required=true must have
    // at least one of its option_value_ids present in the submitted selection.
    let option_groups = crate::db::products::get_option_groups(pool.inner(), body.spu_id).await;
    for group in option_groups.iter().filter(|g| g.is_required) {
        let group_values = crate::db::products::get_option_values(pool.inner(), group.id).await;
        let group_value_ids: std::collections::HashSet<i64> =
            group_values.into_iter().map(|v| v.id).collect();
        let has_selection = body.selected_options.iter().any(|id| group_value_ids.contains(id));
        if !has_selection {
            return Err((
                Status::UnprocessableEntity,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(format!(
                        "Required option group '{}' has no selection",
                        group.name_en
                    )),
                }),
            ));
        }
    }

    // Calculate unit price: base price + sum of option deltas
    let mut option_delta = 0.0_f64;
    for ov_id in &body.selected_options {
        if let Some(ov) = crate::db::products::get_option_value_by_id(pool.inner(), *ov_id).await {
            option_delta += ov.price_delta;
        }
    }

    // Try to find a matching SKU
    let sku = crate::db::products::get_sku_by_options(pool.inner(), body.spu_id, &body.selected_options).await;
    let unit_price = if let Some(ref s) = sku {
        s.price
    } else {
        spu.base_price + option_delta
    };

    let sku_id = match sku.map(|s| s.id).or(body.sku_id) {
        Some(id) if id > 0 => id,
        _ => return Err((
            Status::UnprocessableEntity,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("No valid SKU found for the given product and options".into()),
            }),
        )),
    };

    let cart_id = crate::db::cart::get_or_create_cart(pool.inner(), user_id)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to get or create cart".into()),
                }),
            )
        })?;

    let cart_item_id = crate::db::cart::add_item(pool.inner(), cart_id, sku_id, body.quantity, unit_price)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to add item to cart".into()),
                }),
            )
        })?;

    if !body.selected_options.is_empty() {
        let _ = crate::db::cart::add_item_options(pool.inner(), cart_item_id, &body.selected_options).await;
    }

    let cart = build_cart_response(pool.inner(), user_id)
        .await
        .map_err(|e| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(e),
                }),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(cart),
        error: None,
    }))
}

#[put("/<item_id>", data = "<body>")]
pub async fn update_item(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
    item_id: i64,
    body: Json<UpdateQuantityRequest>,
) -> Result<Json<ApiResponse<CartResponse>>, (Status, Json<ApiResponse<()>>)> {
    if body.quantity < 1 {
        return Err((
            Status::UnprocessableEntity,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Quantity must be at least 1".into()),
            }),
        ));
    }

    if !verify_cart_item_ownership(pool.inner(), item_id, user.claims.sub).await {
        return Err((
            Status::Forbidden,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Cart item does not belong to you".into()),
            }),
        ));
    }

    crate::db::cart::update_item_quantity(pool.inner(), item_id, body.quantity)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to update item".into()),
                }),
            )
        })?;

    let cart = build_cart_response(pool.inner(), user.claims.sub)
        .await
        .map_err(|e| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(e),
                }),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(cart),
        error: None,
    }))
}

// Rank 2 so that DELETE /clear is tried first (rank 1 via specificity)
#[delete("/<item_id>", rank = 2)]
pub async fn remove_item(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
    item_id: i64,
) -> Result<Json<ApiResponse<CartResponse>>, (Status, Json<ApiResponse<()>>)> {
    if !verify_cart_item_ownership(pool.inner(), item_id, user.claims.sub).await {
        return Err((
            Status::Forbidden,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Cart item does not belong to you".into()),
            }),
        ));
    }

    crate::db::cart::remove_item(pool.inner(), item_id)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to remove item".into()),
                }),
            )
        })?;

    let cart = build_cart_response(pool.inner(), user.claims.sub)
        .await
        .map_err(|e| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(e),
                }),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(cart),
        error: None,
    }))
}

#[delete("/clear")]
pub async fn clear_cart(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    let cart_id = crate::db::cart::get_or_create_cart(pool.inner(), user.claims.sub)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to get cart".into()),
                }),
            )
        })?;

    crate::db::cart::clear_cart(pool.inner(), cart_id)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to clear cart".into()),
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
    routes![get_cart, add_to_cart, update_item, remove_item, clear_cart]
}
