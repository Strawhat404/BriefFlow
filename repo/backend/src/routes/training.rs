use rocket::{get, post, delete, routes};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use sqlx::MySqlPool;

use shared::dto::{
    ApiResponse, AttemptSummary, ExamQuestionDetail, ExamOptionDetail,
    FinishExamResponse, ScoreAnalytics, StartExamResponse, SubmitAnswerRequest,
    SubmitAnswerResponse, WrongAnswerReviewSession, ReviewQuestion, WrongQuestionDetail,
};
use crate::middleware::auth_guard::AuthenticatedUser;

#[derive(Debug, serde::Serialize)]
pub struct AttemptDetail {
    pub summary: AttemptSummary,
    pub answers: Vec<AttemptAnswerDetail>,
}

#[derive(Debug, serde::Serialize)]
pub struct AttemptAnswerDetail {
    pub question_id: i64,
    pub selected_option_ids: Vec<i64>,
    pub is_correct: bool,
}

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

#[post("/start/<exam_id>")]
pub async fn start_exam(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
    exam_id: i64,
) -> Result<Json<ApiResponse<StartExamResponse>>, (Status, Json<ApiResponse<()>>)> {
    let user_id = user.claims.sub;

    // Get exam version
    let version = crate::db::exam::get_exam_version(pool.inner(), exam_id)
        .await
        .ok_or_else(|| {
            (
                Status::NotFound,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Exam version not found".into()),
                }),
            )
        })?;

    // Create attempt
    let attempt_id = crate::db::training::create_attempt(
        pool.inner(),
        user_id,
        exam_id,
        version.question_count,
    )
    .await
    .map_err(|e| {
        (
            Status::InternalServerError,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to start exam: {}", e)),
            }),
        )
    })?;

    // Get exam questions
    let exam_questions = crate::db::exam::get_exam_questions(pool.inner(), exam_id).await;

    let questions: Vec<ExamQuestionDetail> = exam_questions
        .into_iter()
        .map(|(q, opts)| ExamQuestionDetail {
            question_id: q.id,
            question_text_en: q.question_text_en,
            question_text_zh: q.question_text_zh,
            question_type: q.question_type,
            options: opts
                .into_iter()
                .map(|o| ExamOptionDetail {
                    id: o.id,
                    label: o.label,
                    content_en: o.content_en,
                    content_zh: o.content_zh,
                })
                .collect(),
        })
        .collect();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(StartExamResponse {
            attempt_id,
            questions,
            time_limit_minutes: version.time_limit_minutes,
        }),
        error: None,
    }))
}

