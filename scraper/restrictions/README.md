# Analyse de scrapabilité des sites d'offres d'emploi

Tests réalisés le 2026-07-17 avec un client HTTP simple (`curl`, équivalent `reqwest`,
sans exécution de JavaScript) et un User-Agent de navigateur.

Ce dossier contient le `robots.txt` de chaque site testé.

## Observations par site

### HelloWork — `hellowork-robots.txt`
- **Accès** : HTTP 200, HTML rendu côté serveur.
- **Données** : les pages de détail `/fr-fr/emplois/{id}.html` contiennent un JSON-LD
  `JobPosting` complet (titre, société, localisation, salaire, type de contrat, date,
  description).
- **robots.txt** : la page de recherche `/fr-fr/emploi/recherche.html` et toute URL avec
  `?` sont en `Disallow`. Les pages de **détail** des offres restent autorisées → passer
  par le sitemap ou les pages catégories SEO pour lister les offres.

### France Travail (ex-Pôle emploi) — `france_travail-robots.txt`
- **Accès** : HTTP 200 mais SPA React → aucune offre dans le HTML.
- **robots.txt** : bloque surtout les espaces perso, formations, et URLs avec paramètres
  (`motsCles=`, `jsessionid`...). Plusieurs sitemaps exposés.
- **Recommandation** : utiliser l'**API officielle "Offres d'emploi v2"** plutôt que
  scraper le site.

### APEC — `apec-robots.txt`
- **Accès** : HTTP 200 mais SPA → coquille HTML vide (~12 Ko), pas d'offres.
- **robots.txt** : aucune règle `Disallow`, uniquement des sitemaps (dont
  `sitemap_offres_search_engine.xml.gz`).

### LinkedIn — `linkedin-robots.txt`
- **Accès** : HTTP 200, les cartes d'offres sont rendues côté serveur (page "guest").
- **robots.txt** : `User-agent: * → Disallow: /` (tout interdit ; crawl uniquement sur
  liste blanche via demande e-mail). Rate-limiting agressif et CGU restrictives.

### Indeed — `indeed-robots.txt`
- **Accès** : HTTP 403 + challenge JavaScript. Non scrapable sans navigateur headless.

### JobTeaser — `job_teaser-robots.txt`
- **Accès** : HTTP 403 + challenge Cloudflare (« Enable JavaScript »). Non scrapable sans
  exécuter du JS. Le sitemap sur CDN reste ouvert mais les pages sont protégées.

### Autres testés (pas de robots.txt conservé)
- **Monster** : HTTP 403, bloqué.
- **Glassdoor** : HTTP 403, challenge Cloudflare.

## Classement des sites à scraper

| Rang | Site | Trafic | Facilité | Verdict |
|------|------|--------|----------|---------|
| 🥇 1 | **HelloWork** | Élevé | JSON-LD `JobPosting` prêt à parser | **Recommandé** |
| 🥈 2 | **France Travail** | Très élevé | HTML inutilisable, mais API officielle | Via API |
| 🥉 3 | **LinkedIn** | Très élevé | HTML parsable mais `Disallow: /` + rate-limit | À éviter |
| — | APEC | Moyen | SPA, données en JS uniquement | Nécessite headless/API |
| ❌ | Indeed / JobTeaser / Monster / Glassdoor | Élevé | Challenge anti-bot (403) | Non scrapable sans JS |

**Conclusion** : commencer par **HelloWork** (JSON-LD, mapping direct sur `JobOffer`),
puis ajouter **France Travail via son API** pour le volume.
