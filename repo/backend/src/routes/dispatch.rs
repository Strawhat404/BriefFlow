use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use sqlx::MySqlPool;

use shared::dto::ApiResponse;
use crate::middleware::auth_guard::{StaffGuard, AdminGuard};
use crate::services::dispatch as dispatch_svc;

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

#[derive(Debug, serde::Deserialize)]
pub struct AssignRequest {
    pub order_id: i64,
    pub zone_id: Option<i64>,
    pub mode: String, // "Grab" or "Assigned"
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateShiftRequest {
    pub user_id: i64,
    pub zone_id: i64,
    pub shift_date: String,
    pub start_time: String,
    pub end_time: String,
}

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

#[get("/zones")]
pub async fn list_zones(pool: &State<MySqlPool>, _staff: StaffGuard) -> Json<ApiResponse<Vec<crate::db::dispatch::StationZone>>> {
    let zones = crate::db::dispatch::list_zones(pool.inner()).await;
    Json(ApiResponse { success: true, data: Some(zones), error: None })
}

#[get("/queue?<zone_id>")]
pub async fn queue(
    pool: &State<MySqlPool>,
    _staff: StaffGuard,
    zone_id: Option<i64>,
) -> Json<ApiResponse<Vec<crate::db::dispatch::TaskAssignment>>> {
    let limit = crate::db::dispatch::get_dispatch_config(pool.inner(), "grab_queue_visible_count")
        .await
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let tasks = crate::db::dispatch::get_queued_tasks(pool.inner(), zone_id, limit).await;
    Json(ApiResponse { success: true, data: Some(tasks), error: None })
}

#[post("/grab/<task_id>")]
pub async fn grab(
    pool: &State<MySqlPool>,
    staff: StaffGuard,
    task_id: i64,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    dispatch_svc::grab_task(pool.inner(), task_id, staff.claims.sub)
        .await
        .map_err(|e| (Status::Conflict, Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) })))?;
    Ok(Json(ApiResponse { success: true, data: None, error: None }))
}

#[post("/accept/<task_id>")]
pub async fn accept(
    pool: &State<MySqlPool>,
    staff: StaffGuard,
    task_id: i64,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    dispatch_svc::handle_accept(pool.inner(), task_id, staff.claims.sub)
        .await
        .map_err(|e| (Status::Conflict, Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) })))?;
    Ok(Json(ApiResponse { success: true, data: None, error: None }))
}

#[post("/reject/<task_id>")]
pub async fn reject(
    pool: &State<MySqlPool>,
    staff: StaffGuard,
    task_id: i64,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    dispatch_svc::handle_reject(pool.inner(), task_id, staff.claims.sub)
        .await
        .map_err(|e| (Status::Conflict, Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) })))?;
    Ok(Json(ApiResponse { success: true, data: None, error: None }))
}

#[post("/start/<task_id>")]
pub async fn start(
    pool: &State<MySqlPool>,
    staff: StaffGuard,
    task_id: i64,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    // Verify ownership
    let task = crate::db::dispatch::get_task(pool.inner(), task_id)
        .await
        .ok_or_else(|| (Status::NotFound, Json(ApiResponse { success: false, data: None, error: Some("Task not found".into()) })))?;
    if task.assigned_to != Some(staff.claims.sub) {
        return Err((Status::Forbidden, Json(ApiResponse { success: false, data: None, error: Some("You are not assigned to this task".into()) })));
    }
    crate::db::dispatch::start_task(pool.inner(), task_id)
        .await
        .map_err(|e| (Status::BadRequest, Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) })))?;
    Ok(Json(ApiResponse { success: true, data: None, error: None }))
}

