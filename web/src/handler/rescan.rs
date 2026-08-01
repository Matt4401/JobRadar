use axum::response::{IntoResponse, Redirect};

async fn rescan() -> impl IntoResponse {
    match scraper::scrape_and_store().await {
        Ok(n) => println!("Search completed successfully. {n} new offers found."),
        Err(e) => eprintln!("Error during search: {e}. No new offers were found."),
    }
    Redirect::to("/")
}
