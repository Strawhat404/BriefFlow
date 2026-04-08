use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::state::AppState;
use shared::dto::{
    ApiResponse, ExamVersionResponse, GenerateExamRequest, ImportQuestionsRequest,
    ImportQuestionsResponse, PaginatedResponse,
};

// ---------------------------------------------------------------------------
// Local DTOs for question bank listing
// ---------------------------------------------------------------------------
#[derive(serde::Deserialize, Clone, Debug)]
struct QuestionListItem {
    id: i64,
    question_text_en: String,
    question_text_zh: Option<String>,
    question_type: String,
    difficulty: String,
    subject_name: Option<String>,
    chapter_name: Option<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct SubjectOption {
    id: i64,
    name_en: String,
    name_zh: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct ChapterOption {
    id: i64,
    subject_id: i64,
    name_en: String,
    name_zh: String,
}

// ---------------------------------------------------------------------------
// AdminPage (Hub)
// ---------------------------------------------------------------------------
#[component]
pub fn AdminPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();

    let page_title = t.t(&loc, "nav.admin");

    rsx! {
        div { class: "page page-admin",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    div { class: "admin-hub-grid",
                        Link {
                            to: crate::Route::QuestionBank { locale: locale.clone() },
                            class: "admin-hub-card",
                            div { class: "admin-hub-icon", "\u{1f4da}" }
                            h3 { "{t.t(&loc, \"page.question_bank\")}" }
                            p { if loc == "zh" { "\u{7ba1}\u{7406}\u{9898}\u{5e93}" } else { "Manage question bank" } }
                        }
                        Link {
                            to: crate::Route::ImportQuestions { locale: locale.clone() },
                            class: "admin-hub-card",
                            div { class: "admin-hub-icon", "\u{1f4e5}" }
                            h3 { "{t.t(&loc, \"btn.import\")}" }
                            p { if loc == "zh" { "\u{4ece}CSV\u{5bfc}\u{5165}\u{9898}\u{76ee}" } else { "Import questions from CSV" } }
                        }
                        Link {
                            to: crate::Route::GenerateExam { locale: locale.clone() },
                            class: "admin-hub-card",
                            div { class: "admin-hub-icon", "\u{2699}" }
                            h3 { "{t.t(&loc, \"btn.generate\")}" }
                            p { if loc == "zh" { "\u{751f}\u{6210}\u{65b0}\u{8003}\u{8bd5}" } else { "Generate new exam" } }
                        }
                        div { class: "admin-hub-card admin-hub-card-info",
                            div { class: "admin-hub-icon", "\u{1f465}" }
                            h3 { if loc == "zh" { "\u{7528}\u{6237}\u{7ba1}\u{7406}" } else { "User Management" } }
                            p { if loc == "zh" { "\u{67e5}\u{770b}\u{7528}\u{6237}\u{6982}\u{89c8}" } else { "View user overview" } }
                        }
                    }
                }
            }

            Footer {}
        }
    }
}

