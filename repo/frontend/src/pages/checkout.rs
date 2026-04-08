use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::components::price_display::PriceDisplay;
use crate::components::slot_picker::SlotPicker;
use crate::components::hold_timer::HoldTimer;
use crate::state::AppState;
use shared::dto::{ApiResponse, CartResponse, CheckoutRequest, CheckoutResponse, PickupSlot};

#[component]
pub fn CheckoutPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let mut app_state = use_context::<Signal<AppState>>();

    let mut selected_slot = use_signal(|| Option::<PickupSlot>::None);
    let mut checkout_result = use_signal(|| Option::<CheckoutResponse>::None);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut placing = use_signal(|| false);

    // Load cart summary
    let cart_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/cart", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<CartResponse> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No cart data".to_string())
        }
    });

    // Load available pickup slots
    let slots_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/store/pickup-slots?date={}", crate::API_BASE, chrono::Utc::now().format("%Y-%m-%d")));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<PickupSlot>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No slots available".to_string())
        }
    });

    let page_title = t.t(&loc, "page.checkout");
    let subtotal_label = t.t(&loc, "label.subtotal");
    let tax_label = t.t(&loc, "label.tax");
    let total_label = t.t(&loc, "label.total");
    let voucher_label = t.t(&loc, "label.voucher_code");
    let hold_warning = t.t(&loc, "msg.hold_warning");
    let place_order_text = if loc == "zh" { "\u{4e0b}\u{5355}" } else { "Place Order" };

    // If checkout completed, show success view
    if let Some(result) = checkout_result() {
        return rsx! {
            div { class: "page page-checkout",
                Navbar { locale: locale.clone() }

                main { class: "main-content",
                    section { class: "section checkout-success",
                        div { class: "success-card",
                            h2 { class: "success-title",
                                if loc == "zh" { "\u{8ba2}\u{5355}\u{5df2}\u{521b}\u{5efa}\u{ff01}" } else { "Order Placed!" }
                            }
                            p { class: "success-order-number",
                                if loc == "zh" { "\u{8ba2}\u{5355}\u{53f7}: " } else { "Order #: " }
                                strong { "{result.order_number}" }
                            }

                            // Voucher code prominently displayed
                            div { class: "voucher-display",
                                h3 { class: "voucher-label", "{voucher_label}" }
                                div { class: "voucher-code", "{result.voucher_code}" }
                            }

                            // Hold timer countdown
                            div { class: "hold-timer-section",
                                HoldTimer { expires_at: result.hold_expires_at.clone(), locale: locale.clone() }
                            }

                            // Warning message
                            div { class: "alert alert-warning",
                                "{hold_warning}"
                            }

                            p { class: "pickup-slot-info",
                                if loc == "zh" { "\u{53d6}\u{9910}\u{65f6}\u{6bb5}: " } else { "Pickup: " }
                                strong { "{result.pickup_slot}" }
                            }

                            div { class: "success-total",
                                span { "{total_label}: " }
                                PriceDisplay { amount: result.total, locale: locale.clone() }
                            }

                            div { class: "success-actions",
                                Link {
                                    to: crate::Route::OrderDetail { locale: locale.clone(), id: result.order_id },
                                    class: "btn btn-primary",
                                    if loc == "zh" { "\u{67e5}\u{770b}\u{8ba2}\u{5355}\u{8be6}\u{60c5}" } else { "View Order Detail" }
                                }
                                Link {
                                    to: crate::Route::Home { locale: locale.clone() },
                                    class: "btn btn-secondary",
                                    if loc == "zh" { "\u{8fd4}\u{56de}\u{9996}\u{9875}" } else { "Back to Home" }
                                }
                            }
                        }
                    }
                }

                Footer {}
            }
        };
    }

    rsx! {
        div { class: "page page-checkout",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    if let Some(err) = error_msg() {
                        div { class: "alert alert-error", "{err}" }
                    }

                    div { class: "checkout-layout",
                        // Cart summary column
                        div { class: "checkout-summary",
                            h3 { if loc == "zh" { "\u{8ba2}\u{5355}\u{6458}\u{8981}" } else { "Order Summary" } }
                            match &*cart_resource.read() {
                                Some(Ok(cart)) => rsx! {
                                    div { class: "checkout-items",
                                        for item in cart.items.iter() {
                                            {
                                                let item_name = if loc == "zh" { &item.spu_name_zh } else { &item.spu_name_en };
                                                let options_text = item.options.join(", ");
                                                rsx! {
                                                    div { class: "checkout-item",
                                                        div { class: "checkout-item-info",
                                                            span { class: "checkout-item-name", "{item_name}" }
                                                            if !options_text.is_empty() {
                                                                span { class: "checkout-item-options", " ({options_text})" }
                                                            }
                                                            span { class: "checkout-item-qty", " x{item.quantity}" }
                                                        }
                                                        PriceDisplay { amount: item.line_total, locale: locale.clone() }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div { class: "checkout-totals",
                                        div { class: "checkout-total-row",
                                            span { "{subtotal_label}" }
                                            PriceDisplay { amount: cart.subtotal, locale: locale.clone() }
                                        }
                                        div { class: "checkout-total-row",
                                            span { "{tax_label} ({cart.tax_rate * 100.0:.0}%)" }
                                            PriceDisplay { amount: cart.tax_amount, locale: locale.clone() }
                                        }
                                        div { class: "checkout-total-row checkout-total-grand",
                                            span { "{total_label}" }
                                            PriceDisplay { amount: cart.total, locale: locale.clone() }
                                        }
                                    }
                                },
                                Some(Err(e)) => rsx! {
                                    div { class: "alert alert-error", "Error loading cart: {e}" }
                                },
                                None => rsx! {
                                    div { class: "loading-spinner", p { "Loading..." } }
                                },
                            }
                        }

                        // Pickup slot selection
                        div { class: "checkout-slot-section",
                            match &*slots_resource.read() {
                                Some(Ok(slots)) => rsx! {
                                    SlotPicker {
                                        slots: slots.clone(),
                                        locale: locale.clone(),
                                        on_select: move |slot: PickupSlot| {
                                            selected_slot.set(Some(slot));
                                        },
                                    }
                                },
                                Some(Err(e)) => rsx! {
                                    div { class: "alert alert-error", "Error loading slots: {e}" }
                                },
                                None => rsx! {
                                    div { class: "loading-spinner", p { "Loading slots..." } }
                                },
                            }

                            if selected_slot().is_some() {
                                div { class: "selected-slot-info",
                                    {
                                        let slot = selected_slot().unwrap();
                                        rsx! {
                                            p {
                                                if loc == "zh" { "\u{5df2}\u{9009}\u{62e9}: " } else { "Selected: " }
                                                strong { "{slot.start} - {slot.end}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Place order button
                    div { class: "checkout-actions",
                        {
                            let locale_place = locale.clone();
                            rsx! {
                                button {
                                    class: "btn btn-primary btn-lg btn-block",
                                    disabled: selected_slot().is_none() || placing(),
                                    onclick: move |_| {
                                        let Some(slot) = selected_slot() else { return; };
                                        let session_cookie = app_state().auth.session_cookie.clone();
                                        let locale_inner = locale_place.clone();
                                        spawn(async move {
                                            placing.set(true);
                                            error_msg.set(None);

                                            let body = CheckoutRequest {
                                                pickup_slot_start: slot.start.clone(),
                                                pickup_slot_end: slot.end.clone(),
                                            };

                                            let mut req = reqwest::Client::new()
                                                .post(&format!("{}/orders/checkout", crate::API_BASE))
                                                .json(&body);
                                            if let Some(ref sc) = session_cookie {
                                                req = req.header("Cookie", format!("brewflow_session={}", sc));
                                            }

                                            match req.send().await {
                                                Ok(resp) => {
                                                    if resp.status().is_success() {
                                                        match resp.json::<ApiResponse<CheckoutResponse>>().await {
                                                            Ok(api) => {
                                                                if let Some(data) = api.data {
                                                                    app_state.write().cart_count = 0;
                                                                    checkout_result.set(Some(data));
                                                                } else {
                                                                    error_msg.set(Some(api.error.unwrap_or_else(|| "Checkout failed".to_string())));
                                                                }
                                                            }
                                                            Err(e) => error_msg.set(Some(format!("Parse error: {}", e))),
                                                        }
                                                    } else {
                                                        let body = resp.text().await.unwrap_or_default();
                                                        error_msg.set(Some(format!("Checkout failed: {}", body)));
                                                    }
                                                }
                                                Err(e) => error_msg.set(Some(format!("Network error: {}", e))),
                                            }

                                            placing.set(false);
                                        });
                                    },
                                    if placing() { "..." } else { "{place_order_text}" }
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
