use std::collections::HashMap;
use std::sync::LazyLock;

use crate::models::job_offer::JobOffer;
use crate::models::migration_history::MigrationsHistory;
use crate::models::Table;

/// Whitelist of the tables and columns the ORM is allowed to reference.
///
/// Built directly from the model structs (`Table` impls), so any change to a
/// model — a renamed, added or removed field — is reflected here automatically.
/// To expose a new table, register its model in the closure below; the columns
/// are derived from the struct and never listed by hand.
static ALLOWED_SCHEMA: LazyLock<HashMap<String, Vec<String>>> = LazyLock::new(|| {
    let mut schema: HashMap<String, Vec<String>> = HashMap::new();
    register::<JobOffer>(&mut schema);
    register::<MigrationsHistory>(&mut schema);
    schema
});

fn register<T: Table>(schema: &mut HashMap<String, Vec<String>>) {
    schema.insert(T::TABLE.to_string(), T::columns());
}

/// Validates that `table` is a known table of the schema.
pub fn validate_table(table: &str) -> Result<(), String> {
    if ALLOWED_SCHEMA.contains_key(table) {
        Ok(())
    } else {
        Err(format!("Unknown or disallowed table: {table}"))
    }
}

/// Validates that `column` is a known column of `table`.
pub fn validate_column(table: &str, column: &str) -> Result<(), String> {
    let columns = ALLOWED_SCHEMA
        .get(table)
        .ok_or_else(|| format!("Unknown or disallowed table: {table}"))?;

    if columns.iter().any(|c| c == column) {
        Ok(())
    } else {
        Err(format!(
            "Unknown or disallowed column `{column}` for table `{table}`"
        ))
    }
}

/// Defense-in-depth scan of the *structural* SQL string.
pub fn detect_suspicious_patterns(query: &str) -> Result<(), String> {
    let danger_patterns = [
        "--",
        "/*",
        "*/",
        ";--",
        "xp_cmdshell",
        " OR 1=1",
        " OR '1'='1'",
    ];

    let upper = query.to_uppercase();
    for pattern in danger_patterns {
        if upper.contains(&pattern.to_uppercase()) {
            return Err(format!(
                "Suspected SQL injection pattern detected: {}",
                pattern
            ));
        }
    }
    Ok(())
}
