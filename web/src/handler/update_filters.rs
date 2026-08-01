use crate::FiltersForm;
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use scraper::parser::filters::match_filters::ScrapperFilters;

fn split_list(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

pub async fn update_filters(Form(form): Form<FiltersForm>) -> impl IntoResponse {
    let filters = ScrapperFilters {
        max_offers: form.max_offers.trim().parse().unwrap_or(6),
        job_keywords: split_list(&form.job_keywords),
        companies: split_list(&form.companies),
        locations: split_list(&form.locations),
        contract_types: split_list(&form.contract_types),
        min_salary: {
            let s = form.min_salary.trim().replace(',', ".");
            if s.is_empty() {
                None
            } else {
                s.parse::<f64>().ok().filter(|v| *v > 0.0)
            }
        },
        exclude_without_salary: form.exclude_without_salary.is_some(),
    };

    if let Err(e) = scraper::parser::filters::update_filters::save_filters(&filters) {
        eprintln!("Error saving filters: {e}");
    }
    Redirect::to("/filters")
}
