// Stockage of the URLs of the different job sites

use std::collections::HashMap;
use std::sync::LazyLock;

// HelloWork : la page `/fr-fr/emploi/recherche.html` est interdite par le  robots.txt
// On cible une page SEO "alternance + métier + ville" (chemin sans query string,
// donc autorisée) pour récupérer des offres d'alternance de développeur à Nantes.
// Les pages de détail restent au format `/fr-fr/emplois/{id}.html` (JSON-LD JobPosting).
pub static HELLO_WORK_URL: &str = "https://www.hellowork.com";



static LINKEDIN_URL: &str = "https://www.linkedin.com/jobs/search/?keywords=developpeur";

static FRANCE_TRAVAIL_URL: &str = "https://candidat.francetravail.fr/offres/recherche";

static APEC_URL: &str = "https://www.apec.fr/candidat/recherche-emploi.html/emploi";

static JOB_TEASER_URL: &str = "https://www.jobteaser.com/fr/job-offers";

pub static URL_FOR_SITE: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("hello_work", HELLO_WORK_URL);
    m.insert("linkedin", LINKEDIN_URL);
    m.insert("france_travail", FRANCE_TRAVAIL_URL);
    m.insert("apec", APEC_URL);
    m.insert("job_teaser", JOB_TEASER_URL);

    m
});
