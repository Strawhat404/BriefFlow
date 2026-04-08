use dioxus::prelude::*;
use crate::state::AppState;

#[component]
pub fn LocaleSwitcher(current_locale: String) -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let nav = use_navigator();

    let is_en = current_locale == "en";
    let is_zh = current_locale == "zh";

    rsx! {
        div { class: "locale-switcher",
            button {
                class: if is_en { "locale-btn locale-btn-active" } else { "locale-btn" },
                disabled: is_en,
                onclick: move |_| {
                    let mut s = state.write();
                    s.locale = "en".to_string();
                    nav.replace(crate::Route::Home { locale: "en".to_string() });
                },
                "EN"
            }
            button {
                class: if is_zh { "locale-btn locale-btn-active" } else { "locale-btn" },
                disabled: is_zh,
                onclick: move |_| {
                    let mut s = state.write();
                    s.locale = "zh".to_string();
                    nav.replace(crate::Route::Home { locale: "zh".to_string() });
                },
                "\u{4e2d}\u{6587}"
            }
        }
    }
}
