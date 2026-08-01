use crate::{
    state::AppState,
    views::{FiltersTemplate, IndexTemplate},
};
use askama::Template;
use axum::{extract::State, http::StatusCode, response::Html, response::IntoResponse};

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let offers = scraper::core::fetch_stored_offers(&state.db)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Error fetching stored offers: {e}");
            Vec::new()
        });
    let template = IndexTemplate { offers: &offers };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error (index): {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error rendering template",
            )
                .into_response()
        }
    }
}
