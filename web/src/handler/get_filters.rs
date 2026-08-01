use crate::views::FiltersTemplate;
use askama::Template;
use axum::{http::StatusCode, response::Html, response::IntoResponse};

pub async fn filters_page() -> impl IntoResponse {
    let filters = scraper::parser::filters::update_filters::load_filters();
    let min_salary_str = filters
        .min_salary
        .map(|v| v.to_string())
        .unwrap_or_default();

    let template = FiltersTemplate {
        filters: &filters,
        min_salary_str,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error (filters): {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error rendering template",
            )
                .into_response()
        }
    }
}
