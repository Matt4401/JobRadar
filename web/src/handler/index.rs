use crate::{render_index, state::AppState};
use axum::{extract::State, response::Html};

async fn index(State(state): State<AppState>) -> Html<String> {
    let offers = scraper::core::fetch_stored_offers(&state.db)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Error fetching stored offers: {e}");
            Vec::new()
        });
    Html(render_index(&offers))
}
