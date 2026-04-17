use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use sqlx::MySqlPool;

use shared::dto::{
    ApiResponse, ExamQuestionDetail, ExamVersionResponse, GenerateExamRequest,
    ImportQuestionsRequest, ImportQuestionsResponse, ExamOptionDetail,
    PaginatedResponse, QuestionListItem,
};
use shared::models::{Chapter, Subject};
use crate::middleware::auth_guard::{AuthenticatedUser, TeacherGuard};

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

#[get("/subjects")]
pub async fn list_subjects(
    pool: &State<MySqlPool>,
) -> Json<ApiResponse<Vec<Subject>>> {
    let subjects = crate::db::exam::list_subjects(pool.inner()).await;

    Json(ApiResponse {
        success: true,
        data: Some(subjects),
        error: None,
    })
}

#[get("/subjects/<id>/chapters")]
pub async fn list_chapters(
    pool: &State<MySqlPool>,
    id: i64,
) -> Json<ApiResponse<Vec<Chapter>>> {
    let chapters = crate::db::exam::list_chapters(pool.inner(), id).await;

    Json(ApiResponse {
        success: true,
        data: Some(chapters),
        error: None,
    })
}

#[get("/questions?<subject_id>&<chapter_id>&<difficulty>&<q>&<page>&<per_page>")]
pub async fn list_questions(
    pool: &State<MySqlPool>,
    _teacher: TeacherGuard,
    subject_id: Option<i64>,
    chapter_id: Option<i64>,
    difficulty: Option<String>,
    q: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Json<ApiResponse<PaginatedResponse<QuestionListItem>>> {
    let page_num = page.unwrap_or(1).max(1);
    let page_size = per_page.unwrap_or(20).min(100);

    let (items, total) = crate::db::exam::get_questions_paginated(
        pool.inner(),
        subject_id,
        chapter_id,
        difficulty.as_deref(),
        q.as_deref(),
        page_num,
        page_size,
    )
    .await;

    Json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse {
            items,
            total,
            page: page_num,
            per_page: page_size,
        }),
        error: None,
    })
}

/// Alias: POST /questions/import forwards to the same handler as POST /import.
#[post("/questions/import", data = "<body>")]
pub async fn import_questions_alias(
    pool: &State<MySqlPool>,
    teacher: TeacherGuard,
    body: Json<ImportQuestionsRequest>,
) -> Result<Json<ApiResponse<ImportQuestionsResponse>>, (Status, Json<ApiResponse<()>>)> {
    import_questions(pool, teacher, body).await
}

#[get("/questions/<id>")]
pub async fn get_question(
    pool: &State<MySqlPool>,
    _teacher: TeacherGuard,
    id: i64,
) -> Result<Json<ApiResponse<ExamQuestionDetail>>, (Status, Json<ApiResponse<()>>)> {
    let question = crate::db::exam::get_question(pool.inner(), id)
        .await
        .ok_or_else(|| {
            (
                Status::NotFound,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Question not found".into()),
                }),
            )
        })?;

    let opts = crate::db::exam::get_question_options(pool.inner(), question.id).await;

    let detail = ExamQuestionDetail {
        question_id: question.id,
        question_text_en: question.question_text_en,
        question_text_zh: question.question_text_zh,
        question_type: question.question_type,
        options: opts
            .into_iter()
            .map(|o| ExamOptionDetail {
                id: o.id,
                label: o.label,
                content_en: o.content_en,
                content_zh: o.content_zh,
            })
            .collect(),
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(detail),
        error: None,
    }))
}

#[post("/import", data = "<body>")]
pub async fn import_questions(
    pool: &State<MySqlPool>,
    _teacher: TeacherGuard,
    body: Json<ImportQuestionsRequest>,
) -> Result<Json<ApiResponse<ImportQuestionsResponse>>, (Status, Json<ApiResponse<()>>)> {
    let csv_result = crate::services::csv_import::parse_csv(&body.csv_content);

    let questions = csv_result.map_err(|errors| {
        (
            Status::BadRequest,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Import failed: {}", errors.join("; "))),
            }),
        )
    })?;

    let mut imported_count = 0i32;
    let mut errors: Vec<String> = Vec::new();

    for q in &questions {
        let qtype = if q.correct_answer.len() > 1 {
            "MultipleChoice"
        } else {
            "SingleChoice"
        };

        let q_id = crate::db::exam::create_question(
            pool.inner(),
            body.subject_id,
            body.chapter_id,
            &q.difficulty,
            &q.question_text,
            None,
            Some(&q.explanation),
            None,
            qtype,
        )
        .await;

        match q_id {
            Ok(question_id) => {
                let options = [
                    ("A", &q.option_a),
                    ("B", &q.option_b),
                    ("C", &q.option_c),
                    ("D", &q.option_d),
                ];
                for (idx, (label, content)) in options.iter().enumerate() {
                    let is_correct = q.correct_answer.to_uppercase().contains(label);
                    let _ = crate::db::exam::create_question_option(
                        pool.inner(),
                        question_id,
                        label,
                        content,
                        None,
                        is_correct,
                        idx as i32,
                    )
                    .await;
                }
                imported_count += 1;
            }
            Err(e) => {
                errors.push(format!("Failed to import question: {}", e));
            }
        }
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(ImportQuestionsResponse {
            imported_count,
            skipped_count: questions.len() as i32 - imported_count,
            errors,
        }),
        error: None,
    }))
}

