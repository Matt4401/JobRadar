use crate::filters::ScrapperFilters;
use crate::http_client::init_http_client::http_client;
use crate::parser::parser::SiteParser;
use crate::parser::strategy::hello_work::parser_hello_work_json::HelloWorkParser;
use crate::parser::strategy::hello_work::url::get_full_url;
use async_trait::async_trait;
use db::models::job_offer::JobOffer;

#[async_trait]
impl SiteParser for HelloWorkParser {
    async fn parse(&self, html: &str, filters: &ScrapperFilters) -> Result<Vec<JobOffer>, String> {
        let client = http_client()?;
        let paths = Self::extract_offer_paths(html);

        if paths.is_empty() {
            return Err("None of the offer links found on the listing page".to_string());
        }
        let mut offers = Vec::new();

        for path in paths {
            if offers.len() >= filters.max_offers {
                break;
            }

            let detail_url = get_full_url(&path);
            let detail_html = match client.get(&detail_url).send().await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("Error reading body {detail_url} : {e}");
                        continue;
                    }
                },
                Err(e) => {
                    eprintln!("Error request {detail_url} : {e}");
                    continue;
                }
            };

            match Self::json_ld_blocks(&detail_html)
                .iter()
                .find(|b| b["@type"] == "JobPosting")
            {
                Some(job) => {
                    let offer = Self::json_ld_to_offer(job);
                    if filters.matches(&offer) {
                        offers.push(offer);
                    }
                }
                None => eprintln!("No JobPosting JSON-LD found on {detail_url}"),
            }
        }
        Ok(offers)
    }
}
