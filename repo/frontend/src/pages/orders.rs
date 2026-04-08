use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::components::price_display::PriceDisplay;
use crate::components::status_badge::StatusBadge;
use crate::components::hold_timer::HoldTimer;
use crate::state::AppState;
use shared::dto::{ApiResponse, OrderDetail, OrderSummary};

// ---------------------------------------------------------------------------
// OrdersPage
// ---------------------------------------------------------------------------
#[component]
pub fn OrdersPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let orders_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/orders", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<OrderSummary>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    let page_title = t.t(&loc, "page.orders");

    rsx! {
        div { class: "page page-orders",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    match &*orders_resource.read() {
                        Some(Ok(orders)) => {
                            if orders.is_empty() {
                                rsx! {
                                    div { class: "empty-state",
                                        p { class: "empty-text",
                                            if loc == "zh" { "\u{6682}\u{65e0}\u{8ba2}\u{5355}" } else { "No orders yet" }
                                        }
                                        Link {
                                            to: crate::Route::Menu { locale: locale.clone() },
                                            class: "btn btn-primary",
                                            if loc == "zh" { "\u{53bb}\u{9009}\u{8d2d}" } else { "Browse Menu" }
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { class: "orders-list",
                                        for order in orders.iter() {
                                            {
                                                let oid = order.id;
                                                rsx! {
                                                    Link {
                                                        to: crate::Route::OrderDetail { locale: locale.clone(), id: oid },
                                                        class: "order-card",
                                                        div { class: "order-card-header",
                                                            span { class: "order-number", "#{order.order_number}" }
                                                            StatusBadge { status: order.status.clone(), locale: locale.clone() }
                                                        }
                                                        div { class: "order-card-body",
                                                            div { class: "order-card-total",
                                                                PriceDisplay { amount: order.total, locale: locale.clone() }
                                                            }
                                                            span { class: "order-card-date", "{order.created_at}" }
                                                            if let Some(ref voucher) = order.voucher_code {
                                                                span { class: "order-card-voucher",
                                                                    if loc == "zh" { "\u{53d6}\u{9910}\u{7801}: " } else { "Voucher: " }
                                                                    "{voucher}"
                                                                }
                                                            }
                                                        }
                                                        if let Some(ref slot) = order.pickup_slot {
                                                            div { class: "order-card-pickup",
                                                                if loc == "zh" { "\u{53d6}\u{9910}: " } else { "Pickup: " }
                                                                "{slot}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
// OrderDetailPage
// ---------------------------------------------------------------------------
#[component]
pub fn OrderDetailPage(locale: String, id: i64) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut action_loading = use_signal(|| false);
    let mut refresh_trigger = use_signal(|| 0u32);

    let detail_resource = use_resource(move || {
        let _trigger = refresh_trigger();
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/orders/{}", crate::API_BASE, id));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<OrderDetail> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "Order not found".to_string())
        }
    });

    let page_title = t.t(&loc, "page.order_detail");
    let subtotal_label = t.t(&loc, "label.subtotal");
    let total_label = t.t(&loc, "label.total");
    let voucher_label = t.t(&loc, "label.voucher_code");
    let confirm_text = t.t(&loc, "btn.confirm");
    let cancel_text = t.t(&loc, "btn.cancel");

    rsx! {
        div { class: "page page-order-detail",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    if let Some(err) = error_msg() {
                        div { class: "alert alert-error", "{err}" }
                    }

                    match &*detail_resource.read() {
                        Some(Ok(detail)) => {
                            let order = &detail.order;
                            let status = &order.status;
                            let can_confirm = status == "Pending";
                            let can_cancel = status == "Pending" || status == "Accepted";

                            rsx! {
                                div { class: "order-detail-card",
                                    // Header
                                    div { class: "order-detail-header",
                                        h3 { "#{order.order_number}" }
                                        StatusBadge { status: order.status.clone(), locale: locale.clone() }
                                    }

                                    // Items
                                    div { class: "order-detail-items",
                                        h4 { if loc == "zh" { "\u{8ba2}\u{5355}\u{9879}\u{76ee}" } else { "Items" } }
                                        for item in detail.items.iter() {
                                            div { class: "order-detail-item",
                                                div { class: "order-item-info",
                                                    span { class: "order-item-name", "{item.spu_name}" }
                                                    if !item.options.is_empty() {
                                                        span { class: "order-item-options", " ({item.options.join(\", \")})" }
                                                    }
                                                    span { class: "order-item-qty", " x{item.quantity}" }
                                                }
                                                PriceDisplay { amount: item.item_total, locale: locale.clone() }
                                            }
                                        }
                                    }

                                    // Total
                                    div { class: "order-detail-totals",
                                        div { class: "order-total-row order-total-grand",
                                            span { "{total_label}" }
                                            PriceDisplay { amount: order.total, locale: locale.clone() }
                                        }
                                    }

                                    // Reservation info
                                    if let Some(ref reservation) = detail.reservation {
                                        div { class: "order-detail-reservation",
                                            h4 { if loc == "zh" { "\u{9884}\u{7ea6}\u{4fe1}\u{606f}" } else { "Reservation" } }
                                            div { class: "reservation-info",
                                                div { class: "voucher-display",
                                                    span { class: "voucher-label", "{voucher_label}: " }
                                                    span { class: "voucher-code", "{reservation.voucher_code}" }
                                                }
                                                p {
                                                    if loc == "zh" { "\u{53d6}\u{9910}\u{65f6}\u{6bb5}: " } else { "Pickup: " }
                                                    "{reservation.pickup_slot_start} - {reservation.pickup_slot_end}"
                                                }
                                                StatusBadge { status: reservation.status.clone(), locale: locale.clone() }
                                                if reservation.status == "Held" {
                                                    HoldTimer { expires_at: reservation.hold_expires_at.clone(), locale: locale.clone() }
                                                }
                                            }
                                        }
                                    }

                                    // Fulfillment timeline
                                    if !detail.fulfillment_history.is_empty() {
                                        div { class: "order-detail-timeline",
                                            h4 { if loc == "zh" { "\u{5c65}\u{7ea6}\u{65f6}\u{95f4}\u{7ebf}" } else { "Fulfillment Timeline" } }
                                            div { class: "timeline",
                                                for event in detail.fulfillment_history.iter() {
                                                    div { class: "timeline-event",
                                                        div { class: "timeline-dot" }
                                                        div { class: "timeline-content",
                                                            div { class: "timeline-header",
                                                                StatusBadge { status: event.from_status.clone().unwrap_or_default(), locale: locale.clone() }
                                                                span { class: "timeline-arrow", " -> " }
                                                                StatusBadge { status: event.to_status.clone(), locale: locale.clone() }
                                                            }
                                                            p { class: "timeline-meta",
                                                                "{event.changed_by} - {event.timestamp}"
                                                            }
                                                            if let Some(ref notes) = event.notes {
                                                                p { class: "timeline-notes", "{notes}" }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Action buttons
                                    if can_confirm || can_cancel {
                                        div { class: "order-detail-actions",
                                            if can_confirm {
                                                {
                                                    let locale_c = locale.clone();
                                                    rsx! {
                                                        button {
                                                            class: "btn btn-primary",
                                                            disabled: action_loading(),
                                                            onclick: move |_| {
                                                                let session_cookie = app_state().auth.session_cookie.clone();
                                                                spawn(async move {
                                                                    action_loading.set(true);
                                                                    error_msg.set(None);
                                                                    let body = serde_json::json!({ "action": "confirm" });
                                                                    let mut req = reqwest::Client::new()
                                                                        .post(&format!("{}/orders/{}/confirm", crate::API_BASE, id))
                                                                        .json(&body);
                                                                    if let Some(ref sc) = session_cookie {
                                                                        req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                                    }
                                                                    match req.send().await {
                                                                        Ok(resp) if resp.status().is_success() => {
                                                                            refresh_trigger.set(refresh_trigger() + 1);
                                                                        }
                                                                        Ok(resp) => {
                                                                            let body = resp.text().await.unwrap_or_default();
                                                                            error_msg.set(Some(format!("Failed: {}", body)));
                                                                        }
                                                                        Err(e) => error_msg.set(Some(format!("Error: {}", e))),
                                                                    }
                                                                    action_loading.set(false);
                                                                });
                                                            },
                                                            "{confirm_text}"
                                                        }
                                                    }
                                                }
                                            }
                                            if can_cancel {
                                                {
                                                    let locale_c = locale.clone();
                                                    rsx! {
                                                        button {
                                                            class: "btn btn-danger",
                                                            disabled: action_loading(),
                                                            onclick: move |_| {
                                                                let session_cookie = app_state().auth.session_cookie.clone();
                                                                spawn(async move {
                                                                    action_loading.set(true);
                                                                    error_msg.set(None);
                                                                    let body = serde_json::json!({ "action": "cancel" });
                                                                    let mut req = reqwest::Client::new()
                                                                        .post(&format!("{}/orders/{}/cancel", crate::API_BASE, id))
                                                                        .json(&body);
                                                                    if let Some(ref sc) = session_cookie {
                                                                        req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                                    }
                                                                    match req.send().await {
                                                                        Ok(resp) if resp.status().is_success() => {
                                                                            refresh_trigger.set(refresh_trigger() + 1);
                                                                        }
                                                                        Ok(resp) => {
                                                                            let body = resp.text().await.unwrap_or_default();
                                                                            error_msg.set(Some(format!("Failed: {}", body)));
                                                                        }
                                                                        Err(e) => error_msg.set(Some(format!("Error: {}", e))),
                                                                    }
                                                                    action_loading.set(false);
                                                                });
                                                            },
                                                            "{cancel_text}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Date info
                                    div { class: "order-detail-date",
                                        span { class: "order-date-label",
                                            if loc == "zh" { "\u{521b}\u{5efa}\u{65f6}\u{95f4}: " } else { "Created: " }
                                        }
                                        span { "{order.created_at}" }
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