#[post("/answer", data = "<body>")]
pub async fn submit_answer(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
    body: Json<SubmitAnswerRequest>,
) -> Result<Json<ApiResponse<SubmitAnswerResponse>>, (Status, Json<ApiResponse<()>>)> {
    let user_id = user.claims.sub;

    // When attempt_id is provided, verify the attempt exists and belongs to this user.
    // When None (review-mode submission), skip attempt ownership check and DB recording.
    if let Some(attempt_id) = body.attempt_id {
        let attempt = crate::db::training::get_attempt(pool.inner(), attempt_id)
            .await
            .ok_or_else(|| {
                (
                    Status::NotFound,
                    Json(ApiResponse {
                        success: false,
                        data: None,
                        error: Some("Attempt not found".into()),
                    }),
                )
            })?;

        if attempt.user_id != user_id {
            return Err((
                Status::Forbidden,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Attempt does not belong to you".into()),
                }),
            ));
        }

        // Get correct option IDs
        let all_opts = crate::db::exam::get_question_options(pool.inner(), body.question_id).await;
        let correct_ids: Vec<i64> = all_opts
            .iter()
            .filter(|o| o.is_correct)
            .map(|o| o.id)
            .collect();

        let mut selected_sorted = body.selected_option_ids.clone();
        selected_sorted.sort();
        let mut correct_sorted = correct_ids.clone();
        correct_sorted.sort();

        let is_correct = selected_sorted == correct_sorted;

        // Record the answer in the attempt
        let selected_json = serde_json::to_string(&body.selected_option_ids).unwrap_or_default();
        crate::db::training::save_answer(
            pool.inner(),
            attempt_id,
            body.question_id,
            &selected_json,
            is_correct,
        )
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to record answer".into()),
                }),
            )
        })?;

        // Update wrong-answer notebook if incorrect
        if !is_correct {
            let _ = crate::db::training::upsert_wrong_answer(pool.inner(), user_id, body.question_id).await;
        }

        let response = SubmitAnswerResponse {
            is_correct,
            correct_option_ids: if is_correct { None } else { Some(correct_ids) },
        };

        return Ok(Json(ApiResponse {
            success: true,
            data: Some(response),
            error: None,
        }));
    }

    // Review-mode path: compute correctness only, no attempt DB writes.
    let all_opts = crate::db::exam::get_question_options(pool.inner(), body.question_id).await;
    let correct_ids: Vec<i64> = all_opts
        .iter()
        .filter(|o| o.is_correct)
        .map(|o| o.id)
        .collect();

    let mut selected_sorted = body.selected_option_ids.clone();
    selected_sorted.sort();
    let mut correct_sorted = correct_ids.clone();
    correct_sorted.sort();

    let is_correct = selected_sorted == correct_sorted;

    // Still update the wrong-answer notebook in review mode so repeat mistakes are tracked
    if !is_correct {
        let _ = crate::db::training::upsert_wrong_answer(pool.inner(), user_id, body.question_id).await;
    }

    let response = SubmitAnswerResponse {
        is_correct,
        correct_option_ids: if is_correct { None } else { Some(correct_ids) },
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

#[post("/finish/<attempt_id>")]
pub async fn finish_exam(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
    attempt_id: i64,
) -> Result<Json<ApiResponse<FinishExamResponse>>, (Status, Json<ApiResponse<()>>)> {
    let user_id = user.claims.sub;

    let attempt = crate::db::training::get_attempt(pool.inner(), attempt_id)
        .await
        .ok_or_else(|| {
            (
                Status::NotFound,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Attempt not found".into()),
                }),
            )
        })?;

    if attempt.user_id != user_id {
        return Err((
            Status::Forbidden,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Attempt does not belong to you".into()),
            }),
        ));
    }

    // Get answers to compute score
    let answers = crate::db::training::get_attempt_answers(pool.inner(), attempt_id).await;
    let correct_count = answers.iter().filter(|a| a.is_correct.unwrap_or(false)).count() as i32;
    let total = attempt.total_questions;
    let score = if total > 0 {
        (correct_count as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    // Finish the attempt
    crate::db::training::finish_attempt(pool.inner(), attempt_id, score, correct_count)
        .await
        .map_err(|e| {
            (
                Status::BadRequest,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(format!("Failed to finish exam: {}", e)),
                }),
            )
        })?;

    // Build wrong questions detail
    let wrong_answers: Vec<&shared::models::AttemptAnswer> = answers.iter().filter(|a| a.is_correct != Some(true)).collect();
    let mut wrong_questions = Vec::new();
    for wa in wrong_answers {
        if let Some(q) = crate::db::exam::get_question(pool.inner(), wa.question_id).await {
            let opts = crate::db::exam::get_question_options(pool.inner(), wa.question_id).await;
            let correct_opts: Vec<String> = opts.iter().filter(|o| o.is_correct).map(|o| o.label.clone()).collect();
            let selected: Vec<i64> = wa.selected_option_ids.as_deref().map(|s| serde_json::from_str(s).unwrap_or_default()).unwrap_or_default();
            let your_opts: Vec<String> = opts.iter().filter(|o| selected.contains(&o.id)).map(|o| o.label.clone()).collect();
            wrong_questions.push(WrongQuestionDetail {
                question_id: q.id,
                question_text_en: q.question_text_en,
                correct_options: correct_opts,
                your_options: your_opts,
                explanation_en: q.explanation_en,
            });
        }
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(FinishExamResponse {
            attempt_id,
            score,
            total_questions: total,
            correct_count,
            wrong_questions,
        }),
        error: None,
    }))
}

