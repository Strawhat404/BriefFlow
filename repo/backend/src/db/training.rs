use sqlx::{MySqlPool, Row};
use shared::models::{ExamAttempt, AttemptAnswer, Question, WrongAnswerEntry};

pub async fn create_attempt(
    pool: &MySqlPool,
    user_id: i64,
    exam_version_id: i64,
    total_questions: i32,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO exam_attempts (user_id, exam_version_id, total_questions, correct_count, status)
         VALUES (?, ?, ?, 0, 'InProgress')"
    )
    .bind(user_id)
    .bind(exam_version_id)
    .bind(total_questions)
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn save_answer(
    pool: &MySqlPool,
    attempt_id: i64,
    question_id: i64,
    selected_ids_json: &str,
    is_correct: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO attempt_answers (attempt_id, question_id, selected_option_ids, is_correct)
         VALUES (?, ?, ?, ?)"
    )
    .bind(attempt_id)
    .bind(question_id)
    .bind(selected_ids_json)
    .bind(is_correct)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn finish_attempt(
    pool: &MySqlPool,
    attempt_id: i64,
    score: f64,
    correct_count: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE exam_attempts SET finished_at = NOW(), score = ?, correct_count = ?, status = 'Completed'
         WHERE id = ?"
    )
    .bind(score)
    .bind(correct_count)
    .bind(attempt_id)
    .execute(pool)
    .await?;

    Ok(())
}

fn row_to_attempt(r: sqlx::mysql::MySqlRow) -> ExamAttempt {
    ExamAttempt {
        id: r.get("id"),
        user_id: r.get("user_id"),
        exam_version_id: r.get("exam_version_id"),
        started_at: r.get("started_at"),
        finished_at: r.get("finished_at"),
        score: r.get("score"),
        total_questions: r.get("total_questions"),
        correct_count: r.get("correct_count"),
        status: r.get("status"),
    }
}

pub async fn get_attempt(pool: &MySqlPool, id: i64) -> Option<ExamAttempt> {
    let row = sqlx::query(
        "SELECT id, user_id, exam_version_id, started_at, finished_at, score,
                total_questions, correct_count, status
         FROM exam_attempts WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .ok()?;

    row.map(row_to_attempt)
}

pub async fn get_user_attempts(pool: &MySqlPool, user_id: i64) -> Vec<ExamAttempt> {
    let rows = sqlx::query(
        "SELECT id, user_id, exam_version_id, started_at, finished_at, score,
                total_questions, correct_count, status
         FROM exam_attempts WHERE user_id = ? ORDER BY started_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter().map(row_to_attempt).collect()
}

pub async fn get_attempt_answers(pool: &MySqlPool, attempt_id: i64) -> Vec<AttemptAnswer> {
    let rows = sqlx::query(
        "SELECT id, attempt_id, question_id, selected_option_ids, is_correct, answered_at
         FROM attempt_answers WHERE attempt_id = ? ORDER BY id"
    )
    .bind(attempt_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|r| AttemptAnswer {
            id: r.get("id"),
            attempt_id: r.get("attempt_id"),
            question_id: r.get("question_id"),
            selected_option_ids: r.get("selected_option_ids"),
            is_correct: r.get("is_correct"),
            answered_at: r.get("answered_at"),
        })
        .collect()
}

pub async fn add_favorite(
    pool: &MySqlPool,
    user_id: i64,
    question_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT IGNORE INTO favorites (user_id, question_id) VALUES (?, ?)"
    )
    .bind(user_id)
    .bind(question_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_favorite(
    pool: &MySqlPool,
    user_id: i64,
    question_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM favorites WHERE user_id = ? AND question_id = ?")
        .bind(user_id)
        .bind(question_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_favorites(pool: &MySqlPool, user_id: i64) -> Vec<Question> {
    let rows = sqlx::query(
        "SELECT q.id, q.subject_id, q.chapter_id, q.difficulty, q.question_text_en,
                q.question_text_zh, q.explanation_en, q.explanation_zh, q.question_type,
                q.created_at, q.updated_at
         FROM favorites f
         JOIN questions q ON q.id = f.question_id
         WHERE f.user_id = ?
         ORDER BY f.created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter()
        .map(|r| Question {
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
        })
        .collect()
}

pub async fn upsert_wrong_answer(
    pool: &MySqlPool,
    user_id: i64,
    question_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO wrong_answer_notebook (user_id, question_id, wrong_count, last_wrong_at, review_interval_days)
         VALUES (?, ?, 1, NOW(), 1)
         ON DUPLICATE KEY UPDATE
           wrong_count = wrong_count + 1,
           last_wrong_at = NOW(),
           next_review_at = DATE_ADD(NOW(), INTERVAL review_interval_days DAY)"
    )
    .bind(user_id)
    .bind(question_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_wrong_answers_for_review(pool: &MySqlPool, user_id: i64) -> Vec<WrongAnswerEntry> {
    let rows = sqlx::query(
        "SELECT id, user_id, question_id, wrong_count, last_wrong_at, next_review_at, review_interval_days
         FROM wrong_answer_notebook
         WHERE user_id = ? AND (next_review_at IS NULL OR next_review_at <= NOW())
         ORDER BY wrong_count DESC, last_wrong_at ASC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter().map(row_to_wrong_entry).collect()
}

pub async fn get_wrong_notebook(pool: &MySqlPool, user_id: i64) -> Vec<WrongAnswerEntry> {
    let rows = sqlx::query(
        "SELECT id, user_id, question_id, wrong_count, last_wrong_at, next_review_at, review_interval_days
         FROM wrong_answer_notebook
         WHERE user_id = ?
         ORDER BY wrong_count DESC, last_wrong_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.into_iter().map(row_to_wrong_entry).collect()
}

fn row_to_wrong_entry(r: sqlx::mysql::MySqlRow) -> WrongAnswerEntry {
    WrongAnswerEntry {
        id: r.get("id"),
        user_id: r.get("user_id"),
        question_id: r.get("question_id"),
        wrong_count: r.get("wrong_count"),
        last_wrong_at: r.get("last_wrong_at"),
        next_review_at: r.get("next_review_at"),
        review_interval_days: r.get("review_interval_days"),
    }
}
