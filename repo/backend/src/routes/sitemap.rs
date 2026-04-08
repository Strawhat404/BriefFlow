use rocket::{get, routes};
use rocket::response::content::{RawText, RawXml};

// ---------------------------------------------------------------------------
// Sitemap generation
// ---------------------------------------------------------------------------

/// Pages that should appear in the sitemap with localized variants.
const PAGES: &[&str] = &[
    "/",
    "/menu",
    "/cart",
    "/orders",
    "/training",
    "/login",
    "/register",
];

/// Read the base URL from the `SITEMAP_BASE_URL` environment variable at
/// request time so intranet deployments do not embed the wrong host.
/// Falls back to an empty string which produces site-root-relative URLs.
fn get_base_url() -> String {
    std::env::var("SITEMAP_BASE_URL").unwrap_or_default()
}

fn build_sitemap_xml(base_url: &str) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xhtml="http://www.w3.org/1999/xhtml">
"#,
    );

    for page in PAGES {
        let en_url = format!("{}/en{}", base_url, page);
        let zh_url = format!("{}/zh{}", base_url, page);

        // English version entry
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>{}</loc>\n", en_url));
        xml.push_str(&format!(
            "    <xhtml:link rel=\"alternate\" hreflang=\"en\" href=\"{}\" />\n",
            en_url
        ));
        xml.push_str(&format!(
            "    <xhtml:link rel=\"alternate\" hreflang=\"zh\" href=\"{}\" />\n",
            zh_url
        ));
        xml.push_str("    <changefreq>weekly</changefreq>\n");
        xml.push_str("    <priority>0.8</priority>\n");
        xml.push_str("  </url>\n");

        // Chinese version entry
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>{}</loc>\n", zh_url));
        xml.push_str(&format!(
            "    <xhtml:link rel=\"alternate\" hreflang=\"en\" href=\"{}\" />\n",
            en_url
        ));
        xml.push_str(&format!(
            "    <xhtml:link rel=\"alternate\" hreflang=\"zh\" href=\"{}\" />\n",
            zh_url
        ));
        xml.push_str("    <changefreq>weekly</changefreq>\n");
        xml.push_str("    <priority>0.8</priority>\n");
        xml.push_str("  </url>\n");
    }

    xml.push_str("</urlset>\n");
    xml
}

// ---------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------

#[get("/sitemap.xml")]
pub async fn sitemap() -> RawXml<String> {
    RawXml(build_sitemap_xml(&get_base_url()))
}

#[get("/robots.txt")]
pub async fn robots() -> RawText<String> {
    let base_url = get_base_url();
    let body = format!(
        "User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\n",
        base_url
    );
    RawText(body)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![sitemap, robots]
}
