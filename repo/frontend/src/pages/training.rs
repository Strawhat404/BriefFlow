use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::state::AppState;
use shared::dto::{
    ApiResponse, AttemptSummary, ExamOptionDetail, ExamQuestionDetail, ExamVersionResponse,
    FinishExamResponse, ReviewQuestion, ScoreAnalytics, StartExamResponse, SubmitAnswerRequest,
    SubmitAnswerResponse, WrongAnswerReviewSession,
};

// ---------------------------------------------------------------------------
// Favorite question DTO (matches ExamQuestionDetail returned by the backend)
// ---------------------------------------------------------------------------
#[derive(serde::Deserialize, Clone, Debug)]
struct FavoriteQuestionItem {
    question_id: i64,
    question_text_en: String,
    question_text_zh: Option<String>,
    // question_type and options are returned but not used on this page; serde ignores extras.
}

// WrongNotebookPage uses WrongAnswerReviewSession / ReviewQuestion from shared (imported above).

// ---------------------------------------------------------------------------
// TrainingPage (Hub)
// ---------------------------------------------------------------------------
#[component]
pub fn TrainingPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();

    let page_title = t.t(&loc, "nav.training");
    let exams_title = t.t(&loc, "page.mock_exams");
    let analytics_title = t.t(&loc, "page.analytics");
    let favorites_title = t.t(&loc, "page.favorites");
    let wrong_title = t.t(&loc, "page.wrong_notebook");
    let review_title = t.t(&loc, "btn.review");

    rsx! {
        div { class: "page page-training",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    div { class: "training-hub-grid",
                        Link {
                            to: crate::Route::MockExams { locale: locale.clone() },
                            class: "training-hub-card",
                            div { class: "training-hub-icon", "\u{1f4dd}" }
                            h3 { "{exams_title}" }
                            p { if loc == "zh" { "\u{7ec3}\u{4e60}\u{6a21}\u{62df}\u{8003}\u{8bd5}" } else { "Practice with mock exams" } }
                        }
                        Link {
                            to: crate::Route::Analytics { locale: locale.clone() },
                            class: "training-hub-card",
                            div { class: "training-hub-icon", "\u{1f4ca}" }
                            h3 { "{analytics_title}" }
                            p { if loc == "zh" { "\u{67e5}\u{770b}\u{6210}\u{7ee9}\u{5206}\u{6790}" } else { "View score analytics" } }
                        }
                        Link {
                            to: crate::Route::Favorites { locale: locale.clone() },
                            class: "training-hub-card",
                            div { class: "training-hub-icon", "\u{2b50}" }
                            h3 { "{favorites_title}" }
                            p { if loc == "zh" { "\u{6536}\u{85cf}\u{7684}\u{9898}\u{76ee}" } else { "Your saved questions" } }
                        }
                        Link {
                            to: crate::Route::WrongNotebook { locale: locale.clone() },
                            class: "training-hub-card",
                            div { class: "training-hub-icon", "\u{1f4d3}" }
                            h3 { "{wrong_title}" }
                            p { if loc == "zh" { "\u{590d}\u{4e60}\u{9519}\u{9898}" } else { "Review wrong answers" } }
                        }
                        Link {
                            to: crate::Route::ReviewSession { locale: locale.clone() },
                            class: "training-hub-card",
                            div { class: "training-hub-icon", "\u{1f504}" }
                            h3 { "{review_title}" }
                            p { if loc == "zh" { "\u{667a}\u{80fd}\u{590d}\u{4e60}\u{6a21}\u{5f0f}" } else { "Smart review mode" } }
                        }
                    }
                }
            }

            Footer {}
        }
    }
}

