use crate::connection::Database;
use crate::orm::query_configs_builder::{QueryConfigs, QueryType};
use crate::orm::sql_query_builder::parse_and_execute_query;

/// Inserts a successfully applied migration into `migrations_history`, so it is
/// considered "the last version" on the next run.
pub async fn record_migration(db: &Database, version: &str) -> Result<(), sqlx::Error> {
    let config = QueryConfigs::new(
        "migrations_history".to_string(),
        QueryType::INSERT,
        vec![],
        None,
        Some(vec![
            ("version".to_string(), version.to_string()),
            ("description".to_string(), format!("Migration {version}")),
            ("success".to_string(), "1".to_string()),
        ]),
        None,
        None,
    );

    parse_and_execute_query(db, config).await
}
