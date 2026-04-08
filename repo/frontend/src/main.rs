mod components;
mod pages;
mod state;

use dioxus::prelude::*;

const API_BASE: &str = "http://localhost:8000/api";

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // Locale-prefixed customer routes
    #[route("/:locale")]
    Home { locale: String },
    #[route("/:locale/menu")]
    Menu { locale: String },
    #[route("/:locale/menu/:id")]
    ProductDetail { locale: String, id: i64 },
    #[route("/:locale/cart")]
    Cart { locale: String },
    #[route("/:locale/checkout")]
    Checkout { locale: String },
    #[route("/:locale/orders")]
    Orders { locale: String },
    #[route("/:locale/orders/:id")]
    OrderDetail { locale: String, id: i64 },

    // Auth routes
    #[route("/:locale/login")]
    Login { locale: String },
    #[route("/:locale/register")]
    Register { locale: String },

    // Staff routes
    #[route("/:locale/staff")]
    StaffDashboard { locale: String },
    #[route("/:locale/staff/orders/:id")]
    StaffOrderDetail { locale: String, id: i64 },
    #[route("/:locale/staff/scan")]
    StaffScan { locale: String },

    // Training routes
    #[route("/:locale/training")]
    Training { locale: String },
    #[route("/:locale/training/exams")]
    MockExams { locale: String },
    #[route("/:locale/training/exams/:id")]
    TakeExam { locale: String, id: i64 },
    #[route("/:locale/training/analytics")]
    Analytics { locale: String },
    #[route("/:locale/training/favorites")]
    Favorites { locale: String },
    #[route("/:locale/training/wrong-notebook")]
    WrongNotebook { locale: String },
    #[route("/:locale/training/review")]
    ReviewSession { locale: String },

    // Admin routes
    #[route("/:locale/admin")]
    Admin { locale: String },
    #[route("/:locale/admin/questions")]
    QuestionBank { locale: String },
    #[route("/:locale/admin/import")]
    ImportQuestions { locale: String },
    #[route("/:locale/admin/generate-exam")]
    GenerateExam { locale: String },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(state::AppState::default()));

    rsx! {
        Router::<Route> {}
    }
}

// ---- Page Components (dispatch to pages module) ----

#[component]
fn Home(locale: String) -> Element {
    rsx! { pages::home::HomePage { locale: locale } }
}

#[component]
fn Menu(locale: String) -> Element {
    rsx! { pages::menu::MenuPage { locale: locale } }
}

#[component]
fn ProductDetail(locale: String, id: i64) -> Element {
    rsx! { pages::product::ProductDetailPage { locale: locale, id: id } }
}

#[component]
fn Cart(locale: String) -> Element {
    rsx! { pages::cart::CartPage { locale: locale } }
}

#[component]
fn Checkout(locale: String) -> Element {
    rsx! { pages::checkout::CheckoutPage { locale: locale } }
}

#[component]
fn Orders(locale: String) -> Element {
    rsx! { pages::orders::OrdersPage { locale: locale } }
}

#[component]
fn OrderDetail(locale: String, id: i64) -> Element {
    rsx! { pages::orders::OrderDetailPage { locale: locale, id: id } }
}

#[component]
fn Login(locale: String) -> Element {
    rsx! { pages::auth::LoginPage { locale: locale } }
}

#[component]
fn Register(locale: String) -> Element {
    rsx! { pages::auth::RegisterPage { locale: locale } }
}

#[component]
fn StaffDashboard(locale: String) -> Element {
    rsx! { pages::staff::StaffDashboardPage { locale: locale } }
}

#[component]
fn StaffOrderDetail(locale: String, id: i64) -> Element {
    rsx! { pages::staff::StaffOrderDetailPage { locale: locale, id: id } }
}

#[component]
fn StaffScan(locale: String) -> Element {
    rsx! { pages::staff::StaffScanPage { locale: locale } }
}

#[component]
fn Training(locale: String) -> Element {
    rsx! { pages::training::TrainingPage { locale: locale } }
}

#[component]
fn MockExams(locale: String) -> Element {
    rsx! { pages::training::MockExamsPage { locale: locale } }
}

#[component]
fn TakeExam(locale: String, id: i64) -> Element {
    rsx! { pages::training::TakeExamPage { locale: locale, id: id } }
}

#[component]
fn Analytics(locale: String) -> Element {
    rsx! { pages::training::AnalyticsPage { locale: locale } }
}

#[component]
fn Favorites(locale: String) -> Element {
    rsx! { pages::training::FavoritesPage { locale: locale } }
}

#[component]
fn WrongNotebook(locale: String) -> Element {
    rsx! { pages::training::WrongNotebookPage { locale: locale } }
}

#[component]
fn ReviewSession(locale: String) -> Element {
    rsx! { pages::training::ReviewSessionPage { locale: locale } }
}

#[component]
fn Admin(locale: String) -> Element {
    rsx! { pages::admin::AdminPage { locale: locale } }
}

#[component]
fn QuestionBank(locale: String) -> Element {
    rsx! { pages::admin::QuestionBankPage { locale: locale } }
}

#[component]
fn ImportQuestions(locale: String) -> Element {
    rsx! { pages::admin::ImportQuestionsPage { locale: locale } }
}

#[component]
fn GenerateExam(locale: String) -> Element {
    rsx! { pages::admin::GenerateExamPage { locale: locale } }
}
