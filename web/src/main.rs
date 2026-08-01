mod handler;
mod routing;
mod state;
mod views;

use crate::routing::router::get_router;
use db::migration_cli::migration_cli::establish_connection;
use state::AppState;

use serde::Deserialize;
use shared::toml::toml_interactions::TomlHelper;
use shared::workspace::workspace::workspace_root;

#[derive(Deserialize)]
pub struct AxumConfig {
    pub port: String,
}

pub fn load_config() -> Result<AxumConfig, String> {
    TomlHelper::new(workspace_root().join("web/src/axum_configs.toml")).read()
}

#[tokio::main]
async fn main() {
    let db = establish_connection().await;
    let state = AppState { db };
    let app = get_router(state).await;
    let config = load_config().expect("Error loading axum_config.toml");
    let addr = config.port;
    let port_copy = addr.clone();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Job Scraper web server running on http://{port_copy}");
    axum::serve(listener, app).await.unwrap();
}
