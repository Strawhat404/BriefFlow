use dioxus::prelude::*;
use shared::dto::PickupSlot;

/// Renders a grid of 15-minute pickup time slots.
/// Available slots are clickable; unavailable slots are grayed out.
/// The selected slot is highlighted. Calls `on_select` with the chosen slot.
#[component]
pub fn SlotPicker(
    slots: Vec<PickupSlot>,
    locale: String,
    on_select: EventHandler<PickupSlot>,
) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.as_str();

    let mut selected_start = use_signal(|| Option::<String>::None);

    let label = t.t(loc, "label.pickup_time");

    rsx! {
        div { class: "slot-picker",
            h3 { class: "slot-picker-title", "{label}" }
            div { class: "slot-picker-grid",
                for slot in slots.iter() {
                    {
                        let is_available = slot.available;
                        let is_selected = selected_start()
                            .as_ref()
                            .map(|s| s == &slot.start)
                            .unwrap_or(false);

                        let slot_class = if !is_available {
                            "slot-cell slot-unavailable"
                        } else if is_selected {
                            "slot-cell slot-selected"
                        } else {
                            "slot-cell slot-available"
                        };

                        // Format time display: show just HH:MM from the ISO string
                        let display_time = format_slot_time(&slot.start);
                        let slot_clone = slot.clone();

                        rsx! {
                            button {
                                class: "{slot_class}",
                                disabled: !is_available,
                                onclick: move |_| {
                                    selected_start.set(Some(slot_clone.start.clone()));
                                    on_select.call(slot_clone.clone());
                                },
                                "{display_time}"
                            }
                        }
                    }
                }
            }
            if slots.is_empty() {
                p { class: "slot-picker-empty",
                    "{t.t(loc, \"error.slot_unavailable\")}"
                }
            }
        }
    }
}

/// Extract a human-readable time (HH:MM) from an ISO-like datetime string.
fn format_slot_time(datetime_str: &str) -> String {
    // Expected format: "2026-04-04T14:30:00" or similar
    if let Some(t_pos) = datetime_str.find('T') {
        let time_part = &datetime_str[t_pos + 1..];
        // Take HH:MM
        if time_part.len() >= 5 {
            return time_part[..5].to_string();
        }
    }
    // Fallback: return as-is
    datetime_str.to_string()
}
