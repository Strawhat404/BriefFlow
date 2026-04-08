use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::components::price_display::PriceDisplay;
use crate::components::status_badge::StatusBadge;
use crate::state::AppState;
use shared::dto::{
    ApiResponse, OrderDetail, OrderSummary, ScanVoucherRequest, ScanVoucherResponse,
    UpdateOrderStatusRequest,
};

// ---------------------------------------------------------------------------
// Staff Dashboard counts
// ---------------------------------------------------------------------------
#[derive(serde::Deserialize, Clone, Debug)]
struct DashboardCounts {
    pending_count: i64,
    in_prep_count: i64,
    ready_count: i64,
}

// ---------------------------------------------------------------------------
// StaffDashboardPage
// ---------------------------------------------------------------------------
#[component]
pub fn StaffDashboardPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let mut status_filter = use_signal(|| String::new());

    // Load dashboard counts
    let counts_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/staff/dashboard/counts", crate::API_BASE));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<DashboardCounts> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    // Load orders with optional filter
    let orders_resource = use_resource(move || {
        let session_cookie = app_state().auth.session_cookie.clone();
        let filter = status_filter();
        async move {
            let url = if filter.is_empty() {
                format!("{}/staff/orders", crate::API_BASE)
            } else {
                format!("{}/staff/orders?status={}", crate::API_BASE, filter)
            };
            let mut req = reqwest::Client::new().get(&url);
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<OrderSummary>> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data".to_string())
        }
    });

    let page_title = t.t(&loc, "page.staff_dashboard");
    let pending_label = t.t(&loc, "status.pending");
    let in_prep_label = t.t(&loc, "status.in_prep");
    let ready_label = t.t(&loc, "status.ready");
    let all_label = if loc == "zh" { "\u{5168}\u{90e8}" } else { "All" };
    let scan_label = t.t(&loc, "btn.scan");

    rsx! {
        div { class: "page page-staff-dashboard",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    div { class: "staff-header",
                        h2 { class: "section-title", "{page_title}" }
                        Link {
                            to: crate::Route::StaffScan { locale: locale.clone() },
                            class: "btn btn-primary",
                            "{scan_label}"
                        }
                    }

                    // Overview cards
                    div { class: "dashboard-cards",
                        match &*counts_resource.read() {
                            Some(Ok(counts)) => rsx! {
                                div {
                                    class: "dashboard-card dashboard-card-pending",
                                    onclick: move |_| status_filter.set("Pending".to_string()),
                                    h3 { class: "dashboard-card-count", "{counts.pending_count}" }
                                    p { class: "dashboard-card-label", "{pending_label}" }
                                }
                                div {
                                    class: "dashboard-card dashboard-card-in-prep",
                                    onclick: move |_| status_filter.set("InPrep".to_string()),
                                    h3 { class: "dashboard-card-count", "{counts.in_prep_count}" }
                                    p { class: "dashboard-card-label", "{in_prep_label}" }
                                }
                                div {
                                    class: "dashboard-card dashboard-card-ready",
                                    onclick: move |_| status_filter.set("Ready".to_string()),
                                    h3 { class: "dashboard-card-count", "{counts.ready_count}" }
                                    p { class: "dashboard-card-label", "{ready_label}" }
                                }
                            },
                            Some(Err(e)) => rsx! {
                                div { class: "alert alert-error", "Error: {e}" }
                            },
                            None => rsx! {
                                div { class: "loading-spinner", p { "Loading counts..." } }
                            },
                        }
                    }

                    // Filter bar
                    div { class: "filter-bar",
                        {
                            let current = status_filter();
                            let statuses: Vec<(&str, &str)> = vec![
                                ("", all_label),
                                ("Pending", &pending_label),
                                ("Accepted", "Accepted"),
                                ("InPrep", &in_prep_label),
                                ("Ready", &ready_label),
                            ];
                            rsx! {
                                for (val, label) in statuses.into_iter() {
                                    {
                                        let is_active = current == val;
                                        let val_owned = val.to_string();
                                        rsx! {
                                            button {
                                                class: if is_active { "filter-btn filter-btn-active" } else { "filter-btn" },
                                                onclick: move |_| status_filter.set(val_owned.clone()),
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Orders list
                    match &*orders_resource.read() {
                        Some(Ok(orders)) => rsx! {
                            div { class: "staff-orders-list",
                                if orders.is_empty() {
                                    p { class: "empty-text",
                                        if loc == "zh" { "\u{6ca1}\u{6709}\u{8ba2}\u{5355}" } else { "No orders" }
                                    }
                                }
                                for order in orders.iter() {
                                    {
                                        let oid = order.id;
                                        rsx! {
                                            Link {
                                                to: crate::Route::StaffOrderDetail { locale: locale.clone(), id: oid },
                                                class: "order-card staff-order-card",
                                                div { class: "order-card-header",
                                                    span { class: "order-number", "#{order.order_number}" }
                                                    StatusBadge { status: order.status.clone(), locale: locale.clone() }
                                                }
                                                div { class: "order-card-body",
                                                    PriceDisplay { amount: order.total, locale: locale.clone() }
                                                    span { class: "order-card-date", "{order.created_at}" }
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
                            div { class: "loading-spinner", p { "Loading orders..." } }
                        },
                    }
                }
            }

            Footer {}
        }
    }
}

// ---------------------------------------------------------------------------
// StaffOrderDetailPage
// ---------------------------------------------------------------------------
#[component]
pub fn StaffOrderDetailPage(locale: String, id: i64) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let mut status_notes = use_signal(|| String::new());
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut success_msg = use_signal(|| Option::<String>::None);
    let mut action_loading = use_signal(|| false);
    let mut refresh_trigger = use_signal(|| 0u32);

    let detail_resource = use_resource(move || {
        let _trigger = refresh_trigger();
        let session_cookie = app_state().auth.session_cookie.clone();
        async move {
            let mut req = reqwest::Client::new()
                .get(&format!("{}/staff/orders/{}", crate::API_BASE, id));
            if let Some(ref sc) = session_cookie {
                req = req.header("Cookie", format!("brewflow_session={}", sc));
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            let data: ApiResponse<OrderDetail> = resp.json().await.map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "Order not found".to_string())
        }
    });

    let page_title = if loc == "zh" { "\u{8ba2}\u{5355}\u{7ba1}\u{7406}" } else { "Manage Order" };
    let notes_label = if loc == "zh" { "\u{5907}\u{6ce8}" } else { "Notes" };

    rsx! {
        div { class: "page page-staff-order-detail",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    if let Some(err) = error_msg() {
                        div { class: "alert alert-error", "{err}" }
                    }
                    if let Some(msg) = success_msg() {
                        div { class: "alert alert-success", "{msg}" }
                    }

                    match &*detail_resource.read() {
                        Some(Ok(detail)) => {
                            let order = &detail.order;
                            let current_status = &order.status;

                            // Determine valid next states from the state machine
                            let allowed = match current_status.as_str() {
                                "Pending" => vec![("Accepted", "Accepted"), ("Canceled", "Canceled")],
                                "Accepted" => vec![("InPrep", "In Preparation"), ("Canceled", "Canceled")],
                                "InPrep" => vec![("Ready", "Ready"), ("Canceled", "Canceled")],
                                "Ready" => vec![("PickedUp", "Picked Up"), ("Canceled", "Canceled")],
                                _ => vec![],
                            };

                            rsx! {
                                div { class: "staff-order-detail-card",
                                    // Order info
                                    div { class: "order-detail-header",
                                        h3 { "#{order.order_number}" }
                                        StatusBadge { status: order.status.clone(), locale: locale.clone() }
                                    }

                                    // Items
                                    div { class: "order-detail-items",
                                        for item in detail.items.iter() {
                                            div { class: "order-detail-item",
                                                span { class: "order-item-name", "{item.spu_name}" }
                                                if !item.options.is_empty() {
                                                    span { class: "order-item-options", " ({item.options.join(\", \")})" }
                                                }
                                                span { class: "order-item-qty", " x{item.quantity}" }
                                                PriceDisplay { amount: item.item_total, locale: locale.clone() }
                                            }
                                        }
                                        div { class: "order-total-row order-total-grand",
                                            span { if loc == "zh" { "\u{603b}\u{8ba1}" } else { "Total" } }
                                            PriceDisplay { amount: order.total, locale: locale.clone() }
                                        }
                                    }

                                    // Status transition buttons
                                    if !allowed.is_empty() {
                                        div { class: "staff-status-update",
                                            h4 { if loc == "zh" { "\u{66f4}\u{65b0}\u{72b6}\u{6001}" } else { "Update Status" } }

                                            div { class: "form-group",
                                                label { "{notes_label}" }
                                                textarea {
                                                    class: "form-input form-textarea",
                                                    placeholder: "{notes_label}",
                                                    value: "{status_notes}",
                                                    oninput: move |evt| status_notes.set(evt.value()),
                                                }
                                            }

                                            div { class: "status-transition-buttons",
                                                for (status_val, status_label) in allowed.iter() {
                                                    {
                                                        let new_status = status_val.to_string();
                                                        let btn_class = if *status_val == "Canceled" {
                                                            "btn btn-danger"
                                                        } else {
                                                            "btn btn-primary"
                                                        };
                                                        rsx! {
                                                            button {
                                                                class: "{btn_class}",
                                                                disabled: action_loading(),
                                                                onclick: move |_| {
                                                                    let session_cookie = app_state().auth.session_cookie.clone();
                                                                    let ns = new_status.clone();
                                                                    let notes = status_notes().clone();
                                                                    spawn(async move {
                                                                        action_loading.set(true);
                                                                        error_msg.set(None);
                                                                        success_msg.set(None);

                                                                        let body = UpdateOrderStatusRequest {
                                                                            new_status: ns.clone(),
                                                                            notes: if notes.is_empty() { None } else { Some(notes) },
                                                                        };

                                                                        let mut req = reqwest::Client::new()
                                                                            .put(&format!("{}/staff/orders/{}/status", crate::API_BASE, id))
                                                                            .json(&body);
                                                                        if let Some(ref sc) = session_cookie {
                                                                            req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                                        }

                                                                        match req.send().await {
                                                                            Ok(resp) if resp.status().is_success() => {
                                                                                success_msg.set(Some(format!("Status updated to {}", ns)));
                                                                                status_notes.set(String::new());
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
                                                                "{status_label}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Fulfillment event history timeline
                                    if !detail.fulfillment_history.is_empty() {
                                        div { class: "order-detail-timeline",
                                            h4 { if loc == "zh" { "\u{5c65}\u{7ea6}\u{5386}\u{53f2}" } else { "Fulfillment History" } }
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
// StaffScanPage
// ---------------------------------------------------------------------------
#[component]
pub fn StaffScanPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let app_state = use_context::<Signal<AppState>>();

    let mut voucher_input = use_signal(|| String::new());
    let mut scan_result = use_signal(|| Option::<ScanVoucherResponse>::None);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut scanning = use_signal(|| false);

    let page_title = if loc == "zh" { "\u{626b}\u{7801}\u{53d6}\u{9910}" } else { "Scan Voucher" };
    let input_label = t.t(&loc, "label.voucher_code");
    let scan_text = t.t(&loc, "btn.scan");
    let mismatch_text = t.t(&loc, "msg.mismatch_warning");

    rsx! {
        div { class: "page page-staff-scan",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    // Scan input form
                    div { class: "scan-card",
                        form {
                            class: "scan-form",
                            onsubmit: move |evt| {
                                evt.prevent_default();
                                let code = voucher_input().clone();
                                let session_cookie = app_state().auth.session_cookie.clone();
                                spawn(async move {
                                    scanning.set(true);
                                    error_msg.set(None);
                                    scan_result.set(None);

                                    let body = ScanVoucherRequest {
                                        voucher_code: code,
                                        order_id: None,
                                    };

                                    let mut req = reqwest::Client::new()
                                        .post(&format!("{}/staff/scan", crate::API_BASE))
                                        .json(&body);
                                    if let Some(ref sc) = session_cookie {
                                        req = req.header("Cookie", format!("brewflow_session={}", sc));
                                    }

                                    match req.send().await {
                                        Ok(resp) => {
                                            if resp.status().is_success() {
                                                match resp.json::<ApiResponse<ScanVoucherResponse>>().await {
                                                    Ok(api) => {
                                                        if let Some(data) = api.data {
                                                            scan_result.set(Some(data));
                                                        } else {
                                                            error_msg.set(Some(api.error.unwrap_or_else(|| "Scan failed".to_string())));
                                                        }
                                                    }
                                                    Err(e) => error_msg.set(Some(format!("Parse error: {}", e))),
                                                }
                                            } else {
                                                let body = resp.text().await.unwrap_or_default();
                                                error_msg.set(Some(format!("Scan failed: {}", body)));
                                            }
                                        }
                                        Err(e) => error_msg.set(Some(format!("Network error: {}", e))),
                                    }

                                    scanning.set(false);
                                });
                            },

                            div { class: "form-group",
                                label { r#for: "voucher-code", "{input_label}" }
                                input {
                                    r#type: "text",
                                    id: "voucher-code",
                                    class: "form-input form-input-lg",
                                    placeholder: if loc == "zh" { "\u{8f93}\u{5165}\u{6216}\u{626b}\u{63cf}\u{53d6}\u{9910}\u{7801}" } else { "Enter or scan voucher code" },
                                    value: "{voucher_input}",
                                    oninput: move |evt| voucher_input.set(evt.value()),
                                    autofocus: true,
                                }
                            }

                            button {
                                r#type: "submit",
                                class: "btn btn-primary btn-lg btn-block",
                                disabled: scanning() || voucher_input().is_empty(),
                                if scanning() { "..." } else { "{scan_text}" }
                            }
                        }
                    }

                    if let Some(err) = error_msg() {
                        div { class: "alert alert-error", "{err}" }
                    }

                    // Scan result
                    if let Some(result) = scan_result() {
                        div { class: "scan-result",
                            if result.mismatch {
                                div { class: "alert alert-error alert-mismatch",
                                    h3 { class: "mismatch-title", "{mismatch_text}" }
                                    if let Some(ref reason) = result.mismatch_reason {
                                        p { class: "mismatch-reason", "{reason}" }
                                    }
                                }
                            }

                            if result.valid {
                                div { class: "scan-valid",
                                    div { class: "scan-valid-badge",
                                        if loc == "zh" { "\u{2713} \u{6709}\u{6548}" } else { "\u{2713} Valid" }
                                    }
                                }
                            } else {
                                div { class: "scan-invalid",
                                    div { class: "scan-invalid-badge",
                                        if loc == "zh" { "\u{2717} \u{65e0}\u{6548}" } else { "\u{2717} Invalid" }
                                    }
                                }
                            }

                            if let Some(ref order) = result.order {
                                div { class: "scan-order-detail",
                                    h3 {
                                        if loc == "zh" { "\u{8ba2}\u{5355}\u{8be6}\u{60c5}" } else { "Order Details" }
                                    }
                                    div { class: "order-card",
                                        div { class: "order-card-header",
                                            span { class: "order-number", "#{order.order_number}" }
                                            StatusBadge { status: order.status.clone(), locale: locale.clone() }
                                        }
                                        div { class: "order-card-body",
                                            PriceDisplay { amount: order.total, locale: locale.clone() }
                                            span { class: "order-card-date", "{order.created_at}" }
                                        }
                                        if let Some(ref slot) = order.pickup_slot {
                                            div { class: "order-card-pickup",
                                                if loc == "zh" { "\u{53d6}\u{9910}: " } else { "Pickup: " }
                                                "{slot}"
                                            }
                                        }
                                    }

                                    Link {
                                        to: crate::Route::StaffOrderDetail { locale: locale.clone(), id: order.id },
                                        class: "btn btn-primary",
                                        if loc == "zh" { "\u{7ba1}\u{7406}\u{8ba2}\u{5355}" } else { "Manage Order" }
                                    }
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
