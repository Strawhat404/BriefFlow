use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::state::AppState;
use shared::dto::{ApiResponse, LoginRequest, LoginResponse};

// ---------------------------------------------------------------------------
// LoginPage
// ---------------------------------------------------------------------------
#[component]
pub fn LoginPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let nav = use_navigator();
    let mut app_state = use_context::<Signal<AppState>>();

    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

    let title = if loc == "zh" { "\u{767b}\u{5f55}" } else { "Login" };
    let username_label = if loc == "zh" { "\u{7528}\u{6237}\u{540d}" } else { "Username" };
    let password_label = if loc == "zh" { "\u{5bc6}\u{7801}" } else { "Password" };
    let submit_text = t.t(&loc, "btn.submit");
    let register_text = if loc == "zh" { "\u{6ca1}\u{6709}\u{8d26}\u{53f7}\u{ff1f}\u{6ce8}\u{518c}" } else { "No account? Register" };

    let locale_submit = locale.clone();
    let locale_nav = locale.clone();

    rsx! {
        div { class: "page page-auth",
            Navbar { locale: locale.clone() }

            main { class: "main-content auth-content",
                div { class: "auth-card",
                    h2 { class: "auth-title", "{title}" }

                    if let Some(err) = error_msg() {
                        div { class: "alert alert-error", "{err}" }
                    }

                    form {
                        class: "auth-form",
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            let user = username().clone();
                            let pass = password().clone();
                            let locale_inner = locale_submit.clone();
                            spawn(async move {
                                loading.set(true);
                                error_msg.set(None);

                                let body = LoginRequest {
                                    username: user,
                                    password: pass,
                                };

                                let result = reqwest::Client::new()
                                    .post(&format!("{}/auth/login", crate::API_BASE))
                                    .json(&body)
                                    .send()
                                    .await;

                                match result {
                                    Ok(resp) => {
                                        if resp.status().is_success() {
                                            match resp.json::<ApiResponse<LoginResponse>>().await {
                                                Ok(api_resp) => {
                                                    if let Some(data) = api_resp.data {
                                                        let user_info = crate::state::UserInfo {
                                                            id: data.user.id,
                                                            username: data.user.username,
                                                            display_name: data.user.display_name,
                                                            roles: data.user.roles,
                                                            preferred_locale: data.user.preferred_locale,
                                                        };
                                                        app_state.write().set_auth(data.session_cookie, user_info);
                                                        nav.push(crate::Route::Home { locale: locale_inner });
                                                    } else {
                                                        error_msg.set(Some(api_resp.error.unwrap_or_else(|| "Login failed".to_string())));
                                                    }
                                                }
                                                Err(e) => error_msg.set(Some(format!("Parse error: {}", e))),
                                            }
                                        } else {
                                            let status = resp.status();
                                            let body = resp.text().await.unwrap_or_default();
                                            error_msg.set(Some(format!("Error {}: {}", status, body)));
                                        }
                                    }
                                    Err(e) => error_msg.set(Some(format!("Network error: {}", e))),
                                }

                                loading.set(false);
                            });
                        },

                        div { class: "form-group",
                            label { r#for: "username", "{username_label}" }
                            input {
                                r#type: "text",
                                id: "username",
                                class: "form-input",
                                placeholder: "{username_label}",
                                required: true,
                                value: "{username}",
                                oninput: move |evt| username.set(evt.value()),
                            }
                        }

                        div { class: "form-group",
                            label { r#for: "password", "{password_label}" }
                            input {
                                r#type: "password",
                                id: "password",
                                class: "form-input",
                                placeholder: "{password_label}",
                                required: true,
                                value: "{password}",
                                oninput: move |evt| password.set(evt.value()),
                            }
                        }

                        button {
                            r#type: "submit",
                            class: "btn btn-primary btn-block",
                            disabled: loading(),
                            if loading() { "..." } else { "{submit_text}" }
                        }
                    }

                    div { class: "auth-footer",
                        Link {
                            to: crate::Route::Register { locale: locale_nav.clone() },
                            class: "auth-link",
                            "{register_text}"
                        }
                    }
                }
            }

            Footer {}
        }
    }
}

// ---------------------------------------------------------------------------
// RegisterPage
// ---------------------------------------------------------------------------

#[derive(serde::Serialize)]
struct RegisterRequest {
    username: String,
    password: String,
    display_name: Option<String>,
    email: Option<String>,
}

