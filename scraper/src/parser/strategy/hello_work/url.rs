pub(crate) const BASE_URL: &str = "https://www.hellowork.com";

pub fn get_full_url(path: &str) -> String {
    format!("{BASE_URL}{path}")
}
