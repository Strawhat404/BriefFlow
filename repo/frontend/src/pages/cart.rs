use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::components::price_display::PriceDisplay;
use crate::state::AppState;
use shared::dto::{ApiResponse, CartResponse};

#[component]
pub fn CartPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let mut app_state = use_context::<Signal<AppState>>();

    let mut cart_data = use_signal(|| Option::<Result<CartResponse, String>>::None);
    let mut loading = use_signal(|| true);
    let mut update_trigger = use_signal(|| 0u32);

    // Load cart data
    let locale_load = locale.clone();
    use_resource(move || {
        let _trigger = update_trigger();
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            loading.set(true);
            let mut req = reqwest::Client::new()
                .get(&format!("{}/cart", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            match req.send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.json::<ApiResponse<CartResponse>>().await {
                            Ok(api) => {
                                if let Some(data) = api.data {
                                    let count = data.items.iter().map(|i| i.quantity).sum::<i32>();
                                    app_state.write().cart_count = count;
                                    cart_data.set(Some(Ok(data)));
                                } else {
                                    cart_data.set(Some(Err(api.error.unwrap_or_else(|| "No cart data".to_string()))));
                                }
                            }
                            Err(e) => cart_data.set(Some(Err(format!("Parse error: {}", e)))),
                        }
                    } else {
                        cart_data.set(Some(Err(format!("HTTP {}", resp.status()))));
                    }
                }
                Err(e) => cart_data.set(Some(Err(format!("Network error: {}", e)))),
            }
            loading.set(false);
        }
    });

    let page_title = t.t(&loc, "page.cart");
    let subtotal_label = t.t(&loc, "label.subtotal");
    let tax_label = t.t(&loc, "label.tax");
    let total_label = t.t(&loc, "label.total");
    let checkout_text = t.t(&loc, "btn.checkout");
    let quantity_label = t.t(&loc, "label.quantity");
    let empty_text = if loc == "zh" { "\u{8d2d}\u{7269}\u{8f66}\u{4e3a}\u{7a7a}" } else { "Your cart is empty" };
    let remove_text = if loc == "zh" { "\u{5220}\u{9664}" } else { "Remove" };

    rsx! {
        div { class: "page page-cart",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    if loading() {
                        div { class: "loading-spinner", p { "Loading..." } }
                    } else {
                        match cart_data() {
                            Some(Ok(cart)) => {
                                if cart.items.is_empty() {
                                    rsx! {
                                        div { class: "empty-state",
                                            p { class: "empty-text", "{empty_text}" }
                                            Link {
                                                to: crate::Route::Menu { locale: locale.clone() },
                                                class: "btn btn-primary",
                                                if loc == "zh" { "\u{53bb}\u{9009}\u{8d2d}" } else { "Browse Menu" }
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div { class: "cart-items",
                                            for item in cart.items.iter() {
                                                {
                                                    let item_name = if loc == "zh" { &item.spu_name_zh } else { &item.spu_name_en };
                                                    let item_id = item.id;
                                                    let item_qty = item.quantity;
                                                    let options_text = item.options.join(", ");
                                                    rsx! {
                                                        div { class: "cart-item",
                                                            div { class: "cart-item-info",
                                                                h3 { class: "cart-item-name", "{item_name}" }
                                                                if !options_text.is_empty() {
                                                                    p { class: "cart-item-options", "{options_text}" }
                                                                }
                                                                if let Some(ref sku) = item.sku_code {
                                                                    p { class: "cart-item-sku", "SKU: {sku}" }
                                                                }
                                                            }
                                                            div { class: "cart-item-controls",
                                                                // Quantity controls
                                                                div { class: "quantity-controls",
                                                                    button {
                                                                        class: "btn btn-sm",
                                                                        disabled: item_qty <= 1,
                                                                        onclick: move |_| {
                                                                            let session_cookie = app_state().auth.session_cookie.clone();
                                                                            let new_qty = item_qty - 1;
                                                                            spawn(async move {
                                                                                let mut req = reqwest::Client::new()
                                                                                    .put(&format!("{}/cart/{}", crate::API_BASE, item_id))
                                                                                    .json(&serde_json::json!({ "quantity": new_qty }));
                                                                                if let Some(ref sc) = session_cookie {
                                                                                    req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                                                }
                                                                                let _ = req.send().await;
                                                                                update_trigger.set(update_trigger() + 1);
                                                                            });
                                                                        },
                                                                        "-"
                                                                    }
                                                                    span { class: "quantity-value", "{item_qty}" }
                                                                    button {
                                                                        class: "btn btn-sm",
                                                                        onclick: move |_| {
                                                                            let session_cookie = app_state().auth.session_cookie.clone();
                                                                            let new_qty = item_qty + 1;
                                                                            spawn(async move {
                                                                                let mut req = reqwest::Client::new()
                                                                                    .put(&format!("{}/cart/{}", crate::API_BASE, item_id))
                                                                                    .json(&serde_json::json!({ "quantity": new_qty }));
                                                                                if let Some(ref sc) = session_cookie {
                                                                                    req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                                                }
                                                                                let _ = req.send().await;
                                                                                update_trigger.set(update_trigger() + 1);
                                                                            });
                                                                        },
                                                                        "+"
                                                                    }
                                                                }
                                                                // Line total
                                                                div { class: "cart-item-price",
                                                                    PriceDisplay { amount: item.line_total, locale: locale.clone() }
                                                                }
                                                                // Remove button
                                                                button {
                                                                    class: "btn btn-sm btn-danger",
                                                                    onclick: move |_| {
                                                                        let session_cookie = app_state().auth.session_cookie.clone();
                                                                        spawn(async move {
                                                                            let mut req = reqwest::Client::new()
                                                                                .delete(&format!("{}/cart/{}", crate::API_BASE, item_id));
                                                                            if let Some(ref sc) = session_cookie {
                                                                                req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                                            }
                                                                            let _ = req.send().await;
                                                                            update_trigger.set(update_trigger() + 1);
                                                                        });
                                                                    },
                                                                    "{remove_text}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        // Totals section
                                        div { class: "cart-totals",
                                            div { class: "cart-total-row",
                                                span { class: "cart-total-label", "{subtotal_label}" }
                                                PriceDisplay { amount: cart.subtotal, locale: locale.clone() }
                                            }
                                            div { class: "cart-total-row",
                                                span { class: "cart-total-label", "{tax_label} ({cart.tax_rate * 100.0:.0}%)" }
                                                PriceDisplay { amount: cart.tax_amount, locale: locale.clone() }
                                            }
                                            div { class: "cart-total-row cart-total-grand",
                                                span { class: "cart-total-label", "{total_label}" }
                                                PriceDisplay { amount: cart.total, locale: locale.clone() }
                                            }
                                        }

                                        div { class: "cart-actions",
                                            Link {
                                                to: crate::Route::Menu { locale: locale.clone() },
                                                class: "btn btn-secondary",
                                                if loc == "zh" { "\u{7ee7}\u{7eed}\u{9009}\u{8d2d}" } else { "Continue Shopping" }
                                            }
                                            Link {
                                                to: crate::Route::Checkout { locale: locale.clone() },
                                                class: "btn btn-primary btn-lg",
                                                "{checkout_text}"
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
            }

            Footer {}
        }
    }
}