#[post("/complete/<task_id>")]
pub async fn complete(
    pool: &State<MySqlPool>,
    staff: StaffGuard,
    task_id: i64,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    // Verify ownership
    let task = crate::db::dispatch::get_task(pool.inner(), task_id)
        .await
        .ok_or_else(|| (Status::NotFound, Json(ApiResponse { success: false, data: None, error: Some("Task not found".into()) })))?;
    if task.assigned_to != Some(staff.claims.sub) {
        return Err((Status::Forbidden, Json(ApiResponse { success: false, data: None, error: Some("You are not assigned to this task".into()) })));
    }
    dispatch_svc::complete_and_score(pool.inner(), task_id)
        .await
        .map_err(|e| (Status::BadRequest, Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) })))?;
    Ok(Json(ApiResponse { success: true, data: None, error: None }))
}

#[get("/my-tasks")]
pub async fn my_tasks(
    pool: &State<MySqlPool>,
    staff: StaffGuard,
) -> Json<ApiResponse<Vec<crate::db::dispatch::TaskAssignment>>> {
    let tasks = crate::db::dispatch::get_staff_active_tasks(pool.inner(), staff.claims.sub).await;
    Json(ApiResponse { success: true, data: Some(tasks), error: None })
}

#[post("/assign", data = "<body>")]
pub async fn assign(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
    body: Json<AssignRequest>,
) -> Result<Json<ApiResponse<i64>>, (Status, Json<ApiResponse<()>>)> {
    let task_id = if body.mode == "Assigned" {
        dispatch_svc::auto_assign(pool.inner(), body.order_id, body.zone_id).await
    } else {
        dispatch_svc::enqueue_for_grab(pool.inner(), body.order_id, body.zone_id, 50).await
    };

    match task_id {
        Ok(id) => Ok(Json(ApiResponse { success: true, data: Some(id), error: None })),
        Err(e) => Err((Status::BadRequest, Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }))),
    }
}

#[get("/recommendations/<order_id>")]
pub async fn recommendations(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
    order_id: i64,
) -> Json<ApiResponse<Vec<dispatch_svc::StaffScore>>> {
    let recs = dispatch_svc::recommend_staff(pool.inner(), order_id, None).await;
    Json(ApiResponse { success: true, data: Some(recs), error: None })
}

#[get("/shifts?<user_id>&<date>")]
pub async fn shifts(
    pool: &State<MySqlPool>,
    _staff: StaffGuard,
    user_id: i64,
    date: String,
) -> Json<ApiResponse<Vec<crate::db::dispatch::ShiftWindow>>> {
    let d = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap_or(chrono::Utc::now().date_naive());
    let s = crate::db::dispatch::get_staff_shifts(pool.inner(), user_id, d).await;
    Json(ApiResponse { success: true, data: Some(s), error: None })
}

#[post("/shifts", data = "<body>")]
pub async fn create_shift(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
    body: Json<CreateShiftRequest>,
) -> Result<Json<ApiResponse<i64>>, (Status, Json<ApiResponse<()>>)> {
    let date = chrono::NaiveDate::parse_from_str(&body.shift_date, "%Y-%m-%d")
        .map_err(|_| (Status::BadRequest, Json(ApiResponse { success: false, data: None, error: Some("Invalid date".into()) })))?;

    let id = crate::db::dispatch::create_shift(
        pool.inner(), body.user_id, body.zone_id, date, &body.start_time, &body.end_time,
    )
    .await
    .map_err(|e| (Status::InternalServerError, Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) })))?;

    Ok(Json(ApiResponse { success: true, data: Some(id), error: None }))
}

#[get("/reputation/<user_id>")]
pub async fn reputation(
    pool: &State<MySqlPool>,
    _admin: AdminGuard,
    user_id: i64,
) -> Json<ApiResponse<Option<crate::db::dispatch::StaffReputation>>> {
    let rep = crate::db::dispatch::get_reputation(pool.inner(), user_id).await;
    Json(ApiResponse { success: true, data: Some(rep), error: None })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list_zones, queue, grab, accept, reject, start, complete, my_tasks, assign, recommendations, shifts, create_shift, reputation]
}
