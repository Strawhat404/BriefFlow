use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// Role
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "staff")]
    Staff,
    #[serde(rename = "customer")]
    Customer,
    #[serde(rename = "academic_affairs")]
    AcademicAffairs,
    #[serde(rename = "teacher")]
    Teacher,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Role::Admin => "admin",
            Role::Staff => "staff",
            Role::Customer => "customer",
            Role::AcademicAffairs => "academic_affairs",
            Role::Teacher => "teacher",
        };
        write!(f, "{}", s)
    }
}

// ---------------------------------------------------------------------------
// Locale
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Locale {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "zh")]
    Zh,
}

impl Locale {
    pub fn to_str(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Zh => "zh",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "zh" | "ZH" | "zh-CN" | "zh-TW" => Locale::Zh,
            _ => Locale::En,
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

// ---------------------------------------------------------------------------
// OrderStatus
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "in_prep")]
    InPrep,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "picked_up")]
    PickedUp,
    #[serde(rename = "canceled")]
    Canceled,
}

impl OrderStatus {
    pub fn allowed_transitions(&self) -> Vec<OrderStatus> {
        match self {
            OrderStatus::Pending => vec![OrderStatus::Accepted, OrderStatus::Canceled],
            OrderStatus::Accepted => vec![OrderStatus::InPrep, OrderStatus::Canceled],
            OrderStatus::InPrep => vec![OrderStatus::Ready, OrderStatus::Canceled],
            OrderStatus::Ready => vec![OrderStatus::PickedUp, OrderStatus::Canceled],
            OrderStatus::PickedUp => vec![],
            OrderStatus::Canceled => vec![],
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderStatus::Pending => "pending",
            OrderStatus::Accepted => "accepted",
            OrderStatus::InPrep => "in_prep",
            OrderStatus::Ready => "ready",
            OrderStatus::PickedUp => "picked_up",
            OrderStatus::Canceled => "canceled",
        };
        write!(f, "{}", s)
    }
}

// ---------------------------------------------------------------------------
// ReservationStatus
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReservationStatus {
    #[serde(rename = "held")]
    Held,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "canceled")]
    Canceled,
}

impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ReservationStatus::Held => "held",
            ReservationStatus::Confirmed => "confirmed",
            ReservationStatus::Expired => "expired",
            ReservationStatus::Canceled => "canceled",
        };
        write!(f, "{}", s)
    }
}

// ---------------------------------------------------------------------------
// QuestionType
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestionType {
    #[serde(rename = "single_choice")]
    SingleChoice,
    #[serde(rename = "multiple_choice")]
    MultipleChoice,
    #[serde(rename = "true_false")]
    TrueFalse,
}

impl fmt::Display for QuestionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            QuestionType::SingleChoice => "single_choice",
            QuestionType::MultipleChoice => "multiple_choice",
            QuestionType::TrueFalse => "true_false",
        };
        write!(f, "{}", s)
    }
}

// ---------------------------------------------------------------------------
// Difficulty
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    #[serde(rename = "easy")]
    Easy,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "hard")]
    Hard,
    #[serde(rename = "mixed")]
    Mixed,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
            Difficulty::Mixed => "mixed",
        };
        write!(f, "{}", s)
    }
}

// ---------------------------------------------------------------------------
// ExamAttemptStatus
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExamAttemptStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "abandoned")]
    Abandoned,
}

impl fmt::Display for ExamAttemptStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ExamAttemptStatus::InProgress => "in_progress",
            ExamAttemptStatus::Completed => "completed",
            ExamAttemptStatus::Abandoned => "abandoned",
        };
        write!(f, "{}", s)
    }
}

// ---------------------------------------------------------------------------
// SnapshotType
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotType {
    #[serde(rename = "user_score")]
    UserScore,
    #[serde(rename = "subject_stats")]
    SubjectStats,
    #[serde(rename = "difficulty_breakdown")]
    DifficultyBreakdown,
    #[serde(rename = "daily_activity")]
    DailyActivity,
}

impl fmt::Display for SnapshotType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SnapshotType::UserScore => "user_score",
            SnapshotType::SubjectStats => "subject_stats",
            SnapshotType::DifficultyBreakdown => "difficulty_breakdown",
            SnapshotType::DailyActivity => "daily_activity",
        };
        write!(f, "{}", s)
    }
}
