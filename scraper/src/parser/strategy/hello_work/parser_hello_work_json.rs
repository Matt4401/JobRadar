use crate::parser::strategy::hello_work::json_helpers::{
    base_salary, employment_type, first_item, str_field,
};
use chrono::{DateTime, Utc};
use db::models::job_offer::JobOffer;
use regex::Regex;
use serde_json::Value;

pub struct HelloWorkParser;

impl HelloWorkParser {
    pub fn new() -> Self {
        HelloWorkParser
    }

    /// Extract all unique offer paths from the listing page HTML.
    pub fn extract_offer_paths(html: &str) -> Vec<String> {
        let re = Regex::new(r"/fr-fr/emplois/\d+[a-z0-9._-]*\.html").unwrap();
        let mut seen: Vec<String> = Vec::new();
        for m in re.find_iter(html) {
            let path = m.as_str().to_string();
            if !seen.contains(&path) {
                seen.push(path);
            }
        }
        seen
    }

    /// Extract all JSON-LD blocks from the HTML and parse them into `serde_json::Value`.
    pub fn json_ld_blocks(html: &str) -> Vec<Value> {
        let re = Regex::new(r#"(?s)<script type="application/ld\+json">(.*?)</script>"#).unwrap();
        re.captures_iter(html)
            .filter_map(|c| serde_json::from_str::<Value>(c[1].trim()).ok())
            .collect()
    }

    /// Convert a JSON-LD `JobPosting` object into a `JobOffer`.
    pub fn json_ld_to_offer(job: &Value) -> JobOffer {
        let title = str_field(job, "title").unwrap_or_default();
        let description = str_field(job, "description").unwrap_or_default();
        let url = str_field(job, "url").unwrap_or_default();
        let company = str_field(&job["hiringOrganization"], "name").unwrap_or_default();
        let location = {
            let place = first_item(&job["jobLocation"]);
            let addr = &place["address"];
            let locality = str_field(addr, "addressLocality");
            let region = str_field(addr, "addressRegion");
            match (locality, region) {
                (Some(l), Some(r)) => format!("{l}, {r}"),
                (Some(l), None) => l,
                (None, Some(r)) => r,
                (None, None) => String::new(),
            }
        };
        let created_at = str_field(job, "datePosted")
            .and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
            .map(|dt| dt.naive_utc())
            .unwrap_or_else(|| Utc::now().naive_utc());
        let contract_type = employment_type(&job["employmentType"]);
        let salary = base_salary(&job["baseSalary"]);
        let study_level = str_field(&job["educationRequirements"], "credentialCategory");
        let experience_level = job
            .get("experienceRequirements")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        JobOffer {
            title,
            description,
            company,
            location,
            url,
            created_at,
            salary,
            study_level,
            contract_type,
            experience_level,
            ..Default::default()
        }
    }
}
