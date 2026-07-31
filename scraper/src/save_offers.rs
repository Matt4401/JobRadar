use db::connection::Database;
use db::models::job_offer::JobOffer;

pub async fn store_offers(db: &Database, offers: &[JobOffer]) -> Result<usize, String> {
    const QUERY: &str = "INSERT INTO job_offers \
        (title, description, company, location, url, score, created_at, \
         salary, study_level, contract_type, duration, remote, experience_level) \
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) \
        ON DUPLICATE KEY UPDATE \
         title = VALUES(title), description = VALUES(description), \
         company = VALUES(company), location = VALUES(location), \
         salary = VALUES(salary), study_level = VALUES(study_level), \
         contract_type = VALUES(contract_type), duration = VALUES(duration), \
         remote = VALUES(remote), experience_level = VALUES(experience_level)";

    let mut stored = 0usize;
    for offer in offers {
        let binds = vec![
            offer.title.clone(),
            offer.description.clone(),
            offer.company.clone(),
            offer.location.clone(),
            offer.url.clone(),
            offer.score.to_string(),
            offer.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            offer.salary.clone().unwrap_or_default(),
            offer.study_level.clone().unwrap_or_default(),
            offer.contract_type.clone().unwrap_or_default(),
            offer.duration.clone().unwrap_or_default(),
            offer.remote.clone().unwrap_or_default(),
            offer.experience_level.clone().unwrap_or_default(),
        ];
        db.execute_query(QUERY, &binds)
            .await
            .map_err(|e| format!("Error storing offer {}: {}", offer.url, e))?;
        stored += 1;
    }
    Ok(stored)
}
