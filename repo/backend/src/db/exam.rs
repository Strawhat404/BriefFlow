use sqlx::{MySqlPool, Row};
use shared::models::{Subject, Chapter, Question, QuestionOption, ExamVersion};

pub async fn list_subjects(pool: &MySqlPool) -> Vec<Subject> {
    let rows = sqlx::query(
        "SELECT id, name_en, name_zh, created_at FROM subjects ORDER BY id"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|r| Subject {
            id: r.get("id"),
            name_en: r.get("name_en"),
            name_zh: r.get("name_zh"),
            created_at: r.get("created_at"),
        })
        .collect()
}

pub async fn list_chapters(pool: &MySqlPool, subject_id: i64) -> Vec<Chapter> {
    let rows = sqlx::query(
        "SELECT id, subject_id, name_en, name_zh, sort_order
         FROM chapters WHERE subject_id = ? ORDER BY sort_order"
    )
    .bind(subject_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|r| Chapter {
            id: r.get("id"),
            subject_id: r.get("subject_id"),
            name_en: r.get("name_en"),
            name_zh: r.get("name_zh"),
            sort_order: r.get("sort_order"),
        })
        .collect()
}

pub async fn create_question(
    pool: &MySqlPool,
    subject_id: i64,
    chapter_id: Option<i64>,
    difficulty: &str,
    question_text_en: &str,
    question_text_zh: Option<&str>,
    explanation_en: Option<&str>,
    explanation_zh: Option<&str>,
    question_type: &str,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO questions (subject_id, chapter_id, difficulty, question_text_en, question_text_zh,
         explanation_en, explanation_zh, question_type)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(subject_id)
    .bind(chapter_id)
    .bind(difficulty)
    .bind(question_text_en)
    .bind(question_text_zh)
    .bind(explanation_en)
    .bind(explanation_zh)
    .bind(question_type)
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn create_question_option(
    pool: &MySqlPool,
    question_id: i64,
    label: &str,
    content_en: &str,
    content_zh: Option<&str>,
    is_correct: bool,
    sort_order: i32,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO question_options (question_id, label, content_en, content_zh, is_correct, sort_order)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(question_id)
    .bind(label)
    .bind(content_en)
    .bind(content_zh)
    .bind(is_correct)
    .bind(sort_order)
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

fn row_to_question(r: sqlx::mysql::MySqlRow) -> Question {
    Question {
        id: r.get("id"),
        subject_id: r.get("subject_id"),
        chapter_id: r.get("chapter_id"),
        difficulty: r.get("difficulty"),
        question_text_en: r.get("question_text_en"),
        question_text_zh: r.get("question_text_zh"),
        explanation_en: r.get("explanation_en"),
        explanation_zh: r.get("explanation_zh"),
        question_type: r.get("question_type"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }
}

pub async fn get_questions_filtered(
    pool: &MySqlPool,
    subject_id: Option<i64>,
    chapter_id: Option<i64>,
    difficulty: Option<&str>,
    limit: i32,
) -> Vec<Question> {
    let mut sql = String::from(
        "SELECT id, subject_id, chapter_id, difficulty, question_text_en, question_text_zh,
                explanation_en, explanation_zh, question_type, created_at, updated_at
         FROM questions WHERE 1=1"
    );

    if subject_id.is_some() {
        sql.push_str(" AND subject_id = ?");
    }
    if chapter_id.is_some() {
        sql.push_str(" AND chapter_id = ?");
    }
    if difficulty.is_some() {
        sql.push_str(" AND difficulty = ?");
    }
    sql.push_str(" ORDER BY RAND() LIMIT ?");

    let mut query = sqlx::query(&sql);

    if let Some(sid) = subject_id {
        query = query.bind(sid);
    }
    if let Some(cid) = chapter_id {
        query = query.bind(cid);
    }
    if let Some(d) = difficulty {
        query = query.bind(d.to_string());
    }
    query = query.bind(limit);

    let rows = query.fetch_all(pool).await.unwrap_or_default();
    rows.into_iter().map(row_to_question).collect()
}

pub async fn get_question(pool: &MySqlPool, id: i64) -> Option<Question> {
    let row = sqlx::query(
        "SELECT id, subject_id, chapter_id, difficulty, question_text_en, question_text_zh,
                explanation_en, explanation_zh, question_type, created_at, updated_at
         FROM questions WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .ok()?;

    row.map(row_to_question)
}

pub async fn get_question_options(pool: &MySqlPool, question_id: i64) -> Vec<QuestionOption> {
    let rows = sqlx::query(
        "SELECT id, question_id, label, content_en, content_zh, is_correct, sort_order
         FROM question_options WHERE question_id = ? ORDER BY sort_order"
    )
    .bind(question_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|r| QuestionOption {
            id: r.get("id"),
            question_id: r.get("question_id"),
            label: r.get("label"),
            content_en: r.get("content_en"),
            content_zh: r.get("content_zh"),
            is_correct: r.get("is_correct"),
            sort_order: r.get("sort_order"),
        })
        .collect()
}

pub async fn create_exam_version(
    pool: &MySqlPool,
    title_en: &str,
    title_zh: Option<&str>,
    subject_id: Option<i64>,
    chapter_id: Option<i64>,
    difficulty: &str,
    question_count: i32,
    time_limit_minutes: i32,
    created_by: i64,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO exam_versions (title_en, title_zh, subject_id, chapter_id, difficulty,
         question_count, time_limit_minutes, created_by)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(title_en)
    .bind(title_zh)
    .bind(subject_id)
    .bind(chapter_id)
    .bind(difficulty)
    .bind(question_count)
    .bind(time_limit_minutes)
    .bind(created_by)
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn add_exam_question(
    pool: &MySqlPool,
    exam_version_id: i64,
    question_id: i64,
    sort_order: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO exam_version_questions (exam_version_id, question_id, sort_order)
         VALUES (?, ?, ?)"
    )
    .bind(exam_version_id)
    .bind(question_id)
    .bind(sort_order)
    .execute(pool)
    .await?;

    Ok(())
}

fn row_to_exam_version(r: sqlx::mysql::MySqlRow) -> ExamVersion {
    ExamVersion {
        id: r.get("id"),
        title_en: r.get("title_en"),
        title_zh: r.get("title_zh"),
        subject_id: r.get("subject_id"),
        chapter_id: r.get("chapter_id"),
        difficulty: r.get("difficulty"),
        question_count: r.get("question_count"),
        time_limit_minutes: r.get("time_limit_minutes"),
        created_by: r.get("created_by"),
        created_at: r.get("created_at"),
        updated_at: None,
    }
}

pub async fn get_exam_version(pool: &MySqlPool, id: i64) -> Option<ExamVersion> {
    let row = sqlx::query(
        "SELECT id, title_en, title_zh, subject_id, chapter_id, difficulty,
                question_count, time_limit_minutes, created_by, created_at
         FROM exam_versions WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .ok()?;

    row.map(row_to_exam_version)
}

pub async fn list_exam_versions(pool: &MySqlPool) -> Vec<ExamVersion> {
    let rows = sqlx::query(
        "SELECT id, title_en, title_zh, subject_id, chapter_id, difficulty,
                question_count, time_limit_minutes, created_by, created_at
         FROM exam_versions ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter().map(row_to_exam_version).collect()
}

/// Returns a page of questions joined with subject/chapter names for the admin question bank.
/// Also returns the total matching count for pagination metadata.
pub async fn get_questions_paginated(
    pool: &MySqlPool,
    subject_id: Option<i64>,
    chapter_id: Option<i64>,
    difficulty: Option<&str>,
    search: Option<&str>,
    page: i32,
    per_page: i32,
) -> (Vec<shared::dto::QuestionListItem>, i64) {
    let mut where_clauses = String::from(" WHERE 1=1");
    if subject_id.is_some() { where_clauses.push_str(" AND q.subject_id = ?"); }
    if chapter_id.is_some() { where_clauses.push_str(" AND q.chapter_id = ?"); }
    if difficulty.is_some() { where_clauses.push_str(" AND q.difficulty = ?"); }
    if search.is_some()     { where_clauses.push_str(" AND q.question_text_en LIKE ?"); }

    // Count query
    let count_sql = format!(
        "SELECT COUNT(*) AS cnt FROM questions q{}",
        where_clauses
    );
    let mut count_q = sqlx::query(&count_sql);
    if let Some(v) = subject_id  { count_q = count_q.bind(v); }
    if let Some(v) = chapter_id  { count_q = count_q.bind(v); }
    if let Some(v) = difficulty  { count_q = count_q.bind(v.to_string()); }
    if let Some(v) = search      { count_q = count_q.bind(format!("%{}%", v)); }
    let total: i64 = count_q.fetch_one(pool).await
        .map(|r| r.get::<i64, _>("cnt"))
        .unwrap_or(0);

    // Data query
    let offset = ((page - 1).max(0) as i64) * (per_page as i64);
    let data_sql = format!(
        "SELECT q.id, q.question_text_en, q.question_text_zh, q.question_type, q.difficulty,
                s.name_en AS subject_name, c.name_en AS chapter_name
         FROM questions q
         LEFT JOIN subjects s ON s.id = q.subject_id
         LEFT JOIN chapters c ON c.id = q.chapter_id{}
         ORDER BY q.id DESC LIMIT ? OFFSET ?",
        where_clauses
    );
    let mut data_q = sqlx::query(&data_sql);
    if let Some(v) = subject_id  { data_q = data_q.bind(v); }
    if let Some(v) = chapter_id  { data_q = data_q.bind(v); }
    if let Some(v) = difficulty  { data_q = data_q.bind(v.to_string()); }
    if let Some(v) = search      { data_q = data_q.bind(format!("%{}%", v)); }
    data_q = data_q.bind(per_page).bind(offset);

    let rows = data_q.fetch_all(pool).await.unwrap_or_default();
    let items = rows.into_iter().map(|r| shared::dto::QuestionListItem {
        id: r.get("id"),
        question_text_en: r.get("question_text_en"),
        question_text_zh: r.get("question_text_zh"),
        question_type: r.get("question_type"),
        difficulty: r.get("difficulty"),
        subject_name: r.get("subject_name"),
        chapter_name: r.get("chapter_name"),
    }).collect();

    (items, total)
}

/// Returns questions with their options for a given exam version, ordered by sort_order.
pub async fn get_exam_questions(
    pool: &MySqlPool,
    exam_version_id: i64,
) -> Vec<(Question, Vec<QuestionOption>)> {
    let question_rows = sqlx::query(
        "SELECT q.id, q.subject_id, q.chapter_id, q.difficulty, q.question_text_en,
                q.question_text_zh, q.explanation_en, q.explanation_zh, q.question_type,
                q.created_at, q.updated_at
         FROM exam_version_questions evq
         JOIN questions q ON q.id = evq.question_id
         WHERE evq.exam_version_id = ?
         ORDER BY evq.sort_order"
    )
    .bind(exam_version_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let mut result = Vec::new();
    for r in question_rows {
        let q = row_to_question(r);
        let opts = get_question_options(pool, q.id).await;
        result.push((q, opts));
    }
    result
}
