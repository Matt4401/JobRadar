// Lib of the scrapper mod

pub mod sites_data;
pub mod parser;
pub mod http_client;
pub mod config;
pub mod core;

pub mod save_offers;

pub use core::scrape_and_store; // for main
