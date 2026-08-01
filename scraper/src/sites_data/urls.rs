// Stockage of the URLs of the different job sites

use std::collections::HashMap;
use std::sync::LazyLock;

// HelloWork : la page `/fr-fr/emploi/recherche.html` est interdite par le  robots.txt
// On cible une page SEO "alternance + métier + ville" (chemin sans query string,
// donc autorisée) pour récupérer des offres d'alternance de développeur à Nantes.
// Les pages de détail restent au format `/fr-fr/emplois/{id}.html` (JSON-LD JobPosting).

// Base du site, utilisée pour reconstruire les URLs absolues des pages de détail.
pub static HELLO_WORK_URL: &str = "https://www.hellowork.com";

// Page de listing SEO effectivement scrapée (contient les liens `/fr-fr/emplois/{id}.html`).
static HELLO_WORK_LISTING_URL: &str =
    "https://www.hellowork.com/fr-fr/alternance/metier_developpeur-ville_nantes-44000.html";

static LINKEDIN_URL: &str = "https://www.linkedin.com/jobs/search/?keywords=developpeur";

static FRANCE_TRAVAIL_URL: &str = "https://candidat.francetravail.fr/offres/recherche";

static APEC_URL: &str = "https://www.apec.fr/candidat/recherche-emploi.html/emploi";

static JOB_TEASER_URL: &str = "https://www.jobteaser.com/fr/job-offers";

pub static URL_FOR_SITE: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("hello_work", HELLO_WORK_LISTING_URL);
    m.insert("linkedin", LINKEDIN_URL);
    m.insert("france_travail", FRANCE_TRAVAIL_URL);
    m.insert("apec", APEC_URL);
    m.insert("job_teaser", JOB_TEASER_URL);

    m
});
