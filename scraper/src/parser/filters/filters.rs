use db::models::job_offer::JobOffer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct ScrapperFilters {
    pub max_offers: usize,
    pub job_keywords: Vec<String>,
    pub companies: Vec<String>,
    pub locations: Vec<String>,
    pub contract_types: Vec<String>,
    pub min_salary: Option<f64>,
    pub exclude_without_salary: bool,
}

impl Default for ScrapperFilters {
    fn default() -> Self {
        Self {
            max_offers: 3,
            job_keywords: Vec::new(),
            companies: Vec::new(),
            locations: Vec::new(),
            contract_types: Vec::new(),
            min_salary: None,
            exclude_without_salary: false,
        }
    }
}

impl ScrapperFilters {
    pub fn matches(&self, offer: &JobOffer) -> bool {
        self.matches_keywords(&offer.title)
            && self.matches_companies(&offer.company)
            && self.matches_locations(&offer.location)
            && self.matches_contract(offer.contract_type.as_deref())
            && self.matches_salary(offer.salary.as_deref())
    }

    fn matches_keywords(&self, title: &str) -> bool {
        contains_any(&self.job_keywords, title)
    }

    fn matches_companies(&self, company: &str) -> bool {
        contains_any(&self.companies, company)
    }

    fn matches_locations(&self, location: &str) -> bool {
        contains_any(&self.locations, location)
    }

    fn matches_contract(&self, contract: Option<&str>) -> bool {
        if self.contract_types.is_empty() {
            return true;
        }
        let Some(contract) = contract else {
            return false;
        };
        let contract_lc = contract.to_lowercase();
        self.contract_types
            .iter()
            .any(|t| contract_lc.contains(&t.to_lowercase()))
    }

    fn matches_salary(&self, salary: Option<&str>) -> bool {
        let Some(min) = self.min_salary else {
            return true;
        };
        if min <= 0.0 {
            return true;
        }
        match salary.and_then(parse_min_salary) {
            Some(value) => value >= min,
            None => !self.exclude_without_salary,
        }
    }
}

// Return true if any of the patterns is found in the haystack (case-insensitive). If patterns is empty, return true.
fn contains_any(patterns: &[String], haystack: &str) -> bool {
    if patterns.is_empty() {
        return true;
    }
    let haystack_lc = haystack.to_lowercase();
    patterns
        .iter()
        .any(|p| haystack_lc.contains(&p.to_lowercase()))
}

// Extract the first sequence of digits from a string and parse it as f64. Returns None if no digits found or parsing fails.
fn parse_min_salary(s: &str) -> Option<f64> {
    let mut digits = String::new();
    let mut started = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
            started = true;
        } else if started && (c == ' ' || c == '\u{a0}') {
            continue;
        } else if started {
            break;
        }
    }
    if digits.is_empty() {
        None
    } else {
        digits.parse::<f64>().ok()
    }
}
