use async_trait::async_trait;
use db::models::job_offer::JobOffer;
use crate::parser::strategy::hellowork::HelloWorkParser;

#[async_trait]
pub trait SiteParser: Send + Sync {
    async fn parse(&self, html: &str) -> Result<Vec<JobOffer>, String>;
}

pub fn parser_factory(site_name: &str) -> Option<Box<dyn SiteParser>> {
    match site_name {
        "hello_work" => Some(Box::new(HelloWorkParser::new())),
        _ => None,
    }
}
