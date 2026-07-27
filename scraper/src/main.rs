#[tokio::main]
async fn main() {
    if let Err(e) = scraper::scrape_and_store().await {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
