use dioxus::prelude::*;
use crate::state::AppState;
use super::locale_switcher::LocaleSwitcher;

#[component]
pub fn Navbar(locale: String) -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let t = shared::i18n::init_translations();
    let loc = locale.as_str();

    let display_name = state()
        .auth
        .user
        .as_ref()
        .map(|u| u.display_name.clone().unwrap_or(u.username.clone()))
        .unwrap_or_default();

    let is_authenticated = state().auth.is_authenticated;
    let cart_count = state().cart_count;
    let is_staff = state().is_staff();
    let is_teacher = state().is_teacher();
    let is_admin = state().is_admin();

    let nav_menu = t.t(loc, "nav.menu");
    let nav_cart = t.t(loc, "nav.cart");
    let nav_orders = t.t(loc, "nav.orders");
    let nav_staff = t.t(loc, "nav.staff");
    let nav_training = t.t(loc, "nav.training");
    let nav_admin = t.t(loc, "nav.admin");

    let locale_logout = locale.clone();
    let locale_login = locale.clone();

    rsx! {
        nav { class: "navbar",
            div { class: "navbar-brand",
                Link { to: crate::Route::Home { locale: locale.clone() },
                    h1 { class: "brand-title", "BrewFlow" }
                }
            }
            div { class: "navbar-menu",
                Link { to: crate::Route::Menu { locale: locale.clone() },
                    class: "nav-link",
                    "{nav_menu}"
                }
                Link { to: crate::Route::Cart { locale: locale.clone() },
                    class: "nav-link",
                    "{nav_cart}"
                    if cart_count > 0 {
                        span { class: "badge", "{cart_count}" }
                    }
                }
                Link { to: crate::Route::Orders { locale: locale.clone() },
                    class: "nav-link",
                    "{nav_orders}"
                }
                if is_staff {
                    Link { to: crate::Route::StaffDashboard { locale: locale.clone() },
                        class: "nav-link",
                        "{nav_staff}"
                    }
                }
                if is_teacher {
                    Link { to: crate::Route::Training { locale: locale.clone() },
                        class: "nav-link",
                        "{nav_training}"
                    }
                }
                if is_admin {
                    Link { to: crate::Route::Admin { locale: locale.clone() },
                        class: "nav-link",
                        "{nav_admin}"
                    }
                }
            }
            div { class: "navbar-end",
                LocaleSwitcher { current_locale: locale.clone() }
                if is_authenticated {
                    span { class: "user-name", "{display_name}" }
                    button {
                        class: "btn btn-sm",
                        onclick: move |_| {
                            let mut s = state.write();
                            s.logout();
                        },
                        "Logout"
                    }
                } else {
                    Link { to: crate::Route::Login { locale: locale_login.clone() },
                        class: "btn btn-primary btn-sm",
                        "Login"
                    }
                }
            }
        }
    }
}
