use axum::{
    routing::{get, post},
    Router,
};

async fn get_router() -> axum::Router {
    Router::new()
        .route("/", get(index))
        .route("/rescan", post(rescan))
        .route("/filters", get(filters_page))
        .route("/filters", post(update_filters))
}
