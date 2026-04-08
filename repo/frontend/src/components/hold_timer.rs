use dioxus::prelude::*;

/// Displays a countdown timer for a hold/reservation expiry.
/// `expires_at` should be an ISO 8601 datetime string (e.g. "2026-04-04T14:30:00").
/// `locale` is used for translating labels.
#[component]
pub fn HoldTimer(expires_at: String, locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.as_str();

    let mut remaining_secs = use_signal(|| compute_remaining(&expires_at));
    let expires_clone = expires_at.clone();

    // Spawn a tick that updates every second
    use_future(move || {
        let expires = expires_clone.clone();
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                let secs = compute_remaining(&expires);
                remaining_secs.set(secs);
                if secs <= 0 {
                    break;
                }
            }
        }
    });

    let secs = remaining_secs();

    if secs <= 0 {
        let msg = t.t(loc, "msg.item_released");
        rsx! {
            div { class: "hold-timer hold-timer-expired",
                span { class: "hold-timer-warning", "{msg}" }
            }
        }
    } else {
        let minutes = secs / 60;
        let seconds = secs % 60;
        let display = format!("{:02}:{:02}", minutes, seconds);
        let label = t.t(loc, "label.hold_timer");
        let warning_class = if secs < 60 { "hold-timer hold-timer-urgent" } else { "hold-timer" };

        rsx! {
            div { class: "{warning_class}",
                span { class: "hold-timer-label", "{label}: " }
                span { class: "hold-timer-countdown", "{display}" }
            }
        }
    }
}

fn compute_remaining(expires_at: &str) -> i64 {
    // Parse ISO 8601 datetime and compute seconds remaining
    let Ok(expiry) = chrono::NaiveDateTime::parse_from_str(expires_at, "%Y-%m-%dT%H:%M:%S") else {
        // Try with fractional seconds
        let Ok(expiry) = chrono::NaiveDateTime::parse_from_str(expires_at, "%Y-%m-%dT%H:%M:%S%.f") else {
            return 0;
        };
        let now = chrono::Utc::now().naive_utc();
        return (expiry - now).num_seconds();
    };
    let now = chrono::Utc::now().naive_utc();
    (expiry - now).num_seconds()
}