#[get("/attempts")]
pub async fn list_attempts(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
) -> Json<ApiResponse<Vec<AttemptSummary>>> {
    let user_id = user.claims.sub;
    let attempts = crate::db::training::get_user_attempts(pool.inner(), user_id).await;

    let summaries: Vec<AttemptSummary> = attempts
        .into_iter()
        .map(|a| AttemptSummary {
            id: a.id,
            exam_title: format!("Exam #{}", a.exam_version_id),
            score: a.score.unwrap_or(0.0),
            date: a.started_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            duration_minutes: a.finished_at.map(|f| {
                (f - a.started_at).num_minutes() as i32
            }),
        })
        .collect();

    Json(ApiResponse {
        success: true,
        data: Some(summaries),
        error: None,
    })
}

#[get("/attempts/<id>")]
pub async fn get_attempt(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
    id: i64,
) -> Result<Json<ApiResponse<AttemptDetail>>, (Status, Json<ApiResponse<()>>)> {
    let user_id = user.claims.sub;

    let attempt = crate::db::training::get_attempt(pool.inner(), id)
        .await
        .ok_or_else(|| {
            (
                Status::NotFound,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Attempt not found".into()),
                }),
            )
        })?;

    if attempt.user_id != user_id {
        return Err((
            Status::Forbidden,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Attempt does not belong to you".into()),
            }),
        ));
    }

    let answers_raw = crate::db::training::get_attempt_answers(pool.inner(), id).await;

    let answers: Vec<AttemptAnswerDetail> = answers_raw
        .into_iter()
        .map(|a| {
            let selected: Vec<i64> = a.selected_option_ids.as_deref().map(|s| serde_json::from_str(s).unwrap_or_default()).unwrap_or_default();
            AttemptAnswerDetail {
                question_id: a.question_id,
                selected_option_ids: selected,
                is_correct: a.is_correct.unwrap_or(false),
            }
        })
        .collect();

    let summary = AttemptSummary {
        id: attempt.id,
        exam_title: format!("Exam #{}", attempt.exam_version_id),
        score: attempt.score.unwrap_or(0.0),
        date: attempt.started_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        duration_minutes: attempt.finished_at.map(|f| {
            (f - attempt.started_at).num_minutes() as i32
        }),
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(AttemptDetail { summary, answers }),
        error: None,
    }))
}

#[get("/analytics")]
pub async fn get_analytics(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
) -> Json<ApiResponse<ScoreAnalytics>> {
    let user_id = user.claims.sub;

    let (overall_score, _total_attempts, _total_correct, _total_questions) =
        crate::db::analytics::get_user_score_analytics(pool.inner(), user_id).await;

    let by_subject = crate::db::analytics::get_subject_stats(pool.inner(), user_id).await;
    let by_difficulty = crate::db::analytics::get_difficulty_breakdown(pool.inner(), user_id).await;

    let recent_raw = crate::db::training::get_user_attempts(pool.inner(), user_id).await;
    let recent_attempts: Vec<AttemptSummary> = recent_raw
        .into_iter()
        .take(10)
        .map(|a| AttemptSummary {
            id: a.id,
            exam_title: format!("Exam #{}", a.exam_version_id),
            score: a.score.unwrap_or(0.0),
            date: a.started_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            duration_minutes: a.finished_at.map(|f| {
                (f - a.started_at).num_minutes() as i32
            }),
        })
        .collect();

    Json(ApiResponse {
        success: true,
        data: Some(ScoreAnalytics {
            overall_score,
            by_subject,
            by_difficulty,
            recent_attempts,
        }),
        error: None,
    })
}

#[post("/favorites/<question_id>")]
pub async fn add_favorite(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
    question_id: i64,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    crate::db::training::add_favorite(pool.inner(), user.claims.sub, question_id)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to add favorite".into()),
                }),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        error: None,
    }))
}

