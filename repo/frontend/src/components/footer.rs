use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            div { class: "footer-content",
                p { class: "footer-text",
                    "BrewFlow Offline Retail & Training Suite"
                }
                p { class: "footer-copyright",
                    "\u{00a9} 2026 BrewFlow. All rights reserved."
                }
            }
        }
    }
}
