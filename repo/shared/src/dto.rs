use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Auth
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    /// The signed `brewflow_session` cookie value.  The frontend stores this
    /// and passes it as `Cookie: brewflow_session=<value>` on every request.
    pub session_cookie: String,
    pub user: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub roles: Vec<String>,
    pub preferred_locale: String,
}

// ---------------------------------------------------------------------------
// Products / Menu
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListItem {
    pub spu_id: i64,
    pub name_en: String,
    pub name_zh: String,
    pub description_en: Option<String>,
    pub description_zh: Option<String>,
    pub category: Option<String>,
    pub image_url: Option<String>,
    pub base_price: f64,
    pub prep_time_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDetail {
    pub spu: ProductListItem,
    pub option_groups: Vec<OptionGroupDetail>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionGroupDetail {
    pub id: i64,
    pub name_en: String,
    pub name_zh: String,
    pub is_required: bool,
    pub options: Vec<OptionValueDetail>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionValueDetail {
    pub id: i64,
    pub label_en: String,
    pub label_zh: String,
    pub price_delta: f64,
    pub is_default: bool,
}

// ---------------------------------------------------------------------------
// Cart
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToCartRequest {
    pub sku_id: Option<i64>,
    pub spu_id: i64,
    pub selected_options: Vec<i64>,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartResponse {
    pub items: Vec<CartItemDetail>,
    pub subtotal: f64,
    pub tax_rate: f64,
    pub tax_amount: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItemDetail {
    pub id: i64,
    pub spu_name_en: String,
    pub spu_name_zh: String,
    pub sku_code: Option<String>,
    pub options: Vec<String>,
    pub quantity: i32,
    pub unit_price: f64,
    pub line_total: f64,
}

// ---------------------------------------------------------------------------
// Checkout / Reservation
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PickupSlot {
    pub start: String,
    pub end: String,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub pickup_slot_start: String,
    pub pickup_slot_end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutResponse {
    pub order_id: i64,
    pub order_number: String,
    pub voucher_code: String,
    pub hold_expires_at: String,
    pub pickup_slot: String,
    pub total: f64,
}

// ---------------------------------------------------------------------------
// Orders
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    pub id: i64,
    pub order_number: String,
    pub status: String,
    pub total: f64,
    pub voucher_code: Option<String>,
    pub created_at: String,
    pub pickup_slot: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetail {
    pub order: OrderSummary,
    pub items: Vec<OrderItemDetail>,
    pub fulfillment_history: Vec<FulfillmentEventDetail>,
    pub reservation: Option<ReservationDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemDetail {
    pub sku_code: String,
    pub spu_name: String,
    pub options: Vec<String>,
    pub quantity: i32,
    pub unit_price: f64,
    pub item_total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentEventDetail {
    pub from_status: Option<String>,
    pub to_status: String,
    pub changed_by: String,
    pub notes: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationDetail {
    pub voucher_code: String,
    pub pickup_slot_start: String,
    pub pickup_slot_end: String,
    pub hold_expires_at: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub new_status: String,
    pub notes: Option<String>,
}

// ---------------------------------------------------------------------------
// Voucher scanning
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanVoucherRequest {
    pub voucher_code: String,
    #[serde(default)]
    pub order_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanVoucherResponse {
    pub valid: bool,
    pub order: Option<OrderSummary>,
    pub mismatch: bool,
    pub mismatch_reason: Option<String>,
}

// ---------------------------------------------------------------------------
// Question bank (admin listing with pagination)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionListItem {
    pub id: i64,
    pub question_text_en: String,
    pub question_text_zh: Option<String>,
    pub question_type: String,
    pub difficulty: String,
    pub subject_name: Option<String>,
    pub chapter_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Question bank import
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportQuestionsRequest {
    pub subject_id: i64,
    pub chapter_id: Option<i64>,
    pub csv_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportQuestionsResponse {
    pub imported_count: i32,
    pub skipped_count: i32,
    pub errors: Vec<String>,
}

// ---------------------------------------------------------------------------
// Exams
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateExamRequest {
    pub title_en: String,
    pub title_zh: Option<String>,
    pub subject_id: Option<i64>,
    pub chapter_id: Option<i64>,
    pub difficulty: Option<String>,
    pub question_count: i32,
    pub time_limit_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamVersionResponse {
    pub id: i64,
    pub title_en: String,
    pub title_zh: Option<String>,
    pub subject_name: Option<String>,
    pub difficulty: String,
    pub question_count: i32,
    pub time_limit_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartExamResponse {
    pub attempt_id: i64,
    pub questions: Vec<ExamQuestionDetail>,
    pub time_limit_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamQuestionDetail {
    pub question_id: i64,
    pub question_text_en: String,
    pub question_text_zh: Option<String>,
    pub question_type: String,
    pub options: Vec<ExamOptionDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamOptionDetail {
    pub id: i64,
    pub label: String,
    pub content_en: String,
    pub content_zh: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitAnswerRequest {
    /// `None` when submitting in wrong-answer review mode (no formal exam attempt).
    #[serde(default)]
    pub attempt_id: Option<i64>,
    pub question_id: i64,
    pub selected_option_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitAnswerResponse {
    pub is_correct: bool,
    /// Populated only when the answer is wrong, so the client can highlight correct options.
    pub correct_option_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinishExamResponse {
    pub attempt_id: i64,
    pub score: f64,
    pub total_questions: i32,
    pub correct_count: i32,
    pub wrong_questions: Vec<WrongQuestionDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrongQuestionDetail {
    pub question_id: i64,
    pub question_text_en: String,
    pub correct_options: Vec<String>,
    pub your_options: Vec<String>,
    pub explanation_en: Option<String>,
}

// ---------------------------------------------------------------------------
// Analytics
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreAnalytics {
    pub overall_score: f64,
    pub by_subject: Vec<SubjectScore>,
    pub by_difficulty: Vec<DifficultyScore>,
    pub recent_attempts: Vec<AttemptSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectScore {
    pub subject_id: i64,
    pub subject_name: String,
    pub avg_score: f64,
    pub attempt_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyScore {
    pub difficulty: String,
    pub avg_score: f64,
    pub attempt_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptSummary {
    pub id: i64,
    pub exam_title: String,
    pub score: f64,
    pub date: String,
    pub duration_minutes: Option<i32>,
}

// ---------------------------------------------------------------------------
// Wrong-answer review
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrongAnswerReviewSession {
    pub questions: Vec<ReviewQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewQuestion {
    pub question_id: i64,
    pub question_text_en: String,
    pub question_text_zh: Option<String>,
    pub question_type: String,
    pub options: Vec<ExamOptionDetail>,
    pub wrong_count: i32,
    pub last_wrong_at: String,
}

// ---------------------------------------------------------------------------
// Generic wrappers
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}