#[delete("/favorites/<question_id>")]
pub async fn remove_favorite(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
    question_id: i64,
) -> Result<Json<ApiResponse<()>>, (Status, Json<ApiResponse<()>>)> {
    crate::db::training::remove_favorite(pool.inner(), user.claims.sub, question_id)
        .await
        .map_err(|_| {
            (
                Status::InternalServerError,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to remove favorite".into()),
                }),
            )
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        error: None,
    }))
}

#[get("/favorites")]
pub async fn list_favorites(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
) -> Json<ApiResponse<Vec<ExamQuestionDetail>>> {
    let questions = crate::db::training::get_favorites(pool.inner(), user.claims.sub).await;

    let mut details = Vec::new();
    for q in questions {
        let opts = crate::db::exam::get_question_options(pool.inner(), q.id).await;
        details.push(ExamQuestionDetail {
            question_id: q.id,
            question_text_en: q.question_text_en,
            question_text_zh: q.question_text_zh,
            question_type: q.question_type,
            options: opts
                .into_iter()
                .map(|o| ExamOptionDetail {
                    id: o.id,
                    label: o.label,
                    content_en: o.content_en,
                    content_zh: o.content_zh,
                })
                .collect(),
        });
    }

    Json(ApiResponse {
        success: true,
        data: Some(details),
        error: None,
    })
}

#[get("/wrong-notebook")]
pub async fn wrong_notebook(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
) -> Json<ApiResponse<WrongAnswerReviewSession>> {
    let entries = crate::db::training::get_wrong_notebook(pool.inner(), user.claims.sub).await;

    let mut questions = Vec::new();
    for entry in entries {
        if let Some(q) = crate::db::exam::get_question(pool.inner(), entry.question_id).await {
            let opts = crate::db::exam::get_question_options(pool.inner(), entry.question_id).await;
            questions.push(ReviewQuestion {
                question_id: q.id,
                question_text_en: q.question_text_en,
                question_text_zh: q.question_text_zh,
                question_type: q.question_type,
                options: opts
                    .into_iter()
                    .map(|o| ExamOptionDetail {
                        id: o.id,
                        label: o.label,
                        content_en: o.content_en,
                        content_zh: o.content_zh,
                    })
                    .collect(),
                wrong_count: entry.wrong_count,
                last_wrong_at: entry.last_wrong_at.map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string()).unwrap_or_default(),
            });
        }
    }

    Json(ApiResponse {
        success: true,
        data: Some(WrongAnswerReviewSession { questions }),
        error: None,
    })
}

#[get("/review-session")]
pub async fn review_session(
    pool: &State<MySqlPool>,
    user: AuthenticatedUser,
) -> Json<ApiResponse<WrongAnswerReviewSession>> {
    let entries = crate::db::training::get_wrong_answers_for_review(pool.inner(), user.claims.sub).await;

    let mut questions = Vec::new();
    for entry in entries {
        if let Some(q) = crate::db::exam::get_question(pool.inner(), entry.question_id).await {
            let opts = crate::db::exam::get_question_options(pool.inner(), entry.question_id).await;
            questions.push(ReviewQuestion {
                question_id: q.id,
                question_text_en: q.question_text_en,
                question_text_zh: q.question_text_zh,
                question_type: q.question_type,
                options: opts
                    .into_iter()
                    .map(|o| ExamOptionDetail {
                        id: o.id,
                        label: o.label,
                        content_en: o.content_en,
                        content_zh: o.content_zh,
                    })
                    .collect(),
                wrong_count: entry.wrong_count,
                last_wrong_at: entry.last_wrong_at.map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string()).unwrap_or_default(),
            });
        }
    }

    Json(ApiResponse {
        success: true,
        data: Some(WrongAnswerReviewSession { questions }),
        error: None,
    })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        start_exam,
        submit_answer,
        finish_exam,
        list_attempts,
        get_attempt,
        get_analytics,
        add_favorite,
        remove_favorite,
        list_favorites,
        wrong_notebook,
        review_session
    ]
}
