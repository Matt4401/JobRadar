pub async fn get_html_from_url(url: &str, client: reqwest::Client) -> Result<String, String> {
    client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Erreur requête {url} : {e}"))?
        .text()
        .await
        .map_err(|e| format!("Erreur lecture du corps {url} : {e}"))
}
