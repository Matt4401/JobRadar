use std::path::{Path, PathBuf};
use shared::workspace_root;

pub fn get_config_file_path() -> PathBuf {
    workspace_root().join("scraper_config.toml")
}

pub fn get_filters_file_path() -> PathBuf {
    workspace_root().join("scraper_filters.toml")
}
