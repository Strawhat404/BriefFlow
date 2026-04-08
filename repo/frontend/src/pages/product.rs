use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::components::price_display::PriceDisplay;
use crate::components::option_selector::OptionSelector;
use crate::state::AppState;
use shared::dto::{AddToCartRequest, ApiResponse, ProductDetail};
use shared::models::SalesTaxConfig;

#[component]
pub fn ProductDetailPage(locale: String, id: i64) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();
    let mut app_state = use_context::<Signal<AppState>>();
    let nav = use_navigator();

    let mut quantity = use_signal(|| 1i32);
    let mut selected_options = use_signal(|| Vec::<i64>::new());
    let mut options_delta = use_signal(|| 0.0f64);
    let mut add_error = use_signal(|| Option::<String>::None);
    let mut add_success = use_signal(|| false);
    let mut adding = use_signal(|| false);

    let product_resource = use_resource(move || {
        async move {
            let url = format!("{}/products/{}", crate::API_BASE, id);
            let resp = reqwest::Client::new()
                .get(&url)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let data: ApiResponse<ProductDetail> = resp
                .json()
                .await
                .map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "Product not found".to_string())
        }
    });

    let tax_resource = use_resource(move || {
        async move {
            let url = format!("{}/store/tax", crate::API_BASE);
            let resp = reqwest::Client::new()
                .get(&url)
                .send()
                .await
                .ok()?;
            let data: ApiResponse<SalesTaxConfig> = resp.json().await.ok()?;
            data.data
        }
    });

    let tax_rate = tax_resource
        .read()
        .as_ref()
        .and_then(|opt| opt.as_ref())
        .map(|cfg| cfg.rate)
        .unwrap_or(0.0);
    let tax_pct = format!("{:.1}%", tax_rate * 100.0);

    rsx! {
        div { class: "page page-product-detail",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                match &*product_resource.read() {
                    Some(Ok(detail)) => {
                        let spu = &detail.spu;
                        let name = if loc == "zh" { &spu.name_zh } else { &spu.name_en };
                        let desc = if loc == "zh" { spu.description_zh.as_deref().unwrap_or("") } else { spu.description_en.as_deref().unwrap_or("") };
                        let cat = spu.category.as_deref().unwrap_or("");
                        let base_price = spu.base_price;
                        let delta = options_delta();
                        let unit_price = base_price + delta;
                        let qty = quantity();
                        let line_total = unit_price * qty as f64;
                        let tax_amount = line_total * tax_rate;
                        let total_with_tax = line_total + tax_amount;
                        let prep = spu.prep_time_minutes;
                        let groups = detail.option_groups.clone();

                        let quantity_label = t.t(&loc, "label.quantity");
                        let tax_label = t.t(&loc, "label.tax");
                        let total_label = t.t(&loc, "label.total");
                        let add_text = t.t(&loc, "btn.add_to_cart");

                        rsx! {
                            section { class: "section product-detail-section",
                                div { class: "product-detail-layout",
                                    // Image column
                                    div { class: "product-detail-image",
                                        if let Some(ref img) = spu.image_url {
                                            img { src: "{img}", alt: "{name}", class: "product-img-large" }
                                        } else {
                                            div { class: "product-img-placeholder-large", "\u{2615}" }
                                        }
                                    }

                                    // Info column
                                    div { class: "product-detail-info",
                                        span { class: "product-category", "{cat}" }
                                        h1 { class: "product-name-large", "{name}" }
                                        p { class: "product-desc-full", "{desc}" }
                                        p { class: "product-prep-info",
                                            if loc == "zh" {
                                                "\u{5236}\u{4f5c}\u{65f6}\u{95f4}: {prep}\u{5206}\u{949f}"
                                            } else {
                                                "Prep time: {prep} min"
                                            }
                                        }

                                        // Base price
                                        div { class: "price-row",
                                            span { class: "price-label",
                                                if loc == "zh" { "\u{57fa}\u{7840}\u{4ef7}\u{683c}" } else { "Base price" }
                                            }
                                            PriceDisplay { amount: base_price, locale: locale.clone() }
                                        }

                                        // Option selector
                                        if !groups.is_empty() {
                                            OptionSelector {
                                                groups: groups,
                                                locale: locale.clone(),
                                                on_change: move |(ids, delta): (Vec<i64>, f64)| {
                                                    selected_options.set(ids);
                                                    options_delta.set(delta);
                                                },
                                            }
                                        }

                                        // Quantity selector
                                        div { class: "quantity-selector",
                                            label { class: "quantity-label", "{quantity_label}" }
                                            div { class: "quantity-controls",
                                                button {
                                                    class: "btn btn-sm",
                                                    disabled: qty <= 1,
                                                    onclick: move |_| {
                                                        if quantity() > 1 {
                                                            quantity.set(quantity() - 1);
                                                        }
                                                    },
                                                    "-"
                                                }
                                                span { class: "quantity-value", "{qty}" }
                                                button {
                                                    class: "btn btn-sm",
                                                    onclick: move |_| quantity.set(quantity() + 1),
                                                    "+"
                                                }
                                            }
                                        }

                                        // Price summary
                                        div { class: "price-summary",
                                            div { class: "price-row",
                                                span { class: "price-label",
                                                    if loc == "zh" { "\u{5355}\u{4ef7}" } else { "Unit price" }
                                                }
                                                PriceDisplay { amount: unit_price, locale: locale.clone() }
                                            }
                                            div { class: "price-row",
                                                span { class: "price-label", "{tax_label} ({tax_pct})" }
                                                PriceDisplay { amount: tax_amount, locale: locale.clone() }
                                            }
                                            div { class: "price-row price-row-total",
                                                span { class: "price-label", "{total_label}" }
                                                PriceDisplay { amount: total_with_tax, locale: locale.clone() }
                                            }
                                        }

                                        if let Some(err) = add_error() {
                                            div { class: "alert alert-error", "{err}" }
                                        }
                                        if add_success() {
                                            div { class: "alert alert-success",
                                                if loc == "zh" { "\u{5df2}\u{52a0}\u{5165}\u{8d2d}\u{7269}\u{8f66}\u{ff01}" } else { "Added to cart!" }
                                            }
                                        }

                                        // Add to cart button
                                        {
                                            let locale_cart = locale.clone();
                                            rsx! {
                                                button {
                                                    class: "btn btn-primary btn-lg btn-block",
                                                    disabled: adding(),
                                                    onclick: move |_| {
                                                        let session_cookie = app_state().auth.session_cookie.clone();
                                                        let opts = selected_options().clone();
                                                        let qty_val = quantity();
                                                        let locale_c = locale_cart.clone();
                                                        spawn(async move {
                                                            adding.set(true);
                                                            add_error.set(None);
                                                            add_success.set(false);

                                                            let body = AddToCartRequest {
                                                                sku_id: None,
                                                                spu_id: id,
                                                                selected_options: opts,
                                                                quantity: qty_val,
                                                            };

                                                            let mut req = reqwest::Client::new()
                                                                .post(&format!("{}/cart/add", crate::API_BASE))
                                                                .json(&body);

                                                            if let Some(ref sc) = session_cookie {
                                                                req = req.header("Cookie", format!("brewflow_session={}", sc));
                                                            }

                                                            match req.send().await {
                                                                Ok(resp) => {
                                                                    if resp.status().is_success() {
                                                                        add_success.set(true);
                                                                        // Update cart count
                                                                        let mut state = app_state.write();
                                                                        state.cart_count += qty_val;
                                                                    } else {
                                                                        let body = resp.text().await.unwrap_or_default();
                                                                        add_error.set(Some(format!("Failed: {}", body)));
                                                                    }
                                                                }
                                                                Err(e) => add_error.set(Some(format!("Network error: {}", e))),
                                                            }
                                                            adding.set(false);
                                                        });
                                                    },
                                                    if adding() { "..." } else { "{add_text}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! {
                        div { class: "section",
                            div { class: "alert alert-error", "Error: {e}" }
                        }
                    },
                    None => rsx! {
                        div { class: "section",
                            div { class: "loading-spinner", p { "Loading..." } }
                        }
                    },
                }
            }

            Footer {}
        }
    }
}
