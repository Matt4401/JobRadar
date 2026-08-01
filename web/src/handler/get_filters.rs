use crate::render_filters;
use axum::response::Html;

pub async fn filters_page() -> Html<String> {
    let filters = scraper::parser::filters::update_filters::load_filters();
    Html(render_filters(&filters))
}
