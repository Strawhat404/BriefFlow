use rocket::{get, post, put, routes};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use sqlx::MySqlPool;

use shared::dto::{
    ApiResponse, OrderDetail, OrderSummary, OrderItemDetail,
    FulfillmentEventDetail, ReservationDetail,
    ScanVoucherRequest, ScanVoucherResponse,
    UpdateOrderStatusRequest,
};
use crate::middleware::auth_guard::StaffGuard;

#[derive(Debug, serde::Serialize)]
pub struct DashboardStats {
    pub pending_count: i64,
    pub in_prep_count: i64,
    pub ready_count: i64,
}

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

#[get("/orders?<status>")]
pub async fn list_all_orders(
    pool: &State<MySqlPool>,
    _staff: StaffGuard,
    status: Option<String>,
) -> Json<ApiResponse<Vec<OrderSummary>>> {
    let orders = crate::db::orders::list_all_orders(pool.inner(), status.as_deref()).await;
    let summaries: Vec<OrderSummary> = orders
        .into_iter()
        .map(|o| OrderSummary {
            id: o.id,
            order_number: o.order_number,
            status: o.status,
            total: o.total,
            voucher_code: None,
            created_at: o.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            pickup_slot: None,
        })
        .collect();
    Json(ApiResponse {
        success: true,
        data: Some(summaries),
        error: None,
    })
}

#[get("/orders/<id>")]
pub async fn get_order(
    pool: &State<MySqlPool>,
    _staff: StaffGuard,
    id: i64,
) -> Result<Json<ApiResponse<OrderDetail>>, (Status, Json<ApiResponse<()>>)> {
    let order = crate::db::orders::get_order(pool.inner(), id)
        .await
        .ok_or_else(|| {
            (
                Status::NotFound,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Order not found".into()),
                }),
            )
        })?;

    let items_rows = crate::db::orders::get_order_items(pool.inner(), order.id).await;
    let items: Vec<OrderItemDetail> = items_rows
        .into_iter()
        .map(|i| OrderItemDetail {
            sku_code: i.sku_code,
            spu_name: i.spu_name,
            options: i.options,
            quantity: i.quantity,
            unit_price: i.unit_price,
            item_total: i.item_total,
        })
        .collect();

    let events = crate::db::orders::get_fulfillment_events(pool.inner(), order.id).await;
    let fulfillment_history: Vec<FulfillmentEventDetail> = events
        .into_iter()
        .map(|e| FulfillmentEventDetail {
            from_status: e.from_status,
            to_status: e.to_status,
            changed_by: e.changed_by_user_id.to_string(),
            notes: e.notes,
            timestamp: e.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        })
        .collect();

    let reservation = if let Some(res_id) = order.reservation_id {
        crate::db::store::get_reservation(pool.inner(), res_id).await.map(|r| {
            ReservationDetail {
                voucher_code: r.voucher_code,
                pickup_slot_start: r.pickup_slot_start.format("%Y-%m-%dT%H:%M:%S").to_string(),
                pickup_slot_end: r.pickup_slot_end.format("%Y-%m-%dT%H:%M:%S").to_string(),
                hold_expires_at: r.hold_expires_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
                status: r.status,
            }
        })
    } else {
        None
    };

    let summary = OrderSummary {
        id: order.id,
        order_number: order.order_number,
        status: order.status,
        total: order.total,
        voucher_code: None,
        created_at: order.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        pickup_slot: None,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(OrderDetail {
            order: summary,
            items,
            fulfillment_history,
            reservation,
        }),
        error: None,
    }))
}

#[put("/orders/<id>/status", data = "<body>")]
pub async fn update_order_status(
    pool: &State<MySqlPool>,
    staff: StaffGuard,
    id: i64,
    body: Json<UpdateOrderStatusRequest>,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    // Get current order status
    let order = crate::db::orders::get_order(pool.inner(), id)
        .await
        .ok_or_else(|| {
            (
                Status::NotFound,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Order not found".into()),
                }),
            )
        })?;

    // Validate transition (pass staff roles for cancel-after-ready check)
    if !crate::services::fulfillment::validate_transition(&order.status, &body.new_status, &staff.claims.roles) {
        return Err((
            Status::BadRequest,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!(
                    "Invalid status transition from '{}' to '{}'",
                    order.status, body.new_status
                )),
            }),
        ));
    }

    // Create fulfillment event
    let _ = crate::db::orders::create_fulfillment_event(
        pool.inner(),
        id,
        &order.status,
        &body.new_status,
        staff.claims.sub,
        body.notes.as_deref(),
    )
    .await;

    // Update order status
    crate::db::orders::update_order_status(pool.inner(), id, &body.new_status)
        .await
        .map_err(|e| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(format!("Failed to update status: {}", e)),
                }),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        error: None,
    }))
}

