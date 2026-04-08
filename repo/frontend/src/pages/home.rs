use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::components::price_display::PriceDisplay;
use shared::dto::{ApiResponse, ProductListItem};
use shared::models::StoreHours;

#[component]
pub fn HomePage(locale: String) -> Element {
    let t = shared::i18n::init_translations();
    let loc = locale.clone();

    let featured = use_resource(move || {
        let locale = loc.clone();
        async move {
            let url = format!("{}/products?featured=true&limit=3", crate::API_BASE);
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

    let store_hours = use_resource(move || async move {
        let url = format!("{}/store/hours", crate::API_BASE);
        let resp = reqwest::Client::new()
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let data: ApiResponse<Vec<StoreHours>> = resp
            .json()
            .await
            .map_err(|e| e.to_string())?;
        data.data.ok_or_else(|| "No hours data".to_string())
    });

    let loc = locale.clone();
    let hero_title = if loc == "zh" { "BrewFlow - \u{60a8}\u{7684}\u{667a}\u{80fd}\u{5496}\u{5561}\u{4f34}\u{4fa3}" } else { "BrewFlow - Your Smart Coffee Companion" };
    let hero_subtitle = if loc == "zh" { "\u{7ebf}\u{4e0a}\u{70b9}\u{5355}\u{ff0c}\u{5230}\u{5e97}\u{53d6}\u{9910}\u{ff0c}\u{667a}\u{80fd}\u{57f9}\u{8bad}" } else { "Order online, pick up in-store, train smarter" };
    let featured_title = t.t(&loc, "nav.menu");
    let hours_title = if loc == "zh" { "\u{8425}\u{4e1a}\u{65f6}\u{95f4}" } else { "Store Hours" };
    let menu_link_text = t.t(&loc, "nav.menu");
    let training_link_text = t.t(&loc, "nav.training");

    rsx! {
        div { class: "page page-home",
            Navbar { locale: locale.clone() }

            main { class: "main-content",
                // Hero section
                section { class: "hero",
                    div { class: "hero-content",
                        h1 { class: "hero-title", "{hero_title}" }
                        p { class: "hero-subtitle", "{hero_subtitle}" }
                        div { class: "hero-actions",
                            Link {
                                to: crate::Route::Menu { locale: locale.clone() },
                                class: "btn btn-primary btn-lg",
                                "{menu_link_text}"
                            }
                            Link {
                                to: crate::Route::Training { locale: locale.clone() },
                                class: "btn btn-secondary btn-lg",
                                "{training_link_text}"
                            }
                        }
                    }
                }

                // Featured products
                section { class: "section featured-section",
                    h2 { class: "section-title", "{featured_title}" }
                    div { class: "product-grid product-grid-featured",
                        match &*featured.read() {
                            Some(Ok(products)) => rsx! {
                                for product in products.iter() {
                                    {
                                        let name = if loc == "zh" { &product.name_zh } else { &product.name_en };
                                        let desc = if loc == "zh" { product.description_zh.as_deref().unwrap_or("") } else { product.description_en.as_deref().unwrap_or("") };
                                        let cat = product.category.as_deref().unwrap_or("");
                                        let pid = product.spu_id;
                                        let price = product.base_price;
                                        rsx! {
                                            div { class: "product-card",
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
                                                    p { class: "product-desc", "{desc}" }
                                                    div { class: "product-card-footer",
                                                        PriceDisplay { amount: price, locale: locale.clone() }
                                                        Link {
                                                            to: crate::Route::ProductDetail { locale: locale.clone(), id: pid },
                                                            class: "btn btn-sm btn-primary",
                                                            if loc == "zh" { "\u{67e5}\u{770b}" } else { "View" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            Some(Err(e)) => rsx! {
                                p { class: "error-text", "Failed to load featured products: {e}" }
                            },
                            None => rsx! {
                                div { class: "loading-spinner",
                                    p { "Loading..." }
                                }
                            },
                        }
                    }
                }

                // Store hours — loaded from /api/store/hours
                section { class: "section store-hours-section",
                    h2 { class: "section-title", "{hours_title}" }
                    div { class: "store-hours-grid",
                        match &*store_hours.read() {
                            Some(Ok(hours_list)) => rsx! {
                                for h in hours_list.iter() {
                                    {
                                        let day_name = match h.day_of_week {
                                            1 => if loc == "zh" { "\u{5468}\u{4e00}" } else { "Monday" },
                                            2 => if loc == "zh" { "\u{5468}\u{4e8c}" } else { "Tuesday" },
                                            3 => if loc == "zh" { "\u{5468}\u{4e09}" } else { "Wednesday" },
                                            4 => if loc == "zh" { "\u{5468}\u{56db}" } else { "Thursday" },
                                            5 => if loc == "zh" { "\u{5468}\u{4e94}" } else { "Friday" },
                                            6 => if loc == "zh" { "\u{5468}\u{516d}" } else { "Saturday" },
                                            7 => if loc == "zh" { "\u{5468}\u{65e5}" } else { "Sunday" },
                                            _ => "?",
                                        };
                                        let time_str = if h.is_closed {
                                            if loc == "zh" { "\u{4f11}\u{606f}" } else { "Closed" }
                                        } else {
                                            ""
                                        };
                                        let open = h.open_time.clone();
                                        let close = h.close_time.clone();
                                        rsx! {
                                            div { class: "store-hours-row",
                                                span { class: "store-hours-day", "{day_name}" }
                                                span { class: "store-hours-time",
                                                    if h.is_closed {
                                                        "{time_str}"
                                                    } else {
                                                        "{open} - {close}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            Some(Err(_)) | None => rsx! {
                                p { class: "store-hours-loading",
                                    if loc == "zh" { "\u{52a0}\u{8f7d}\u{4e2d}..." } else { "Loading hours..." }
                                }
                            },
                        }
                    }
                }

                // Quick links
                section { class: "section quick-links-section",
                    div { class: "quick-links",
                        Link {
                            to: crate::Route::Menu { locale: locale.clone() },
                            class: "quick-link-card",
                            div { class: "quick-link-icon", "\u{2615}" }
                            h3 { "{menu_link_text}" }
                        }
                        Link {
                            to: crate::Route::Training { locale: locale.clone() },
                            class: "quick-link-card",
                            div { class: "quick-link-icon", "\u{1f4da}" }
                            h3 { "{training_link_text}" }
                        }
                        Link {
                            to: crate::Route::Orders { locale: locale.clone() },
                            class: "quick-link-card",
                            div { class: "quick-link-icon", "\u{1f4cb}" }
                            h3 { "{t.t(&loc, \"nav.orders\")}" }
                        }
                    }
                }
            }

            Footer {}
        }
    }
}