#[component]
pub fn RegisterPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let nav = use_navigator();

    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut display_name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut success_msg = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

    let title = if loc == "zh" { "\u{6ce8}\u{518c}" } else { "Register" };
    let username_label = if loc == "zh" { "\u{7528}\u{6237}\u{540d}" } else { "Username" };
    let password_label = if loc == "zh" { "\u{5bc6}\u{7801}" } else { "Password" };
    let display_name_label = if loc == "zh" { "\u{663e}\u{793a}\u{540d}\u{79f0}" } else { "Display Name" };
    let email_label = if loc == "zh" { "\u{7535}\u{5b50}\u{90ae}\u{4ef6}" } else { "Email" };
    let submit_text = t.t(&loc, "btn.submit");
    let login_text = if loc == "zh" { "\u{5df2}\u{6709}\u{8d26}\u{53f7}\u{ff1f}\u{767b}\u{5f55}" } else { "Already have an account? Login" };

    let locale_submit = locale.clone();
    let locale_nav = locale.clone();

    rsx! {
        div { class: "page page-auth",
            Navbar { locale: locale.clone() }

            main { class: "main-content auth-content",
                div { class: "auth-card",
                    h2 { class: "auth-title", "{title}" }

                    if let Some(err) = error_msg() {
                        div { class: "alert alert-error", "{err}" }
                    }
                    if let Some(msg) = success_msg() {
                        div { class: "alert alert-success", "{msg}" }
                    }

                    form {
                        class: "auth-form",
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            let user = username().clone();
                            let pass = password().clone();
                            let dname = display_name().clone();
                            let em = email().clone();
                            let locale_inner = locale_submit.clone();
                            spawn(async move {
                                loading.set(true);
                                error_msg.set(None);
                                success_msg.set(None);

                                let body = RegisterRequest {
                                    username: user,
                                    password: pass,
                                    display_name: if dname.is_empty() { None } else { Some(dname) },
                                    email: if em.is_empty() { None } else { Some(em) },
                                };

                                let result = reqwest::Client::new()
                                    .post(&format!("{}/auth/register", crate::API_BASE))
                                    .json(&body)
                                    .send()
                                    .await;

                                match result {
                                    Ok(resp) => {
                                        if resp.status().is_success() {
                                            success_msg.set(Some("Registration successful! Redirecting to login...".to_string()));
                                            // Brief delay then redirect
                                            tokio::time::sleep(std::time::Duration::from_millis(1_500)).await;
                                            nav.push(crate::Route::Login { locale: locale_inner });
                                        } else {
                                            let body = resp.text().await.unwrap_or_default();
                                            error_msg.set(Some(format!("Registration failed: {}", body)));
                                        }
                                    }
                                    Err(e) => error_msg.set(Some(format!("Network error: {}", e))),
                                }

                                loading.set(false);
                            });
                        },

                        div { class: "form-group",
                            label { r#for: "reg-username", "{username_label}" }
                            input {
                                r#type: "text",
                                id: "reg-username",
                                class: "form-input",
                                placeholder: "{username_label}",
                                required: true,
                                value: "{username}",
                                oninput: move |evt| username.set(evt.value()),
                            }
                        }

                        div { class: "form-group",
                            label { r#for: "reg-password", "{password_label}" }
                            input {
                                r#type: "password",
                                id: "reg-password",
                                class: "form-input",
                                placeholder: "{password_label}",
                                required: true,
                                value: "{password}",
                                oninput: move |evt| password.set(evt.value()),
                            }
                        }

                        div { class: "form-group",
                            label { r#for: "reg-display-name", "{display_name_label}" }
                            input {
                                r#type: "text",
                                id: "reg-display-name",
                                class: "form-input",
                                placeholder: "{display_name_label}",
                                value: "{display_name}",
                                oninput: move |evt| display_name.set(evt.value()),
                            }
                        }

                        div { class: "form-group",
                            label { r#for: "reg-email", "{email_label}" }
                            input {
                                r#type: "email",
                                id: "reg-email",
                                class: "form-input",
                                placeholder: "{email_label}",
                                value: "{email}",
                                oninput: move |evt| email.set(evt.value()),
                            }
                        }

                        button {
                            r#type: "submit",
                            class: "btn btn-primary btn-block",
                            disabled: loading(),
                            if loading() { "..." } else { "{submit_text}" }
                        }
                    }

                    div { class: "auth-footer",
                        Link {
                            to: crate::Route::Login { locale: locale_nav.clone() },
                            class: "auth-link",
                            "{login_text}"
                        }
                    }
                }
            }

            Footer {}
        }
    }
}