// ---------------------------------------------------------------------------
// MockExamsPage
// ---------------------------------------------------------------------------
#[component]
pub fn MockExamsPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let exams_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/exam/versions", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<ExamVersionResponse>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    let page_title = t.t(&loc, "page.mock_exams");
    let difficulty_label = t.t(&loc, "label.difficulty");
    let time_limit_label = t.t(&loc, "label.time_limit");
    let start_text = t.t(&loc, "btn.start_exam");

    rsx! {
        div { class: "page page-mock-exams",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    match &*exams_resource.read() {
                        Some(Ok(exams)) => {
                            if exams.is_empty() {
                                rsx! {
                                    p { class: "empty-text",
                                        if loc == "zh" { "\u{6682}\u{65e0}\u{53ef}\u{7528}\u{8003}\u{8bd5}" } else { "No exams available" }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "exam-list",
                                        for exam in exams.iter() {
                                            {
                                                let eid = exam.id;
                                                let title = if loc == "zh" {
                                                    exam.title_zh.as_deref().unwrap_or(&exam.title_en)
                                                } else {
                                                    &exam.title_en
                                                };
                                                let subject = exam.subject_name.as_deref().unwrap_or("-");
                                                rsx! {
                                                    div { class: "exam-card",
                                                        div { class: "exam-card-header",
                                                            h3 { class: "exam-title", "{title}" }
                                                            span { class: "exam-subject", "{subject}" }
                                                        }
                                                        div { class: "exam-card-body",
                                                            div { class: "exam-meta",
                                                                span { class: "exam-difficulty",
                                                                    "{difficulty_label}: {exam.difficulty}"
                                                                }
                                                                span { class: "exam-question-count",
                                                                    if loc == "zh" {
                                                                        "{exam.question_count}\u{9898}"
                                                                    } else {
                                                                        "{exam.question_count} questions"
                                                                    }
                                                                }
                                                                span { class: "exam-time-limit",
                                                                    "{time_limit_label}: {exam.time_limit_minutes} min"
                                                                }
                                                            }
                                                        }
                                                        div { class: "exam-card-footer",
                                                            Link {
                                                                to: crate::Route::TakeExam { locale: locale.clone(), id: eid },
                                                                class: "btn btn-primary",
                                                                "{start_text}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            div { class: "alert alert-error", "Error: {e}" }
                        },
                        None => rsx! {
                            div { class: "loading-spinner", p { "Loading..." } }
                        },
                    }
                }
            }

            Footer {}
        }
    }
}

// ---------------------------------------------------------------------------
// TakeExamPage
// ---------------------------------------------------------------------------
#[component]
pub fn TakeExamPage(locale: String, id: i64) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let mut current_index = use_signal(|| 0usize);
    let mut answers = use_signal(|| std::collections::HashMap::<i64, Vec<i64>>::new());
    let mut exam_data = use_signal(|| Option::<StartExamResponse>::None);
    let mut exam_result = use_signal(|| Option::<FinishExamResponse>::None);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| true);
    let mut finishing = use_signal(|| false);
    let mut timer_secs = use_signal(|| 0i64);

    // Start exam on mount
    let locale_start = locale.clone();
    use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .post(&format!("{}/training/start/{}", crate::API_BASE, id));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            match req.send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.json::<ApiResponse<StartExamResponse>>().await {
                            Ok(api) => {
                                if let Some(data) = api.data {
                                    timer_secs.set(data.time_limit_minutes as i64 * 60);
                                    exam_data.set(Some(data));
                                } else {
                                    error_msg.set(Some(api.error.unwrap_or_else(|| "Failed to start exam".to_string())));
                                }
                            }
                            Err(e) => error_msg.set(Some(format!("Parse error: {}", e))),
                        }
                    } else {
                        let body = resp.text().await.unwrap_or_default();
                        error_msg.set(Some(format!("Failed: {}", body)));
                    }
                }
                Err(e) => error_msg.set(Some(format!("Network error: {}", e))),
            }
            loading.set(false);
        }
    });

    // Timer countdown
    use_future(move || {
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let current = timer_secs();
                if current <= 0 || exam_result().is_some() {
                    break;
                }
                timer_secs.set(current - 1);
            }
        }
    });

    let finish_text = t.t(&loc, "btn.finish_exam");
    let complete_msg = t.t(&loc, "msg.exam_complete");
    let score_label = t.t(&loc, "label.score");

    // Show results if exam is finished
    if let Some(result) = exam_result() {
        return rsx! {
            div { class: "page page-take-exam",
                Navbar { locale: locale.clone() }

                main { class: "main-content",
                    section { class: "section exam-results",
                        h2 { class: "section-title", "{complete_msg}" }

                        div { class: "result-summary-card",
                            div { class: "result-score",
                                h3 { "{score_label}" }
                                div { class: "score-display", "{result.score:.1}%" }
                            }
                            div { class: "result-stats",
                                p {
                                    if loc == "zh" {
                                        "\u{6b63}\u{786e}: {result.correct_count}/{result.total_questions}"
                                    } else {
                                        "Correct: {result.correct_count}/{result.total_questions}"
                                    }
                                }
                            }
                        }

                        // Wrong questions detail
                        if !result.wrong_questions.is_empty() {
                            div { class: "wrong-questions-section",
                                h3 {
                                    if loc == "zh" { "\u{9519}\u{9898}\u{8be6}\u{60c5}" } else { "Wrong Answers" }
                                }
                                for wq in result.wrong_questions.iter() {
                                    div { class: "wrong-question-card",
                                        p { class: "wrong-question-text", "{wq.question_text_en}" }
                                        div { class: "wrong-question-answers",
                                            p { class: "correct-answer",
                                                if loc == "zh" { "\u{6b63}\u{786e}\u{7b54}\u{6848}: " } else { "Correct: " }
                                                strong { "{wq.correct_options.join(\", \")}" }
                                            }
                                            p { class: "your-answer",
                                                if loc == "zh" { "\u{4f60}\u{7684}\u{7b54}\u{6848}: " } else { "Your answer: " }
                                                strong { "{wq.your_options.join(\", \")}" }
                                            }
                                        }
                                        if let Some(ref explanation) = wq.explanation_en {
                                            p { class: "wrong-question-explanation", "{explanation}" }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "exam-result-actions",
                            Link {
                                to: crate::Route::MockExams { locale: locale.clone() },
                                class: "btn btn-primary",
                                if loc == "zh" { "\u{8fd4}\u{56de}\u{8003}\u{8bd5}\u{5217}\u{8868}" } else { "Back to Exams" }
                            }
                            Link {
                                to: crate::Route::Analytics { locale: locale.clone() },
                                class: "btn btn-secondary",
                                if loc == "zh" { "\u{67e5}\u{770b}\u{5206}\u{6790}" } else { "View Analytics" }
                            }
                        }
                    }
                }

                Footer {}
            }
        };
    }

    rsx! {
        div { class: "page page-take-exam",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                if loading() {
                    div { class: "loading-spinner", p { "Starting exam..." } }
                } else if let Some(err) = error_msg() {
                    div { class: "section",
                        div { class: "alert alert-error", "{err}" }
                        Link {
                            to: crate::Route::MockExams { locale: locale.clone() },
                            class: "btn btn-secondary",
                            if loc == "zh" { "\u{8fd4}\u{56de}" } else { "Back" }
                        }
                    }
                } else if let Some(data) = exam_data() {
                    {
                        let total_questions = data.questions.len();
                        let idx = current_index();
                        let secs = timer_secs();
                        let mins = secs / 60;
                        let sec_part = secs % 60;
                        let timer_display = format!("{:02}:{:02}", mins, sec_part);
                        let timer_class = if secs < 60 { "exam-timer exam-timer-urgent" } else { "exam-timer" };

                        if idx < total_questions {
                            let question = &data.questions[idx];
                            let q_id = question.question_id;
                            let q_text = if loc == "zh" {
                                question.question_text_zh.as_deref().unwrap_or(&question.question_text_en)
                            } else {
                                &question.question_text_en
                            };
                            let is_multi = question.question_type == "multiple_choice";
                            let current_answers = answers().get(&q_id).cloned().unwrap_or_default();

                            rsx! {
                                section { class: "section exam-section",
                                    // Exam header: timer + progress
                                    div { class: "exam-header",
                                        span { class: "{timer_class}", "{timer_display}" }
                                        span { class: "exam-progress",
                                            "{idx + 1} / {total_questions}"
                                        }
                                    }

                                    // Question
                                    div { class: "exam-question-card",
                                        h3 { class: "exam-question-number",
                                            if loc == "zh" { "\u{7b2c}{idx + 1}\u{9898}" } else { "Question {idx + 1}" }
                                        }
                                        p { class: "exam-question-text", "{q_text}" }

                                        // Options
                                        div { class: "exam-options",
                                            for opt in question.options.iter() {
                                                {
                                                    let opt_id = opt.id;
                                                    let is_selected = current_answers.contains(&opt_id);
                                                    let opt_content = if loc == "zh" {
                                                        opt.content_zh.as_deref().unwrap_or(&opt.content_en)
                                                    } else {
                                                        &opt.content_en
                                                    };
                                                    let btn_class = if is_selected {
                                                        "exam-option exam-option-selected"
                                                    } else {
                                                        "exam-option"
                                                    };

                                                    rsx! {
                                                        button {
                                                            class: "{btn_class}",
                                                            onclick: move |_| {
                                                                let mut ans = answers.write();
                                                                let entry = ans.entry(q_id).or_insert_with(Vec::new);
                                                                if is_multi {
                                                                    if let Some(pos) = entry.iter().position(|&id| id == opt_id) {
                                                                        entry.remove(pos);
                                                                    } else {
                                                                        entry.push(opt_id);
                                                                    }
                                                                } else {
                                                                    *entry = vec![opt_id];
                                                                }
                                                            },
                                                            span { class: "exam-option-label", "{opt.label}. " }
                                                            span { class: "exam-option-content", "{opt_content}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Navigation buttons
                                    div { class: "exam-nav",
                                        button {
                                            class: "btn btn-secondary",
                                            disabled: idx == 0,
                                            onclick: move |_| {
                                                if current_index() > 0 {
                                                    current_index.set(current_index() - 1);
                                                }
                                            },
                                            if loc == "zh" { "\u{4e0a}\u{4e00}\u{9898}" } else { "Previous" }
                                        }

                                        if idx < total_questions - 1 {
                                            button {
                                                class: "btn btn-primary",
                                                onclick: move |_| {
                                                    current_index.set(current_index() + 1);
                                                },
                                                if loc == "zh" { "\u{4e0b}\u{4e00}\u{9898}" } else { "Next" }
                                            }
                                        }

                                        {
                                            let locale_finish = locale.clone();
                                            rsx! {
                                                button {
                                                    class: "btn btn-warning",
                                                    disabled: finishing(),
                                                    onclick: move |_| {
                                                        let session_cookie = app_state().auth.session_cookie.clone();
                                                        let attempt_id = data.attempt_id;
                                                        let all_answers = answers();
                                                        spawn(async move {
                                                            finishing.set(true);

                                                            // Submit all answers
                                                            for (qid, opts) in all_answers.iter() {
                                                                let body = SubmitAnswerRequest {
                                                                    attempt_id: Some(attempt_id),
                                                                    question_id: *qid,
                                                                    selected_option_ids: opts.clone(),
                                                                };
                                                                let mut req = reqwest::Client::new()
                                                                    .post(&format!("{}/training/answer", crate::API_BASE))
                                                                    .json(&body);
                                                                if let Some(ref sc) = session_cookie {
                                                                    req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                                }
                                                                let _ = req.send().await;
                                                            }

                                                            // Finish exam
                                                            let mut req = reqwest::Client::new()
                                                                .post(&format!("{}/training/finish/{}", crate::API_BASE, attempt_id));
                                                            if let Some(ref sc) = session_cookie {
                                                                req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                            }

                                                            match req.send().await {
                                                                Ok(resp) => {
                                                                    if resp.status().is_success() {
                                                                        if let Ok(api) = resp.json::<ApiResponse<FinishExamResponse>>().await {
                                                                            if let Some(result) = api.data {
                                                                                exam_result.set(Some(result));
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(_) => {}
                                                            }
                                                            finishing.set(false);
                                                        });
                                                    },
                                                    if finishing() { "..." } else { "{finish_text}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                div { class: "section",
                                    p { "No questions" }
                                }
                            }
                        }
                    }
                }
            }

            Footer {}
        }
    }
}

// ---------------------------------------------------------------------------
// AnalyticsPage
// ---------------------------------------------------------------------------
#[component]
pub fn AnalyticsPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let analytics_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/training/analytics", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<ScoreAnalytics> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    let page_title = t.t(&loc, "page.analytics");
    let score_label = t.t(&loc, "label.score");
    let subject_label = t.t(&loc, "label.subject");
    let difficulty_label = t.t(&loc, "label.difficulty");

    rsx! {
        div { class: "page page-analytics",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    match &*analytics_resource.read() {
                        Some(Ok(analytics)) => rsx! {
                            // Overall score
                            div { class: "analytics-overall",
                                div { class: "analytics-score-card",
                                    h3 { if loc == "zh" { "\u{603b}\u{4f53}\u{5f97}\u{5206}" } else { "Overall Score" } }
                                    div { class: "score-display score-display-large",
                                        "{analytics.overall_score:.1}%"
                                    }
                                }
                            }

                            // Per-subject breakdown
                            div { class: "analytics-section",
                                h3 { class: "analytics-section-title",
                                    if loc == "zh" { "\u{6309}\u{79d1}\u{76ee}\u{5206}\u{6790}" } else { "By Subject" }
                                }
                                if analytics.by_subject.is_empty() {
                                    p { class: "empty-text",
                                        if loc == "zh" { "\u{6682}\u{65e0}\u{6570}\u{636e}" } else { "No data yet" }
                                    }
                                } else {
                                    div { class: "analytics-table-wrapper",
                                        table { class: "analytics-table",
                                            thead {
                                                tr {
                                                    th { "{subject_label}" }
                                                    th { "{score_label}" }
                                                    th { if loc == "zh" { "\u{6b21}\u{6570}" } else { "Attempts" } }
                                                }
                                            }
                                            tbody {
                                                for subject in analytics.by_subject.iter() {
                                                    tr {
                                                        td { "{subject.subject_name}" }
                                                        td { class: "analytics-score-cell", "{subject.avg_score:.1}%" }
                                                        td { "{subject.attempt_count}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Per-difficulty breakdown
                            div { class: "analytics-section",
                                h3 { class: "analytics-section-title",
                                    if loc == "zh" { "\u{6309}\u{96be}\u{5ea6}\u{5206}\u{6790}" } else { "By Difficulty" }
                                }
                                if analytics.by_difficulty.is_empty() {
                                    p { class: "empty-text",
                                        if loc == "zh" { "\u{6682}\u{65e0}\u{6570}\u{636e}" } else { "No data yet" }
                                    }
                                } else {
                                    div { class: "analytics-difficulty-cards",
                                        for diff in analytics.by_difficulty.iter() {
                                            div { class: "analytics-diff-card",
                                                h4 { "{diff.difficulty}" }
                                                div { class: "score-display", "{diff.avg_score:.1}%" }
                                                p { class: "diff-attempts",
                                                    "{diff.attempt_count} "
                                                    if loc == "zh" { "\u{6b21}" } else { "attempts" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Recent attempts table
                            div { class: "analytics-section",
                                h3 { class: "analytics-section-title",
                                    if loc == "zh" { "\u{6700}\u{8fd1}\u{8003}\u{8bd5}" } else { "Recent Attempts" }
                                }
                                if analytics.recent_attempts.is_empty() {
                                    p { class: "empty-text",
                                        if loc == "zh" { "\u{6682}\u{65e0}\u{8bb0}\u{5f55}" } else { "No attempts yet" }
                                    }
                                } else {
                                    div { class: "analytics-table-wrapper",
                                        table { class: "analytics-table",
                                            thead {
                                                tr {
                                                    th { if loc == "zh" { "\u{8003}\u{8bd5}" } else { "Exam" } }
                                                    th { "{score_label}" }
                                                    th { if loc == "zh" { "\u{65e5}\u{671f}" } else { "Date" } }
                                                    th { if loc == "zh" { "\u{65f6}\u{957f}(\u{5206}\u{949f})" } else { "Duration (min)" } }
                                                }
                                            }
                                            tbody {
                                                for attempt in analytics.recent_attempts.iter() {
                                                    tr {
                                                        td { "{attempt.exam_title}" }
                                                        td { class: "analytics-score-cell", "{attempt.score:.1}%" }
                                                        td { "{attempt.date}" }
                                                        td {
                                                            {attempt.duration_minutes.map(|d| format!("{}", d)).unwrap_or_else(|| "-".to_string())}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            div { class: "alert alert-error", "Error: {e}" }
                        },
                        None => rsx! {
                            div { class: "loading-spinner", p { "Loading..." } }
                        },
                    }
                }
            }

            Footer {}
        }
    }
}

// ---------------------------------------------------------------------------
// FavoritesPage
// ---------------------------------------------------------------------------
#[component]
pub fn FavoritesPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let mut refresh_trigger = use_signal(|| 0u32);

    let favs_resource = use_resource(move || {
        let _trigger = refresh_trigger();
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/training/favorites", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<FavoriteQuestionItem>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    let page_title = t.t(&loc, "page.favorites");

    rsx! {
        div { class: "page page-favorites",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    match &*favs_resource.read() {
                        Some(Ok(favorites)) => {
                            if favorites.is_empty() {
                                rsx! {
                                    p { class: "empty-text",
                                        if loc == "zh" { "\u{6682}\u{65e0}\u{6536}\u{85cf}" } else { "No favorites yet" }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "favorites-list",
                                        for fav in favorites.iter() {
                                            {
                                                let q_id = fav.question_id;
                                                let q_text = if loc == "zh" {
                                                    fav.question_text_zh.as_deref().unwrap_or(&fav.question_text_en)
                                                } else {
                                                    &fav.question_text_en
                                                };
                                                let is_fav = true; // all items on favorites page are favorited
                                                rsx! {
                                                    div { class: "favorite-card",
                                                        div { class: "favorite-content",
                                                            p { class: "favorite-question-text", "{q_text}" }
                                                        }
                                                        button {
                                                            class: if is_fav { "btn btn-favorite btn-favorite-active" } else { "btn btn-favorite" },
                                                            onclick: move |_| {
                                                                let session_cookie = app_state().auth.session_cookie.clone();
                                                                spawn(async move {
                                                                    let method = if is_fav { "DELETE" } else { "POST" };
                                                                    let url = format!("{}/training/favorites/{}", crate::API_BASE, q_id);
                                                                    let client = reqwest::Client::new();
                                                                    let mut req = if is_fav {
                                                                        client.delete(&url)
                                                                    } else {
                                                                        client.post(&url)
                                                                    };
                                                                    if let Some(ref sc) = session_cookie {
                                                                        req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                                    }
                                                                    let _ = req.send().await;
                                                                    refresh_trigger.set(refresh_trigger() + 1);
                                                                });
                                                            },
                                                            if is_fav { "\u{2605}" } else { "\u{2606}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            div { class: "alert alert-error", "Error: {e}" }
                        },
                        None => rsx! {
                            div { class: "loading-spinner", p { "Loading..." } }
                        },
                    }
                }
            }

            Footer {}
        }
    }
}

// ---------------------------------------------------------------------------
// WrongNotebookPage
// ---------------------------------------------------------------------------
#[component]
pub fn WrongNotebookPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let notebook_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/training/wrong-notebook", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<WrongAnswerReviewSession> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    let page_title = t.t(&loc, "page.wrong_notebook");

    rsx! {
        div { class: "page page-wrong-notebook",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    match &*notebook_resource.read() {
                        Some(Ok(session)) => {
                            if session.questions.is_empty() {
                                rsx! {
                                    p { class: "empty-text",
                                        if loc == "zh" { "\u{6682}\u{65e0}\u{9519}\u{9898}\u{8bb0}\u{5f55}" } else { "No wrong answers recorded" }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "wrong-notebook-list",
                                        for entry in session.questions.iter() {
                                            {
                                                let q_text = if loc == "zh" {
                                                    entry.question_text_zh.as_deref().unwrap_or(&entry.question_text_en)
                                                } else {
                                                    &entry.question_text_en
                                                };
                                                rsx! {
                                                    div { class: "wrong-entry-card",
                                                        div { class: "wrong-entry-content",
                                                            p { class: "wrong-entry-question", "{q_text}" }
                                                        }
                                                        div { class: "wrong-entry-meta",
                                                            div { class: "wrong-entry-stat",
                                                                span { class: "wrong-entry-label",
                                                                    if loc == "zh" { "\u{9519}\u{8bef}\u{6b21}\u{6570}" } else { "Wrong count" }
                                                                }
                                                                span { class: "wrong-entry-count wrong-count-badge", "{entry.wrong_count}" }
                                                            }
                                                            div { class: "wrong-entry-stat",
                                                                span { class: "wrong-entry-label",
                                                                    if loc == "zh" { "\u{4e0a}\u{6b21}\u{9519}\u{8bef}" } else { "Last wrong" }
                                                                }
                                                                span { "{entry.last_wrong_at}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            div { class: "alert alert-error", "Error: {e}" }
                        },
                        None => rsx! {
                            div { class: "loading-spinner", p { "Loading..." } }
                        },
                    }
                }
            }

            Footer {}
        }
    }
}

// ---------------------------------------------------------------------------
// ReviewSessionPage
// ---------------------------------------------------------------------------
#[component]
pub fn ReviewSessionPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let mut current_index = use_signal(|| 0usize);
    let mut selected_options = use_signal(|| Vec::<i64>::new());
    let mut answered = use_signal(|| false);
    let mut is_correct = use_signal(|| false);

    let review_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/training/review-session", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<WrongAnswerReviewSession> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    let page_title = t.t(&loc, "btn.review");

    rsx! {
        div { class: "page page-review-session",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title",
                        if loc == "zh" { "\u{667a}\u{80fd}\u{590d}\u{4e60}" } else { "Smart Review" }
                    }

                    match &*review_resource.read() {
                        Some(Ok(session)) => {
                            let total = session.questions.len();
                            let idx = current_index();

                            if total == 0 {
                                rsx! {
                                    div { class: "empty-state",
                                        p { class: "empty-text",
                                            if loc == "zh" { "\u{6682}\u{65e0}\u{9700}\u{8981}\u{590d}\u{4e60}\u{7684}\u{9898}\u{76ee}" } else { "No questions due for review" }
                                        }
                                        Link {
                                            to: crate::Route::Training { locale: locale.clone() },
                                            class: "btn btn-primary",
                                            if loc == "zh" { "\u{8fd4}\u{56de}" } else { "Back" }
                                        }
                                    }
                                }
                            } else if idx >= total {
                                rsx! {
                                    div { class: "review-complete",
                                        h3 {
                                            if loc == "zh" { "\u{590d}\u{4e60}\u{5b8c}\u{6210}\u{ff01}" } else { "Review Complete!" }
                                        }
                                        p {
                                            if loc == "zh" {
                                                "\u{5df2}\u{590d}\u{4e60} {total} \u{9053}\u{9898}\u{76ee}"
                                            } else {
                                                "Reviewed {total} questions"
                                            }
                                        }
                                        Link {
                                            to: crate::Route::Training { locale: locale.clone() },
                                            class: "btn btn-primary",
                                            if loc == "zh" { "\u{8fd4}\u{56de}\u{57f9}\u{8bad}" } else { "Back to Training" }
                                        }
                                    }
                                }
                            } else {
                                let question = &session.questions[idx];
                                let q_text = if loc == "zh" {
                                    question.question_text_zh.as_deref().unwrap_or(&question.question_text_en)
                                } else {
                                    &question.question_text_en
                                };
                                let is_multi = question.question_type == "multiple_choice";
                                let q_id = question.question_id;
                                let wrong_count = question.wrong_count;

                                rsx! {
                                    // Progress
                                    div { class: "review-header",
                                        span { class: "review-progress", "{idx + 1} / {total}" }
                                        span { class: "review-wrong-count",
                                            if loc == "zh" { "\u{9519}\u{8bef}\u{6b21}\u{6570}: {wrong_count}" } else { "Wrong count: {wrong_count}" }
                                        }
                                    }

                                    div { class: "review-question-card",
                                        p { class: "review-question-text", "{q_text}" }

                                        div { class: "review-options",
                                            for opt in question.options.iter() {
                                                {
                                                    let opt_id = opt.id;
                                                    let is_selected = selected_options().contains(&opt_id);
                                                    let opt_content = if loc == "zh" {
                                                        opt.content_zh.as_deref().unwrap_or(&opt.content_en)
                                                    } else {
                                                        &opt.content_en
                                                    };

                                                    let btn_class = if is_selected {
                                                        "exam-option exam-option-selected"
                                                    } else {
                                                        "exam-option"
                                                    };

                                                    rsx! {
                                                        button {
                                                            class: "{btn_class}",
                                                            disabled: answered(),
                                                            onclick: move |_| {
                                                                let mut sel = selected_options.write();
                                                                if is_multi {
                                                                    if let Some(pos) = sel.iter().position(|&id| id == opt_id) {
                                                                        sel.remove(pos);
                                                                    } else {
                                                                        sel.push(opt_id);
                                                                    }
                                                                } else {
                                                                    *sel = vec![opt_id];
                                                                }
                                                            },
                                                            span { class: "exam-option-label", "{opt.label}. " }
                                                            span { class: "exam-option-content", "{opt_content}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Submit answer / show result
                                        if !answered() {
                                            {
                                                let locale_answer = locale.clone();
                                                rsx! {
                                                    button {
                                                        class: "btn btn-primary btn-block",
                                                        disabled: selected_options().is_empty(),
                                                        onclick: move |_| {
                                                            let session_cookie = app_state().auth.session_cookie.clone();
                                                            let opts = selected_options().clone();
                                                            spawn(async move {
                                                                let body = SubmitAnswerRequest {
                                                                    attempt_id: None, // review-mode: no formal exam attempt
                                                                    question_id: q_id,
                                                                    selected_option_ids: opts,
                                                                };
                                                                let mut req = reqwest::Client::new()
                                                                    .post(&format!("{}/training/answer", crate::API_BASE))
                                                                    .json(&body);
                                                                if let Some(ref sc) = session_cookie {
                                                                    req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                                }
                                                                match req.send().await {
                                                                    Ok(resp) => {
                                                                        if let Ok(api) = resp.json::<ApiResponse<SubmitAnswerResponse>>().await {
                                                                            is_correct.set(api.data.map(|r| r.is_correct).unwrap_or(false));
                                                                        }
                                                                    }
                                                                    Err(_) => {}
                                                                }
                                                                answered.set(true);
                                                            });
                                                        },
                                                        if loc == "zh" { "\u{63d0}\u{4ea4}\u{7b54}\u{6848}" } else { "Submit Answer" }
                                                    }
                                                }
                                            }
                                        } else {
                                            // Show correct/incorrect feedback
                                            div { class: if is_correct() { "review-feedback review-correct" } else { "review-feedback review-incorrect" },
                                                h4 {
                                                    if is_correct() {
                                                        if loc == "zh" { "\u{2713} \u{56de}\u{7b54}\u{6b63}\u{786e}\u{ff01}" } else { "\u{2713} Correct!" }
                                                    } else {
                                                        if loc == "zh" { "\u{2717} \u{56de}\u{7b54}\u{9519}\u{8bef}" } else { "\u{2717} Incorrect" }
                                                    }
                                                }
                                            }

                                            button {
                                                class: "btn btn-primary btn-block",
                                                onclick: move |_| {
                                                    current_index.set(current_index() + 1);
                                                    selected_options.set(Vec::new());
                                                    answered.set(false);
                                                    is_correct.set(false);
                                                },
                                                if loc == "zh" { "\u{4e0b}\u{4e00}\u{9898}" } else { "Next Question" }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            div { class: "alert alert-error", "Error: {e}" }
                        },
                        None => rsx! {
                            div { class: "loading-spinner", p { "Loading..." } }
                        },
                    }
                }
            }

            Footer {}
        }
    }
}
