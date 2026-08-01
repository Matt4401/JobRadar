// Lib of the scrapper mod

pub mod config;
pub mod core;
pub mod http_client;
pub mod parser;
pub mod sites_data;

pub mod save_offers;

pub use core::scrape_and_store; // for main