#[post("/generate", data = "<body>")]
pub async fn generate_exam(
    pool: &State<MySqlPool>,
    teacher: TeacherGuard,
    body: Json<GenerateExamRequest>,
) -> Result<Json<ApiResponse<ExamVersionResponse>>, (Status, Json<ApiResponse<()>>)> {
    let created_by = teacher.claims.sub;

    // Generate question IDs
    let question_ids = crate::services::exam_generator::generate_exam(
        pool.inner(),
        body.subject_id,
        body.chapter_id,
        body.difficulty.as_deref(),
        body.question_count,
    )
    .await;

    if question_ids.is_empty() {
        return Err((
            Status::BadRequest,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("No questions matched the criteria".into()),
            }),
        ));
    }

    let actual_count = question_ids.len() as i32;

    // Create exam version
    let version_id = crate::db::exam::create_exam_version(
        pool.inner(),
        &body.title_en,
        body.title_zh.as_deref(),
        body.subject_id,
        body.chapter_id,
        body.difficulty.as_deref().unwrap_or("medium"),
        actual_count,
        body.time_limit_minutes,
        created_by,
    )
    .await
    .map_err(|e| {
        (
            Status::InternalServerError,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to create exam version: {}", e)),
            }),
        )
    })?;

    // Add questions to exam
    for (idx, qid) in question_ids.iter().enumerate() {
        let _ = crate::db::exam::add_exam_question(pool.inner(), version_id, *qid, idx as i32).await;
    }

    // Look up subject name if available
    let subject_name = if let Some(sid) = body.subject_id {
        let subjects = crate::db::exam::list_subjects(pool.inner()).await;
        subjects.into_iter().find(|s| s.id == sid).map(|s| s.name_en)
    } else {
        None
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(ExamVersionResponse {
            id: version_id,
            title_en: body.title_en.clone(),
            title_zh: body.title_zh.clone(),
            subject_name,
            difficulty: body.difficulty.clone().unwrap_or_else(|| "medium".into()),
            question_count: actual_count,
            time_limit_minutes: body.time_limit_minutes,
        }),
        error: None,
    }))
}

#[get("/versions")]
pub async fn list_versions(
    pool: &State<MySqlPool>,
    _user: AuthenticatedUser,
) -> Json<ApiResponse<Vec<ExamVersionResponse>>> {
    let versions = crate::db::exam::list_exam_versions(pool.inner()).await;

    let responses: Vec<ExamVersionResponse> = versions
        .into_iter()
        .map(|v| ExamVersionResponse {
            id: v.id,
            title_en: v.title_en,
            title_zh: v.title_zh,
            subject_name: None,
            difficulty: v.difficulty,
            question_count: v.question_count,
            time_limit_minutes: v.time_limit_minutes,
        })
        .collect();

    Json(ApiResponse {
        success: true,
        data: Some(responses),
        error: None,
    })
}

#[get("/versions/<id>")]
pub async fn get_version(
    pool: &State<MySqlPool>,
    _user: AuthenticatedUser,
    id: i64,
) -> Result<Json<ApiResponse<ExamVersionResponse>>, (Status, Json<ApiResponse<()>>)> {
    let version = crate::db::exam::get_exam_version(pool.inner(), id)
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

    Ok(Json(ApiResponse {
        success: true,
        data: Some(ExamVersionResponse {
            id: version.id,
            title_en: version.title_en,
            title_zh: version.title_zh,
            subject_name: None,
            difficulty: version.difficulty,
            question_count: version.question_count,
            time_limit_minutes: version.time_limit_minutes,
        }),
        error: None,
    }))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        list_subjects,
        list_chapters,
        list_questions,
        get_question,
        import_questions,
        import_questions_alias,
        generate_exam,
        list_versions,
        get_version
    ]
}
