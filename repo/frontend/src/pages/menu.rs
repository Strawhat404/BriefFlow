use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::components::price_display::PriceDisplay;
use shared::dto::{ApiResponse, ProductListItem};

#[component]
pub fn MenuPage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();

    let mut category_filter = use_signal(|| String::new());

    let products_resource = use_resource(move || {
        async move {
            let url = format!("{}/products", crate::API_BASE);
            let resp = reqwest::Client::new()
                .get(&url)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            let data: ApiResponse<Vec<ProductListItem>> = resp
                .json()
                .await
                .map_err(|e| e.to_string())?;
            data.data.ok_or_else(|| "No data returned".to_string())
        }
    });

    let page_title = t.t(&loc, "page.menu");

    rsx! {
        div { class: "page page-menu",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                section { class: "section",
                    h2 { class: "section-title", "{page_title}" }

                    // Category filter bar
                    match &*products_resource.read() {
                        Some(Ok(products)) => {
                            let mut categories: Vec<String> = products
                                .iter()
                                .filter_map(|p| p.category.clone())
                                .collect::<std::collections::HashSet<_>>()
                                .into_iter()
                                .collect();
                            categories.sort();

                            let current_filter = category_filter();
                            let filtered: Vec<&ProductListItem> = products
                                .iter()
                                .filter(|p| {
                                    if current_filter.is_empty() {
                                        true
                                    } else {
                                        p.category.as_deref().unwrap_or("") == current_filter.as_str()
                                    }
                                })
                                .collect();

                            rsx! {
                                div { class: "filter-bar",
                                    button {
                                        class: if current_filter.is_empty() { "filter-btn filter-btn-active" } else { "filter-btn" },
                                        onclick: move |_| category_filter.set(String::new()),
                                        if loc == "zh" { "\u{5168}\u{90e8}" } else { "All" }
                                    }
                                    for cat in categories.iter() {
                                        {
                                            let cat_clone = cat.clone();
                                            let is_active = current_filter == *cat;
                                            rsx! {
                                                button {
                                                    class: if is_active { "filter-btn filter-btn-active" } else { "filter-btn" },
                                                    onclick: move |_| category_filter.set(cat_clone.clone()),
                                                    "{cat}"
                                                }
                                            }
                                        }
                                    }
                                }

                                if filtered.is_empty() {
                                    p { class: "empty-text",
                                        if loc == "zh" { "\u{6ca1}\u{6709}\u{627e}\u{5230}\u{4ea7}\u{54c1}" } else { "No products found" }
                                    }
                                }

                                div { class: "product-grid",
                                    for product in filtered.iter() {
                                        {
                                            let name = if loc == "zh" { &product.name_zh } else { &product.name_en };
                                            let desc = if loc == "zh" { product.description_zh.as_deref().unwrap_or("") } else { product.description_en.as_deref().unwrap_or("") };
                                            let cat = product.category.as_deref().unwrap_or("");
                                            let pid = product.spu_id;
                                            let price = product.base_price;
                                            let prep = product.prep_time_minutes;
                                            rsx! {
                                                Link {
                                                    to: crate::Route::ProductDetail { locale: locale.clone(), id: pid },
                                                    class: "product-card product-card-link",
                                                    div { class: "product-card-image",
                                                        if let Some(ref img) = product.image_url {
                                                            img { src: "{img}", alt: "{name}", class: "product-img" }
                                                        } else {
                                                            div { class: "product-img-placeholder", "\u{2615}" }
                                                        }
                                                    }
                                                    div { class: "product-card-body",
                                                        span { class: "product-category", "{cat}" }
                                                        h3 { class: "product-name", "{name}" }
                                                        p { class: "product-desc product-desc-truncated", "{desc}" }
                                                        div { class: "product-card-footer",
                                                            PriceDisplay { amount: price, locale: locale.clone() }
                                                            span { class: "product-prep-time",
                                                                if loc == "zh" {
                                                                    "{prep}\u{5206}\u{949f}"
                                                                } else {
                                                                    "{prep} min"
                                                                }
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
                            div { class: "alert alert-error", "Failed to load menu: {e}" }
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
