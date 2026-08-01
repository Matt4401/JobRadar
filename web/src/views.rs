use askama::Template;
use db::models::job_offer::JobOffer;
use scraper::parser::filters::match_filters::ScrapperFilters;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub offers: &'a [JobOffer],
}

#[derive(Template)]
#[template(path = "filters.html")]
pub struct FiltersTemplate<'a> {
    pub filters: &'a ScrapperFilters,
    pub min_salary_str: String,
}
