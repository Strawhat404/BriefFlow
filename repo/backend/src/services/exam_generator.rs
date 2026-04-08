use sqlx::MySqlPool;
use rand::seq::SliceRandom;

use crate::db;

/// Randomly select question IDs matching the given criteria.
///
/// Fetches all matching questions from the database, shuffles them,
/// and returns up to `count` question IDs.
pub async fn generate_exam(
    pool: &MySqlPool,
    subject_id: Option<i64>,
    chapter_id: Option<i64>,
    difficulty: Option<&str>,
    count: i32,
) -> Vec<i64> {
    // Fetch more than needed to allow random selection diversity
    let fetch_limit = (count * 3).max(50);
    let questions = db::exam::get_questions_filtered(
        pool,
        subject_id,
        chapter_id,
        difficulty,
        fetch_limit,
    )
    .await;

    let mut ids: Vec<i64> = questions.into_iter().map(|q| q.id).collect();

    let mut rng = rand::thread_rng();
    ids.shuffle(&mut rng);
    ids.truncate(count as usize);

    ids
}