// ---------------------------------------------------------------------------
// QuestionBankPage
// ---------------------------------------------------------------------------
#[component]
pub fn QuestionBankPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let mut search_query = use_signal(|| String::new());
    let mut subject_filter = use_signal(|| Option::<i64>::None);
    let mut difficulty_filter = use_signal(|| String::new());
    let mut current_page = use_signal(|| 1i32);
    let per_page = 20;

    // Load subjects for filter dropdown
    let subjects_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/exam/subjects", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<SubjectOption>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    // Load questions with filters and pagination
    let questions_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        let page = current_page();
        let search = search_query();
        let subject = subject_filter();
        let difficulty = difficulty_filter();
        async move {
            let mut url = format!(
                "{}/exam/questions?page={}&per_page={}",
                crate::API_BASE, page, per_page
            );
            if !search.is_empty() {
                url.push_str(&format!("&q={}", search));
            }
            if let Some(sid) = subject {
                url.push_str(&format!("&subject_id={}", sid));
            }
            if !difficulty.is_empty() {
                url.push_str(&format!("&difficulty={}", difficulty));
            }

            let mut req = reqwest::Client::new().get(&url);
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<PaginatedResponse<QuestionListItem>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    let page_title = t.t(&loc, "page.question_bank");
    let subject_label = t.t(&loc, "label.subject");
    let difficulty_label = t.t(&loc, "label.difficulty");
    let all_label = if loc == "zh" { "\u{5168}\u{90e8}" } else { "All" };
    let search_placeholder = if loc == "zh" { "\u{641c}\u{7d22}\u{9898}\u{76ee}..." } else { "Search questions..." };

    rsx! {
        div { class: "page page-question-bank",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    // Filter bar
                    div { class: "qb-filters",
                        // Search input
                        div { class: "form-group",
                            input {
                                r#type: "text",
                                class: "form-input",
                                placeholder: "{search_placeholder}",
                                value: "{search_query}",
                                oninput: move |evt| {
                                    search_query.set(evt.value());
                                    current_page.set(1);
                                },
                            }
                        }

                        // Subject filter
                        div { class: "form-group",
                            label { "{subject_label}" }
                            select {
                                class: "form-select",
                                onchange: move |evt| {
                                    let val = evt.value();
                                    subject_filter.set(val.parse::<i64>().ok());
                                    current_page.set(1);
                                },
                                option { value: "", "{all_label}" }
                                if let Some(Ok(subjects)) = &*subjects_resource.read() {
                                    for s in subjects.iter() {
                                        {
                                            let name = if loc == "zh" { &s.name_zh } else { &s.name_en };
                                            rsx! {
                                                option { value: "{s.id}", "{name}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Difficulty filter
                        div { class: "form-group",
                            label { "{difficulty_label}" }
                            select {
                                class: "form-select",
                                onchange: move |evt| {
                                    difficulty_filter.set(evt.value());
                                    current_page.set(1);
                                },
                                option { value: "", "{all_label}" }
                                option { value: "easy", "Easy" }
                                option { value: "medium", "Medium" }
                                option { value: "hard", "Hard" }
                            }
                        }
                    }

                    // Questions table
                    match &*questions_resource.read() {
                        Some(Ok(paginated)) => {
                            let total_pages = ((paginated.total as f64) / (per_page as f64)).ceil() as i32;
                            rsx! {
                                div { class: "qb-table-wrapper",
                                    table { class: "qb-table",
                                        thead {
                                            tr {
                                                th { "ID" }
                                                th { if loc == "zh" { "\u{9898}\u{76ee}" } else { "Question" } }
                                                th { if loc == "zh" { "\u{7c7b}\u{578b}" } else { "Type" } }
                                                th { "{difficulty_label}" }
                                                th { "{subject_label}" }
                                            }
                                        }
                                        tbody {
                                            for q in paginated.items.iter() {
                                                {
                                                    let q_text = if loc == "zh" {
                                                        q.question_text_zh.as_deref().unwrap_or(&q.question_text_en)
                                                    } else {
                                                        &q.question_text_en
                                                    };
                                                    // Truncate long text
                                                    let preview = if q_text.len() > 80 {
                                                        format!("{}...", &q_text[..80])
                                                    } else {
                                                        q_text.to_string()
                                                    };
                                                    let subject = q.subject_name.as_deref().unwrap_or("-");
                                                    rsx! {
                                                        tr {
                                                            td { "{q.id}" }
                                                            td { class: "qb-question-preview", "{preview}" }
                                                            td { "{q.question_type}" }
                                                            td { "{q.difficulty}" }
                                                            td { "{subject}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Pagination
                                div { class: "pagination",
                                    span { class: "pagination-info",
                                        if loc == "zh" {
                                            "\u{5171} {paginated.total} \u{9898}"
                                        } else {
                                            "{paginated.total} questions total"
                                        }
                                    }
                                    div { class: "pagination-controls",
                                        button {
                                            class: "btn btn-sm",
                                            disabled: current_page() <= 1,
                                            onclick: move |_| current_page.set(current_page() - 1),
                                            if loc == "zh" { "\u{4e0a}\u{4e00}\u{9875}" } else { "Prev" }
                                        }
                                        span { class: "pagination-current",
                                            "{current_page()} / {total_pages}"
                                        }
                                        button {
                                            class: "btn btn-sm",
                                            disabled: current_page() >= total_pages,
                                            onclick: move |_| current_page.set(current_page() + 1),
                                            if loc == "zh" { "\u{4e0b}\u{4e00}\u{9875}" } else { "Next" }
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
// ImportQuestionsPage
// ---------------------------------------------------------------------------
#[component]
pub fn ImportQuestionsPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let mut selected_subject = use_signal(|| Option::<i64>::None);
    let mut selected_chapter = use_signal(|| Option::<i64>::None);
    let mut csv_content = use_signal(|| String::new());
    let mut import_result = use_signal(|| Option::<ImportQuestionsResponse>::None);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut importing = use_signal(|| false);

    // Load subjects
    let subjects_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/exam/subjects", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<SubjectOption>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    // Load chapters based on selected subject
    let chapters_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        let subject_id = selected_subject();
        async move {
            let Some(sid) = subject_id else {
                return Ok(Vec::<ChapterOption>::new());
            };
            let mut req = reqwest::Client::new()
                .get(&format!("{}/exam/subjects/{}/chapters", crate::API_BASE, sid));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<ChapterOption>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    let page_title = if loc == "zh" { "\u{5bfc}\u{5165}\u{9898}\u{76ee}" } else { "Import Questions" };
    let subject_label = t.t(&loc, "label.subject");
    let chapter_label = t.t(&loc, "label.chapter");
    let import_text = t.t(&loc, "btn.import");

    let csv_format_help = if loc == "zh" {
        "CSV\u{683c}\u{5f0f}: question_text_en,question_text_zh,question_type,difficulty,option_a,option_b,option_c,option_d,correct_answer,explanation"
    } else {
        "CSV format: question_text_en,question_text_zh,question_type,difficulty,option_a,option_b,option_c,option_d,correct_answer,explanation"
    };

    rsx! {
        div { class: "page page-import-questions",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    if let Some(err) = error_msg() {
                        div { class: "alert alert-error", "{err}" }
                    }

                    // Import result display
                    if let Some(result) = import_result() {
                        div { class: "import-result-card",
                            h3 { if loc == "zh" { "\u{5bfc}\u{5165}\u{7ed3}\u{679c}" } else { "Import Results" } }
                            div { class: "import-stats",
                                div { class: "import-stat import-stat-success",
                                    span { class: "import-stat-count", "{result.imported_count}" }
                                    span { class: "import-stat-label",
                                        if loc == "zh" { "\u{5df2}\u{5bfc}\u{5165}" } else { "Imported" }
                                    }
                                }
                                div { class: "import-stat import-stat-skipped",
                                    span { class: "import-stat-count", "{result.skipped_count}" }
                                    span { class: "import-stat-label",
                                        if loc == "zh" { "\u{5df2}\u{8df3}\u{8fc7}" } else { "Skipped" }
                                    }
                                }
                                if !result.errors.is_empty() {
                                    div { class: "import-stat import-stat-errors",
                                        span { class: "import-stat-count", "{result.errors.len()}" }
                                        span { class: "import-stat-label",
                                            if loc == "zh" { "\u{9519}\u{8bef}" } else { "Errors" }
                                        }
                                    }
                                }
                            }
                            if !result.errors.is_empty() {
                                div { class: "import-errors",
                                    h4 { if loc == "zh" { "\u{9519}\u{8bef}\u{8be6}\u{60c5}" } else { "Error Details" } }
                                    ul { class: "import-error-list",
                                        for err in result.errors.iter() {
                                            li { class: "import-error-item", "{err}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Import form
                    div { class: "import-form-card",
                        form {
                            class: "import-form",
                            onsubmit: move |evt| {
                                evt.prevent_default();
                                let Some(sid) = selected_subject() else {
                                    error_msg.set(Some("Please select a subject".to_string()));
                                    return;
                                };
                                let csv = csv_content().clone();
                                if csv.is_empty() {
                                    error_msg.set(Some("CSV content is empty".to_string()));
                                    return;
                                }
                                let chapter = selected_chapter();
                                let session_cookie = app_state().auth.session_cookie.clone();
                                spawn(async move {
                                    importing.set(true);
                                    error_msg.set(None);
                                    import_result.set(None);

                                    let body = ImportQuestionsRequest {
                                        subject_id: sid,
                                        chapter_id: chapter,
                                        csv_content: csv,
                                    };

                                    let mut req = reqwest::Client::new()
                                        .post(&format!("{}/exam/questions/import", crate::API_BASE))
                                        .json(&body);
                                    if let Some(ref sc) = session_cookie {
                                        req = req.header("Cookie", format!("brewflow_session={}", sc));
                                    }

                                    match req.send().await {
                                        Ok(resp) => {
                                            if resp.status().is_success() {
                                                match resp.json::<ApiResponse<ImportQuestionsResponse>>().await {
                                                    Ok(api) => {
                                                        if let Some(data) = api.data {
                                                            import_result.set(Some(data));
                                                        } else {
                                                            error_msg.set(Some(api.error.unwrap_or_else(|| "Import failed".to_string())));
                                                        }
                                                    }
                                                    Err(e) => error_msg.set(Some(format!("Parse error: {}", e))),
                                                }
                                            } else {
                                                let body = resp.text().await.unwrap_or_default();
                                                error_msg.set(Some(format!("Import failed: {}", body)));
                                            }
                                        }
                                        Err(e) => error_msg.set(Some(format!("Network error: {}", e))),
                                    }
                                    importing.set(false);
                                });
                            },

                            // Subject selector
                            div { class: "form-group",
                                label { "{subject_label} *" }
                                select {
                                    class: "form-select",
                                    required: true,
                                    onchange: move |evt| {
                                        selected_subject.set(evt.value().parse::<i64>().ok());
                                        selected_chapter.set(None);
                                    },
                                    option { value: "",
                                        if loc == "zh" { "\u{9009}\u{62e9}\u{79d1}\u{76ee}" } else { "Select subject" }
                                    }
                                    if let Some(Ok(subjects)) = &*subjects_resource.read() {
                                        for s in subjects.iter() {
                                            {
                                                let name = if loc == "zh" { &s.name_zh } else { &s.name_en };
                                                rsx! {
                                                    option { value: "{s.id}", "{name}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Chapter selector
                            div { class: "form-group",
                                label { "{chapter_label}" }
                                select {
                                    class: "form-select",
                                    onchange: move |evt| {
                                        selected_chapter.set(evt.value().parse::<i64>().ok());
                                    },
                                    option { value: "",
                                        if loc == "zh" { "\u{9009}\u{62e9}\u{7ae0}\u{8282}(\u{53ef}\u{9009})" } else { "Select chapter (optional)" }
                                    }
                                    if let Some(Ok(chapters)) = &*chapters_resource.read() {
                                        for c in chapters.iter() {
                                            {
                                                let name = if loc == "zh" { &c.name_zh } else { &c.name_en };
                                                rsx! {
                                                    option { value: "{c.id}", "{name}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // CSV import: local file upload OR paste
                            div { class: "form-group",
                                label { "CSV File" }
                                p { class: "form-help-text",
                                    "Upload a local CSV file or paste content directly below."
                                }
                                input {
                                    r#type: "file",
                                    accept: ".csv,text/csv",
                                    class: "form-input",
                                    onchange: move |evt| {
                                        if let Some(file_engine) = evt.files() {
                                            let names = file_engine.files();
                                            if let Some(name) = names.into_iter().next() {
                                                spawn(async move {
                                                    if let Some(content) =
                                                        file_engine.read_file_to_string(&name).await
                                                    {
                                                        csv_content.set(content);
                                                    }
                                                });
                                            }
                                        }
                                    },
                                }
                            }
                            div { class: "form-group",
                                label { "CSV Content" }
                                p { class: "form-help-text", "{csv_format_help}" }
                                textarea {
                                    class: "form-input form-textarea form-textarea-lg",
                                    placeholder: "question_text_en,question_text_zh,single_choice,easy,Option A,...",
                                    rows: "12",
                                    value: "{csv_content}",
                                    oninput: move |evt| csv_content.set(evt.value()),
                                }
                            }

                            button {
                                r#type: "submit",
                                class: "btn btn-primary btn-lg",
                                disabled: importing(),
                                if importing() { "..." } else { "{import_text}" }
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
// GenerateExamPage
// ---------------------------------------------------------------------------
#[component]
pub fn GenerateExamPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let mut title_en = use_signal(|| String::new());
    let mut title_zh = use_signal(|| String::new());
    let mut selected_subject = use_signal(|| Option::<i64>::None);
    let mut selected_chapter = use_signal(|| Option::<i64>::None);
    let mut selected_difficulty = use_signal(|| String::new());
    let mut question_count = use_signal(|| 20i32);
    let mut time_limit = use_signal(|| 60i32);
    let mut generated_exam = use_signal(|| Option::<ExamVersionResponse>::None);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut generating = use_signal(|| false);

    // Load subjects
    let subjects_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/exam/subjects", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<SubjectOption>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    // Load chapters
    let chapters_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        let subject_id = selected_subject();
        async move {
            let Some(sid) = subject_id else {
                return Ok(Vec::<ChapterOption>::new());
            };
            let mut req = reqwest::Client::new()
                .get(&format!("{}/exam/subjects/{}/chapters", crate::API_BASE, sid));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<ChapterOption>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    let page_title = if loc == "zh" { "\u{751f}\u{6210}\u{8003}\u{8bd5}" } else { "Generate Exam" };
    let subject_label = t.t(&loc, "label.subject");
    let chapter_label = t.t(&loc, "label.chapter");
    let difficulty_label = t.t(&loc, "label.difficulty");
    let time_limit_label = t.t(&loc, "label.time_limit");
    let generate_text = t.t(&loc, "btn.generate");

    rsx! {
        div { class: "page page-generate-exam",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    if let Some(err) = error_msg() {
                        div { class: "alert alert-error", "{err}" }
                    }

                    // Generated exam summary
                    if let Some(exam) = generated_exam() {
                        div { class: "generated-exam-card",
                            h3 {
                                if loc == "zh" { "\u{8003}\u{8bd5}\u{5df2}\u{751f}\u{6210}\u{ff01}" } else { "Exam Generated!" }
                            }
                            div { class: "generated-exam-details",
                                p {
                                    if loc == "zh" { "\u{6807}\u{9898}: " } else { "Title: " }
                                    strong {
                                        {if loc == "zh" {
                                            exam.title_zh.as_deref().unwrap_or(&exam.title_en)
                                        } else {
                                            &exam.title_en
                                        }}
                                    }
                                }
                                p {
                                    "{difficulty_label}: "
                                    strong { "{exam.difficulty}" }
                                }
                                p {
                                    if loc == "zh" { "\u{9898}\u{6570}: " } else { "Questions: " }
                                    strong { "{exam.question_count}" }
                                }
                                p {
                                    "{time_limit_label}: "
                                    strong { "{exam.time_limit_minutes} min" }
                                }
                                if let Some(ref subject) = exam.subject_name {
                                    p {
                                        "{subject_label}: "
                                        strong { "{subject}" }
                                    }
                                }
                            }
                        }
                    }

                    // Generate form
                    div { class: "generate-form-card",
                        form {
                            class: "generate-form",
                            onsubmit: move |evt| {
                                evt.prevent_default();
                                let te = title_en().clone();
                                if te.is_empty() {
                                    error_msg.set(Some("Title is required".to_string()));
                                    return;
                                }
                                let tz = title_zh().clone();
                                let sid = selected_subject();
                                let cid = selected_chapter();
                                let diff = selected_difficulty().clone();
                                let qc = question_count();
                                let tl = time_limit();
                                let session_cookie = app_state().auth.session_cookie.clone();
                                spawn(async move {
                                    generating.set(true);
                                    error_msg.set(None);
                                    generated_exam.set(None);

                                    let body = GenerateExamRequest {
                                        title_en: te,
                                        title_zh: if tz.is_empty() { None } else { Some(tz) },
                                        subject_id: sid,
                                        chapter_id: cid,
                                        difficulty: if diff.is_empty() { None } else { Some(diff) },
                                        question_count: qc,
                                        time_limit_minutes: tl,
                                    };

                                    let mut req = reqwest::Client::new()
                                        .post(&format!("{}/exam/generate", crate::API_BASE))
                                        .json(&body);
                                    if let Some(ref sc) = session_cookie {
                                        req = req.header("Cookie", format!("brewflow_session={}", sc));
                                    }

                                    match req.send().await {
                                        Ok(resp) => {
                                            if resp.status().is_success() {
                                                match resp.json::<ApiResponse<ExamVersionResponse>>().await {
                                                    Ok(api) => {
                                                        if let Some(data) = api.data {
                                                            generated_exam.set(Some(data));
                                                        } else {
                                                            error_msg.set(Some(api.error.unwrap_or_else(|| "Generation failed".to_string())));
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
                                    generating.set(false);
                                });
                            },

                            // Title EN
                            div { class: "form-group",
                                label { "Title (EN) *" }
                                input {
                                    r#type: "text",
                                    class: "form-input",
                                    placeholder: "Exam title in English",
                                    required: true,
                                    value: "{title_en}",
                                    oninput: move |evt| title_en.set(evt.value()),
                                }
                            }

                            // Title ZH
                            div { class: "form-group",
                                label { "Title (ZH)" }
                                input {
                                    r#type: "text",
                                    class: "form-input",
                                    placeholder: "\u{8003}\u{8bd5}\u{6807}\u{9898}",
                                    value: "{title_zh}",
                                    oninput: move |evt| title_zh.set(evt.value()),
                                }
                            }

                            // Subject
                            div { class: "form-group",
                                label { "{subject_label}" }
                                select {
                                    class: "form-select",
                                    onchange: move |evt| {
                                        selected_subject.set(evt.value().parse::<i64>().ok());
                                        selected_chapter.set(None);
                                    },
                                    option { value: "",
                                        if loc == "zh" { "\u{5168}\u{90e8}\u{79d1}\u{76ee}" } else { "All subjects" }
                                    }
                                    if let Some(Ok(subjects)) = &*subjects_resource.read() {
                                        for s in subjects.iter() {
                                            {
                                                let name = if loc == "zh" { &s.name_zh } else { &s.name_en };
                                                rsx! {
                                                    option { value: "{s.id}", "{name}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Chapter
                            div { class: "form-group",
                                label { "{chapter_label}" }
                                select {
                                    class: "form-select",
                                    onchange: move |evt| {
                                        selected_chapter.set(evt.value().parse::<i64>().ok());
                                    },
                                    option { value: "",
                                        if loc == "zh" { "\u{5168}\u{90e8}\u{7ae0}\u{8282}" } else { "All chapters" }
                                    }
                                    if let Some(Ok(chapters)) = &*chapters_resource.read() {
                                        for c in chapters.iter() {
                                            {
                                                let name = if loc == "zh" { &c.name_zh } else { &c.name_en };
                                                rsx! {
                                                    option { value: "{c.id}", "{name}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Difficulty
                            div { class: "form-group",
                                label { "{difficulty_label}" }
                                select {
                                    class: "form-select",
                                    onchange: move |evt| selected_difficulty.set(evt.value()),
                                    option { value: "",
                                        if loc == "zh" { "\u{6df7}\u{5408}" } else { "Mixed" }
                                    }
                                    option { value: "easy", "Easy" }
                                    option { value: "medium", "Medium" }
                                    option { value: "hard", "Hard" }
                                }
                            }

                            // Question count
                            div { class: "form-group",
                                label {
                                    if loc == "zh" { "\u{9898}\u{6570}" } else { "Question Count" }
                                }
                                input {
                                    r#type: "number",
                                    class: "form-input",
                                    min: "1",
                                    max: "200",
                                    value: "{question_count}",
                                    oninput: move |evt| {
                                        if let Ok(v) = evt.value().parse::<i32>() {
                                            question_count.set(v);
                                        }
                                    },
                                }
                            }

                            // Time limit
                            div { class: "form-group",
                                label { "{time_limit_label} (min)" }
                                input {
                                    r#type: "number",
                                    class: "form-input",
                                    min: "5",
                                    max: "300",
                                    value: "{time_limit}",
                                    oninput: move |evt| {
                                        if let Ok(v) = evt.value().parse::<i32>() {
                                            time_limit.set(v);
                                        }
                                    },
                                }
                            }

                            button {
                                r#type: "submit",
                                class: "btn btn-primary btn-lg",
                                disabled: generating(),
                                if generating() { "..." } else { "{generate_text}" }
                            }
                        }
                    }
                }
            }

            Footer {}
        }
    }
}
