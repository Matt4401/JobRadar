use shared::workspace_root;
use std::path::PathBuf;

pub fn get_config_file_path() -> PathBuf {
    workspace_root().join("scraper_config.toml")
}

pub fn get_filters_file_path() -> PathBuf {
    workspace_root().join("scraper_filters.toml")
}
