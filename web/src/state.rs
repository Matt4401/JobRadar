use db::connection::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}
