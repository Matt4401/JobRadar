use db::connection::Database;

#[derive(Clone)] // need clone cause axum's State clone the state in every thread
pub struct AppState {
    pub db: Database,
}
