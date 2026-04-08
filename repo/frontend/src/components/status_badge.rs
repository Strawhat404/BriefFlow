use dioxus::prelude::*;

/// Renders a colored badge/pill for an order status.
/// Recognized statuses: Pending, Accepted, InPrep, Ready, PickedUp, Canceled, Held, Confirmed, Expired.
#[component]
pub fn StatusBadge(status: String, locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.as_str();

    let (badge_class, i18n_key) = match status.as_str() {
        "Pending" => ("status-badge status-pending", "status.pending"),
        "Accepted" => ("status-badge status-accepted", "status.accepted"),
        "InPrep" => ("status-badge status-in-prep", "status.in_prep"),
        "Ready" => ("status-badge status-ready", "status.ready"),
        "PickedUp" => ("status-badge status-picked-up", "status.picked_up"),
        "Canceled" => ("status-badge status-canceled", "status.canceled"),
        "Held" => ("status-badge status-held", "status.held"),
        "Confirmed" => ("status-badge status-confirmed", "status.confirmed"),
        "Expired" => ("status-badge status-expired", "status.expired"),
        _ => ("status-badge", ""),
    };

    let label = if i18n_key.is_empty() {
        status.clone()
    } else {
        t.t(loc, i18n_key)
    };

    rsx! {
        span { class: "{badge_class}", "{label}" }
    }
}
