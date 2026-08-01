use crate::handler::get_filters::filters_page;
use crate::handler::index::index;
use crate::handler::rescan::rescan;
use crate::handler::update_filters::update_filters;
use crate::routing::paths::{FILTERS_PATH, RESCAN_PATH, ROOT_PATH};
use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub async fn get_router(state: AppState) -> axum::Router {
    Router::new()
        .route(ROOT_PATH, get(index))
        .route(RESCAN_PATH, post(rescan))
        .route(FILTERS_PATH, get(filters_page))
        .route(FILTERS_PATH, post(update_filters))
        .with_state(state)
}
