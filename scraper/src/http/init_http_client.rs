pub const USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0 Safari/537.36";

pub fn http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent(crate::http::init_http_client::USER_AGENT)
        .build()
        .map_err(|e| format!("Impossible de construire le client HTTP : {e}"))
}
