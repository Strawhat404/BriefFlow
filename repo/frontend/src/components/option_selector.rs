use dioxus::prelude::*;
use shared::dto::OptionGroupDetail;

/// Renders product customization option groups (e.g. Size, Milk Type, Sweetness).
/// Each group is displayed with its options as a radio-button group.
/// Calls `on_change` with the currently selected option IDs and the total price delta.
#[component]
pub fn OptionSelector(
    groups: Vec<OptionGroupDetail>,
    locale: String,
    on_change: EventHandler<(Vec<i64>, f64)>,
) -> Element {
    let loc = locale.as_str();
    let is_zh = loc == "zh";

    // Initialize selected options: pick the default (or first) option for each group.
    let initial_selections: Vec<(i64, i64, f64)> = groups
        .iter()
        .filter_map(|g| {
            let default_opt = g.options.iter().find(|o| o.is_default).or(g.options.first());
            default_opt.map(|o| (g.id, o.id, o.price_delta))
        })
        .collect();

    // Store as Vec<(group_id, option_id, price_delta)>
    let mut selections = use_signal(|| initial_selections);

    let total_delta: f64 = selections().iter().map(|(_, _, d)| d).sum();
    let delta_display = format!("+{:.2}", total_delta);

    rsx! {
        div { class: "option-selector",
            for group in groups.iter() {
                {
                    let group_name = if is_zh { &group.name_zh } else { &group.name_en };
                    let required_marker = if group.is_required { " *" } else { "" };
                    let group_id = group.id;

                    rsx! {
                        div { class: "option-group",
                            label { class: "option-group-label",
                                "{group_name}{required_marker}"
                            }
                            div { class: "option-group-choices",
                                for option in group.options.iter() {
                                    {
                                        let opt_id = option.id;
                                        let opt_label = if is_zh {
                                            option.label_zh.clone()
                                        } else {
                                            option.label_en.clone()
                                        };
                                        let opt_delta = option.price_delta;
                                        let delta_text = if opt_delta > 0.0 {
                                            format!(" (+{:.2})", opt_delta)
                                        } else if opt_delta < 0.0 {
                                            format!(" ({:.2})", opt_delta)
                                        } else {
                                            String::new()
                                        };

                                        let is_selected = selections()
                                            .iter()
                                            .any(|(gid, oid, _)| *gid == group_id && *oid == opt_id);

                                        let btn_class = if is_selected {
                                            "option-choice option-choice-selected"
                                        } else {
                                            "option-choice"
                                        };

                                        rsx! {
                                            button {
                                                class: "{btn_class}",
                                                onclick: move |_| {
                                                    let mut sels = selections.write();
                                                    // Replace the selection for this group
                                                    if let Some(entry) = sels.iter_mut().find(|(gid, _, _)| *gid == group_id) {
                                                        entry.1 = opt_id;
                                                        entry.2 = opt_delta;
                                                    } else {
                                                        sels.push((group_id, opt_id, opt_delta));
                                                    }
                                                    let ids: Vec<i64> = sels.iter().map(|(_, oid, _)| *oid).collect();
                                                    let delta: f64 = sels.iter().map(|(_, _, d)| d).sum();
                                                    drop(sels);
                                                    on_change.call((ids, delta));
                                                },
                                                "{opt_label}{delta_text}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "option-selector-summary",
                span { class: "option-delta-label", "Options: " }
                span { class: "option-delta-value", "{delta_display}" }
            }
        }
    }
}
