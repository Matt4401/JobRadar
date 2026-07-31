use crate::config::get_configs_files::{get_config_file_path, get_filters_file_path};
use crate::core::ScrapperConfig;
use crate::parser::filters::match_filters::ScrapperFilters;
use shared::toml::toml_interactions::TomlHelper;

pub fn load_config() -> Result<ScrapperConfig, String> {
    TomlHelper::new(get_config_file_path()).read()
}

pub fn load_filters() -> ScrapperFilters {
    TomlHelper::new(get_filters_file_path())
        .read()
        .unwrap_or_else(|e| {
            eprintln!("Filters file not found or invalid: {e}. Using default filters.");
            ScrapperFilters::default()
        })
}

pub fn save_filters(filters: &ScrapperFilters) -> Result<(), String> {
    TomlHelper::new(get_filters_file_path()).write(filters)
}
