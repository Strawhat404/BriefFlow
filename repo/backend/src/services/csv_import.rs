use serde::Deserialize;

/// Represents a single question parsed from a CSV row.
#[derive(Debug, Clone, Deserialize)]
pub struct CsvQuestion {
    pub question_text: String,
    pub option_a: String,
    pub option_b: String,
    pub option_c: String,
    pub option_d: String,
    pub correct_answer: String,
    pub difficulty: String,
    pub explanation: String,
}

/// Parse CSV content into a list of `CsvQuestion` values.
///
/// Expected columns: question,option_a,option_b,option_c,option_d,correct,difficulty,explanation
///
/// Returns `Ok(questions)` on success, or `Err(errors)` listing per-row problems.
pub fn parse_csv(content: &str) -> Result<Vec<CsvQuestion>, Vec<String>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(content.as_bytes());

    let mut questions = Vec::new();
    let mut errors = Vec::new();

    for (idx, result) in reader.records().enumerate() {
        let row_num = idx + 2; // 1-indexed, plus header row
        match result {
            Ok(record) => {
                if record.len() < 8 {
                    errors.push(format!("Row {}: expected 8 columns, got {}", row_num, record.len()));
                    continue;
                }

                let question_text = record.get(0).unwrap_or("").trim().to_string();
                let option_a = record.get(1).unwrap_or("").trim().to_string();
                let option_b = record.get(2).unwrap_or("").trim().to_string();
                let option_c = record.get(3).unwrap_or("").trim().to_string();
                let option_d = record.get(4).unwrap_or("").trim().to_string();
                let correct_answer = record.get(5).unwrap_or("").trim().to_uppercase();
                let difficulty = record.get(6).unwrap_or("medium").trim().to_lowercase();
                let explanation = record.get(7).unwrap_or("").trim().to_string();

                if question_text.is_empty() {
                    errors.push(format!("Row {}: question text is empty", row_num));
                    continue;
                }

                if !["a", "b", "c", "d", "ab", "ac", "ad", "bc", "bd", "cd", "abc", "abd", "acd", "bcd", "abcd"]
                    .contains(&correct_answer.to_lowercase().as_str())
                {
                    errors.push(format!(
                        "Row {}: invalid correct answer '{}', expected one of A/B/C/D or combination",
                        row_num, correct_answer
                    ));
                    continue;
                }

                if !["easy", "medium", "hard"].contains(&difficulty.as_str()) {
                    errors.push(format!(
                        "Row {}: invalid difficulty '{}', expected easy/medium/hard",
                        row_num, difficulty
                    ));
                    continue;
                }

                questions.push(CsvQuestion {
                    question_text,
                    option_a,
                    option_b,
                    option_c,
                    option_d,
                    correct_answer,
                    difficulty,
                    explanation,
                });
            }
            Err(e) => {
                errors.push(format!("Row {}: parse error: {}", row_num, e));
            }
        }
    }

    if questions.is_empty() && !errors.is_empty() {
        Err(errors)
    } else {
        Ok(questions)
    }
}