#[post("/scan", data = "<body>")]
pub async fn scan_voucher(
    pool: &State<MySqlPool>,
    staff: StaffGuard,
    body: Json<ScanVoucherRequest>,
) -> Result<Json<ApiResponse<ScanVoucherResponse>>, (Status, Json<ApiResponse<()>>)> {
    let voucher = crate::db::store::get_voucher_by_code(pool.inner(), &body.voucher_code)
        .await
        .ok_or_else(|| {
            (
                Status::NotFound,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Voucher not found".into()),
                }),
            )
        })?;

    tracing::info!(voucher = %crate::services::crypto::mask_for_log(&body.voucher_code, 4), "Voucher scanned");

    // Look up the associated order BEFORE marking as scanned so we can validate its state.
    let order = match voucher.order_id {
        Some(oid) => crate::db::orders::get_order(pool.inner(), oid).await,
        None => None,
    };

    // Validate order/reservation state: only accept vouchers for orders in a
    // pickupable state.  Cancelled, refunded, or already-completed orders must
    // not be re-accepted at the counter.
    const VALID_ORDER_STATES: &[&str] = &["Accepted", "Ready", "Confirmed"];
    let (order_is_valid, order_state_reason) = match &order {
        Some(o) if VALID_ORDER_STATES.contains(&o.status.as_str()) => (true, None),
        Some(o) => (
            false,
            Some(format!("Order is in '{}' state, not eligible for pickup", o.status)),
        ),
        None => (false, Some("No order associated with this voucher".into())),
    };

    if !order_is_valid {
        return Ok(Json(ApiResponse {
            success: true,
            data: Some(ScanVoucherResponse {
                valid: false,
                order: None,
                mismatch: true,
                mismatch_reason: order_state_reason,
            }),
            error: None,
        }));
    }

    // State is valid — mark as scanned.
    let _ = crate::db::store::mark_voucher_scanned(pool.inner(), voucher.id, staff.claims.sub).await;

    // Check voucher-order mismatch if the request provides an order_id
    let (mut mismatch_flag, mut mismatch_reason) = (voucher.mismatch_flag, voucher.mismatch_reason);
    if let (Some(voucher_order_id), Some(presented_order_id)) = (voucher.order_id, body.order_id) {
        let (matches, reason) = crate::services::fulfillment::check_voucher_match(voucher_order_id, presented_order_id);
        if !matches {
            let _ = crate::db::store::set_voucher_mismatch(
                pool.inner(),
                voucher.id,
                reason.as_deref().unwrap_or("Order mismatch"),
            )
            .await;
            mismatch_flag = true;
            mismatch_reason = reason;
        }
    }

    let order_summary = order.map(|o| OrderSummary {
        id: o.id,
        order_number: o.order_number,
        status: o.status,
        total: o.total,
        voucher_code: Some(body.voucher_code.clone()),
        created_at: o.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        pickup_slot: None,
    });

    Ok(Json(ApiResponse {
        success: true,
        data: Some(ScanVoucherResponse {
            valid: true,
            order: order_summary,
            mismatch: mismatch_flag,
            mismatch_reason,
        }),
        error: None,
    }))
}

/// Alias route: GET /dashboard/counts returns the same payload as /dashboard.
/// The frontend calls this path; the more descriptive name is intentional.
#[get("/dashboard/counts")]
pub async fn dashboard_counts(
    pool: &State<MySqlPool>,
    _staff: StaffGuard,
) -> Json<ApiResponse<DashboardStats>> {
    get_dashboard_stats(pool.inner()).await
}

#[get("/dashboard")]
pub async fn dashboard(
    pool: &State<MySqlPool>,
    _staff: StaffGuard,
) -> Json<ApiResponse<DashboardStats>> {
    get_dashboard_stats(pool.inner()).await
}

async fn get_dashboard_stats(pool: &MySqlPool) -> Json<ApiResponse<DashboardStats>> {
    let counts = crate::db::orders::count_orders_by_status(pool).await;
    let mut pending_count = 0i64;
    let mut in_prep_count = 0i64;
    let mut ready_count = 0i64;
    for (status, cnt) in counts {
        match status.as_str() {
            "Pending" | "pending" => pending_count = cnt,
            "InPrep" | "in_prep" => in_prep_count = cnt,
            "Ready" | "ready" => ready_count = cnt,
            _ => {}
        }
    }
    Json(ApiResponse {
        success: true,
        data: Some(DashboardStats {
            pending_count,
            in_prep_count,
            ready_count,
        }),
        error: None,
    })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list_all_orders, get_order, update_order_status, scan_voucher, dashboard, dashboard_counts]
}
