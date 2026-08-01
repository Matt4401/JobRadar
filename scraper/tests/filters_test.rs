// Integration tests for the offer filtering logic, exercised through the public
// `ScrapperFilters::matches` API.

use db::models::job_offer::JobOffer;
use scraper::parser::filters::match_filters::ScrapperFilters;

fn offer(title: &str, company: &str, location: &str) -> JobOffer {
    JobOffer {
        title: title.to_string(),
        company: company.to_string(),
        location: location.to_string(),
        ..Default::default()
    }
}

fn filters() -> ScrapperFilters {
    ScrapperFilters::default()
}

#[test]
fn default_filters_accept_any_offer() {
    let f = filters();
    assert!(f.matches(&offer("Anything", "Some Corp", "Anywhere")));
}

#[test]
fn keyword_filter_requires_at_least_one_match() {
    let mut f = filters();
    f.job_keywords = vec!["développeur".to_string(), "data".to_string()];

    assert!(f.matches(&offer("Développeur Rust", "ACME", "Nantes")));
    assert!(f.matches(&offer("Data Engineer", "ACME", "Nantes")));
    assert!(!f.matches(&offer("Commercial B2B", "ACME", "Nantes")));
}

#[test]
fn keyword_matching_is_case_insensitive() {
    let mut f = filters();
    f.job_keywords = vec!["Développeur".to_string()];
    assert!(f.matches(&offer("DÉVELOPPEUR WEB", "ACME", "Nantes")));
}

#[test]
fn location_filter_rejects_other_cities() {
    let mut f = filters();
    f.locations = vec!["Nantes".to_string()];

    assert!(f.matches(&offer("Dev", "ACME", "Nantes, Pays de la Loire")));
    assert!(!f.matches(&offer("Dev", "ACME", "Paris")));
}

#[test]
fn company_filter_keeps_only_listed_companies() {
    let mut f = filters();
    f.companies = vec!["ACME".to_string()];

    assert!(f.matches(&offer("Dev", "ACME Corp", "Nantes")));
    assert!(!f.matches(&offer("Dev", "Globex", "Nantes")));
}

#[test]
fn contract_filter_empty_accepts_all_including_missing() {
    let f = filters();

    let mut with_contract = offer("Dev", "ACME", "Nantes");
    with_contract.contract_type = Some("INTERN".to_string());
    assert!(f.matches(&with_contract));

    // Missing contract type is accepted when the filter is empty.
    assert!(f.matches(&offer("Dev", "ACME", "Nantes")));
}

#[test]
fn contract_filter_rejects_missing_and_non_matching_when_set() {
    let mut f = filters();
    f.contract_types = vec!["INTERN".to_string()];

    let mut matching = offer("Dev", "ACME", "Nantes");
    matching.contract_type = Some("intern, full_time".to_string());
    assert!(f.matches(&matching));

    let mut non_matching = offer("Dev", "ACME", "Nantes");
    non_matching.contract_type = Some("FULL_TIME".to_string());
    assert!(!f.matches(&non_matching));

    // Missing contract type is rejected when the filter is set.
    assert!(!f.matches(&offer("Dev", "ACME", "Nantes")));
}

#[test]
fn salary_filter_disabled_without_min() {
    let f = filters();

    let mut with_salary = offer("Dev", "ACME", "Nantes");
    with_salary.salary = Some("1000 €".to_string());
    assert!(f.matches(&with_salary));

    assert!(f.matches(&offer("Dev", "ACME", "Nantes")));
}

#[test]
fn salary_filter_respects_min_and_parses_spaces() {
    let mut f = filters();
    f.min_salary = Some(30000.0);

    let mut high = offer("Dev", "ACME", "Nantes");
    high.salary = Some("35 000 € / an".to_string());
    assert!(f.matches(&high));

    let mut low = offer("Dev", "ACME", "Nantes");
    low.salary = Some("25 000 €".to_string());
    assert!(!f.matches(&low));
}

#[test]
fn salary_filter_parses_nbsp_thousands_separator() {
    let mut f = filters();
    f.min_salary = Some(30000.0);

    let mut nbsp = offer("Dev", "ACME", "Nantes");
    nbsp.salary = Some("30\u{a0}000 € brut".to_string());
    assert!(f.matches(&nbsp));
}

#[test]
fn missing_salary_kept_unless_exclusion_enabled() {
    let mut f = filters();
    f.min_salary = Some(30000.0);

    // No salary info: kept by default.
    assert!(f.matches(&offer("Dev", "ACME", "Nantes")));

    // No salary info: rejected when exclusion is enabled.
    f.exclude_without_salary = true;
    assert!(!f.matches(&offer("Dev", "ACME", "Nantes")));
}

#[test]
fn matches_combines_all_criteria_with_and() {
    let mut f = filters();
    f.job_keywords = vec!["développeur".to_string()];
    f.locations = vec!["Nantes".to_string()];
    f.contract_types = vec!["INTERN".to_string()];

    let mut ok = offer("Développeur web", "ACME", "Nantes, Pays de la Loire");
    ok.contract_type = Some("INTERN".to_string());
    assert!(f.matches(&ok));

    let mut wrong_location = ok.clone();
    wrong_location.location = "Paris".to_string();
    assert!(!f.matches(&wrong_location));

    let mut wrong_title = ok.clone();
    wrong_title.title = "Comptable".to_string();
    assert!(!f.matches(&wrong_title));
}
