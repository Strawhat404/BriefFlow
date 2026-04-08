use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub preferred_locale: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spu {
    pub id: i64,
    pub name_en: String,
    pub name_zh: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub category: Option<String>,
    pub image_url: Option<String>,
    pub base_price: f64,
    pub prep_time_minutes: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionGroup {
    pub id: i64,
    pub spu_id: i64,
    pub name_en: String,
    pub name_zh: String,
    pub is_required: bool,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionValue {
    pub id: i64,
    pub group_id: i64,
    pub label_en: String,
    pub label_zh: String,
    pub price_delta: f64,
    pub is_default: bool,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sku {
    pub id: i64,
    pub spu_id: i64,
    pub sku_code: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreHours {
    pub id: i64,
    pub day_of_week: u8,
    pub open_time: String,
    pub close_time: String,
    pub is_closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub id: i64,
    pub user_id: i64,
    pub pickup_slot_start: NaiveDateTime,
    pub pickup_slot_end: NaiveDateTime,
    pub voucher_code: String,
    pub hold_expires_at: NaiveDateTime,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cart {
    pub id: i64,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub id: i64,
    pub cart_id: i64,
    pub sku_id: i64,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub reservation_id: Option<i64>,
    pub order_number: String,
    pub subtotal: f64,
    pub tax_amount: f64,
    pub total: f64,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub sku_id: i64,
    pub quantity: i32,
    pub unit_price: f64,
    pub item_total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentEvent {
    pub id: i64,
    pub order_id: i64,
    pub from_status: Option<String>,
    pub to_status: String,
    pub changed_by_user_id: i64,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voucher {
    pub id: i64,
    pub reservation_id: i64,
    pub order_id: Option<i64>,
    pub code: String,
    pub scanned_at: Option<NaiveDateTime>,
    pub scanned_by_user_id: Option<i64>,
    pub mismatch_flag: bool,
    pub mismatch_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: i64,
    pub name_en: String,
    pub name_zh: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub id: i64,
    pub subject_id: i64,
    pub name_en: String,
    pub name_zh: String,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: i64,
    pub subject_id: i64,
    pub chapter_id: Option<i64>,
    pub difficulty: String,
    pub question_text_en: String,
    pub question_text_zh: Option<String>,
    pub explanation_en: Option<String>,
    pub explanation_zh: Option<String>,
    pub question_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    pub id: i64,
    pub question_id: i64,
    pub label: String,
    pub content_en: String,
    pub content_zh: Option<String>,
    pub is_correct: bool,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamVersion {
    pub id: i64,
    pub title_en: String,
    pub title_zh: Option<String>,
    pub subject_id: Option<i64>,
    pub chapter_id: Option<i64>,
    pub difficulty: String,
    pub question_count: i32,
    pub time_limit_minutes: i32,
    pub created_by: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamAttempt {
    pub id: i64,
    pub user_id: i64,
    pub exam_version_id: i64,
    pub started_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
    pub score: Option<f64>,
    pub total_questions: i32,
    pub correct_count: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptAnswer {
    pub id: i64,
    pub attempt_id: i64,
    pub question_id: i64,
    pub selected_option_ids: Option<String>,
    pub is_correct: Option<bool>,
    pub answered_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favorite {
    pub id: i64,
    pub user_id: i64,
    pub question_id: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrongAnswerEntry {
    pub id: i64,
    pub user_id: i64,
    pub question_id: i64,
    pub wrong_count: i32,
    pub last_wrong_at: Option<NaiveDateTime>,
    pub next_review_at: Option<NaiveDateTime>,
    pub review_interval_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsSnapshot {
    pub id: i64,
    pub user_id: Option<i64>,
    pub snapshot_type: String,
    pub snapshot_data: String,
    pub snapshot_date: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesTaxConfig {
    pub id: i64,
    pub tax_name: String,
    pub rate: f64,
    pub is_active: bool,
}
