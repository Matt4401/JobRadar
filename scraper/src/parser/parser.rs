use async_trait::async_trait;
use db::models::job_offer::JobOffer;
use crate::parser::filters::filters::ScrapperFilters;
use crate::parser::strategy::hello_work::parser_hello_work_json::HelloWorkParser;

#[async_trait]
pub trait SiteParser: Send + Sync {
    async fn parse(&self, html: &str, filters: &ScrapperFilters) -> Result<Vec<JobOffer>, String>;
}

pub fn parser_factory(site_name: &str) -> Option<Box<dyn SiteParser>> {
    match site_name {
        "hello_work" => Some(Box::new(HelloWorkParser::new())),
        _ => None,
    }
}
