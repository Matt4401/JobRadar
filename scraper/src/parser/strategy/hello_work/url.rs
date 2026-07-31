use crate::sites_data::urls::HELLO_WORK_URL;

pub(crate) const BASE_URL: &str = HELLO_WORK_URL;

pub fn get_full_url(path: &str) -> String {
    format!("{BASE_URL}{path}")
}
