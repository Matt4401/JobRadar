use crate::connection::Database;
use crate::migration_cli::find_migrations::migrations_dir;
use std::fs;

/// Reads a single migration file (by its name inside the migrations directory)
/// and runs it as a SQL script. Used by the bootstrap functions to apply the `000-*` files.
pub async fn run_migration_file(db: &Database, filename: &str) -> Result<(), sqlx::Error> {
    let path = migrations_dir().join(filename);
    let script = fs::read_to_string(&path)
        .map_err(|e| sqlx::Error::Protocol(format!("Cannot read {}: {e}", path.display())))?;
    db.execute_script(&script).await
}
