use db::connection::Database;
use db::migration_cli::migration_cli::establish_connection;
use db::models::job_offer::JobOffer;
use serde::Deserialize;
use shared::get_html_from_url;

use crate::http_client::init_http_client::http_client;
use crate::parser::filters::match_filters::ScrapperFilters;
use crate::parser::filters::update_filters::{load_config, load_filters};
use crate::parser::parser::parser_factory;
use crate::save_offers::store_offers;
use crate::sites_data::urls::URL_FOR_SITE;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScrapperConfig {
    #[serde(alias = "sites")]
    pub sites_list: Vec<String>,
    pub log_system: bool,
    pub email_system: bool,
    pub crash_on_scrapping_errors: bool,
}

pub async fn scrape_all(filters: &ScrapperFilters) -> Vec<JobOffer> {
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {e}. No sites will be scraped.");
            return Vec::new();
        }
    };
    let mut all_offers: Vec<JobOffer> = Vec::new();

    for site in &config.sites_list {
        let Some(parser) = parser_factory(site) else {
            eprintln!("No parser implemented for site '{site}'");
            continue;
        };
        let Some(url) = URL_FOR_SITE.get(site.as_str()) else {
            eprintln!("No URL configured for site '{site}'");
            continue;
        };
        let client = match http_client() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };
        let html = match get_html_from_url(url, client).await {
            Ok(html) => html,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        match parser.parse(&html, filters).await {
            Ok(offers) => {
                println!("{} offers scraped from {site}", offers.len());
                all_offers.extend(offers);
            }
            Err(e) => eprintln!("Error parsing offers from {site}: {e}"),
        }
    }

    all_offers
}

pub async fn fetch_stored_offers(db: &Database) -> Result<Vec<JobOffer>, String> {
    let query = "SELECT * FROM job_offers ORDER BY created_at DESC";

    db.fetch_all_as::<JobOffer>(query, &[])
        .await
        .map_err(|e| format!("Error fetching stored offers: {e}"))
}

pub async fn scrape_and_store() -> Result<usize, String> {
    let filters = load_filters();
    let offers = scrape_all(&filters).await;

    if offers.is_empty() {
        println!("No offers scraped. Nothing to store.");
        return Ok(0);
    }
    let db = establish_connection().await;
    let stored = store_offers(&db, &offers).await?;

    println!("{stored} offers stored in the database.");
    Ok(stored)
}
