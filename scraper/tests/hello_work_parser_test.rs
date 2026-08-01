// Integration tests for the HelloWork parser primitives (no network access).

use scraper::parser::strategy::hello_work::parser_hello_work_json::HelloWorkParser;
use serde_json::json;

#[test]
fn extract_offer_paths_finds_and_dedupes_links() {
    let html = r#"
        <a href="/fr-fr/emplois/77024967.html">A</a>
        <a href="/fr-fr/emplois/77024967.html">dup</a>
        <a href="/fr-fr/emplois/80781085-dev-web.html">B</a>
        <a href="/fr-fr/entreprise/acme.html">not an offer</a>
    "#;
    let paths = HelloWorkParser::extract_offer_paths(html);
    assert_eq!(
        paths,
        vec![
            "/fr-fr/emplois/77024967.html".to_string(),
            "/fr-fr/emplois/80781085-dev-web.html".to_string(),
        ]
    );
}

#[test]
fn extract_offer_paths_empty_when_none() {
    assert!(HelloWorkParser::extract_offer_paths("<html>nothing</html>").is_empty());
}

#[test]
fn json_ld_blocks_parses_valid_and_skips_invalid() {
    let html = r#"
        <script type="application/ld+json">{"@type":"JobPosting","title":"Dev"}</script>
        <script type="application/ld+json">{ invalid json }</script>
        <script type="application/ld+json">{"@type":"BreadcrumbList"}</script>
    "#;
    let blocks = HelloWorkParser::json_ld_blocks(html);
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0]["@type"], "JobPosting");
    assert_eq!(blocks[1]["@type"], "BreadcrumbList");
}

#[test]
fn json_ld_to_offer_maps_all_fields() {
    let job = json!({
        "@type": "JobPosting",
        "title": "Développeur Rust",
        "description": "Nice mission",
        "url": "https://www.hellowork.com/fr-fr/emplois/123.html",
        "datePosted": "2026-07-15T09:00:00+02:00",
        "employmentType": ["INTERN", "FULL_TIME"],
        "hiringOrganization": { "name": "ACME" },
        "jobLocation": [{
            "address": {
                "addressLocality": "Nantes",
                "addressRegion": "Pays de la Loire"
            }
        }],
        "baseSalary": {
            "currency": "EUR",
            "value": { "value": 35000, "unitText": "YEAR" }
        },
        "educationRequirements": { "credentialCategory": "bac+5" },
        "experienceRequirements": "1 year"
    });

    let offer = HelloWorkParser::json_ld_to_offer(&job);
    assert_eq!(offer.title, "Développeur Rust");
    assert_eq!(offer.description, "Nice mission");
    assert_eq!(offer.company, "ACME");
    assert_eq!(offer.location, "Nantes, Pays de la Loire");
    assert_eq!(
        offer.url,
        "https://www.hellowork.com/fr-fr/emplois/123.html"
    );
    assert_eq!(offer.contract_type, Some("INTERN, FULL_TIME".to_string()));
    assert_eq!(offer.salary, Some("35000 EUR / YEAR".to_string()));
    assert_eq!(offer.study_level, Some("bac+5".to_string()));
    assert_eq!(offer.experience_level, Some("1 year".to_string()));
    assert_eq!(
        offer.created_at.format("%Y-%m-%d").to_string(),
        "2026-07-15"
    );
}

#[test]
fn json_ld_to_offer_defaults_missing_fields() {
    let job = json!({ "@type": "JobPosting" });
    let offer = HelloWorkParser::json_ld_to_offer(&job);
    assert_eq!(offer.title, "");
    assert_eq!(offer.company, "");
    assert_eq!(offer.location, "");
    assert_eq!(offer.contract_type, None);
    assert_eq!(offer.salary, None);
}

#[test]
fn json_ld_to_offer_location_region_only() {
    let job = json!({
        "@type": "JobPosting",
        "jobLocation": { "address": { "addressRegion": "Bretagne" } }
    });
    let offer = HelloWorkParser::json_ld_to_offer(&job);
    assert_eq!(offer.location, "Bretagne");
}
