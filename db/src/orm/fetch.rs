use crate::connection::Database;
use crate::orm::query_configs_builder::QueryConfigs;
use crate::orm::security::secure_request::detect_suspicious_patterns;
use crate::orm::security::verify_syntax::validate_syntax;
use crate::orm::sql_query_builder::build_sql_query;
use sqlx::mysql::MySqlRow;
use sqlx::FromRow;

/// Builds, secures and runs a SELECT, mapping every returned row into `T`.
#[allow(dead_code)] // not used for the moment, but il will change soon
pub async fn fetch_all_as<T>(db: &Database, config: QueryConfigs) -> Result<Vec<T>, sqlx::Error>
where
    T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
{
    let (query, binds) = secure_select(&config)?;
    db.fetch_all_as::<T>(&query, &binds).await
}

/// Same as [`fetch_all_as`] but for queries expected to return zero or one row.
pub async fn fetch_optional_as<T>(
    db: &Database,
    config: QueryConfigs,
) -> Result<Option<T>, sqlx::Error>
where
    T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
{
    let (query, binds) = secure_select(&config)?;
    db.fetch_optional_as::<T>(&query, &binds).await
}

/// Builds the parameterized SQL and runs it through the same security checks as
/// the write path (syntax validation + suspicious pattern scan).
fn secure_select(config: &QueryConfigs) -> Result<(String, Vec<String>), sqlx::Error> {
    let (query, binds) = build_sql_query(config).map_err(sqlx::Error::Protocol)?;
    validate_syntax(&query).map_err(sqlx::Error::Protocol)?;
    detect_suspicious_patterns(&query).map_err(sqlx::Error::Protocol)?;
    Ok((query, binds))
}
