use crate::connection::Database;
use crate::migration_cli::find_migrations::discover_migrations;
use crate::migration_cli::record_migration::record_migration;
use crate::migration_cli::run_migration_files::run_migration_file;
use crate::models::migration_history::MigrationsHistory;
use crate::orm::fetch::fetch_optional_as;
use crate::orm::query_configs_builder::{OrderBy, QueryConfigs, QueryType, SortDirection};
use shared::get_env_variable;
use std::fs;

const BOOTSTRAP_PREFIX: &str = "000-";

pub async fn run_migration_cli() {
    let db = establish_connection().await;

    let mut last_version = match get_history(&db).await {
        Ok(Some(history)) => Some(history.version),
        Ok(None) => None,
        Err(e) => {
            eprintln!("Error fetching migration history: {}", e);
            None
        }
    };

    if last_version.is_none() {
        if let Err(e) = create_tables(&db).await {
            eprintln!("Failed to create tables: {}", e);
            return;
        }
        last_version = match get_history(&db).await {
            Ok(Some(history)) => Some(history.version),
            _ => None,
        };
    }

    if let Err(e) = run_migrations(&db, last_version).await {
        eprintln!("Migration run failed: {}", e);
    }
}

pub async fn establish_connection() -> Database {
    let database_url = get_env_variable("DATABASE_URL");
    let mut db = Database::new(&database_url).await;

    match db.connect().await {
        Ok(_) => println!("Database connection established successfully."),
        Err(e) => println!("Error establishing database connection: {}", e),
    }
    db
}

pub async fn get_history(db: &Database) -> Result<Option<MigrationsHistory>, sqlx::Error> {
    let config = QueryConfigs::new(
        "migrations_history".to_string(),
        QueryType::SELECT,
        vec![],
        None,
        None,
        Some(OrderBy {
            column: "id".to_string(),
            direction: SortDirection::Desc,
        }),
        Some(1),
    );

    let latest = fetch_optional_as::<MigrationsHistory>(db, config).await?;
    Ok(latest)
}

pub async fn create_tables(db: &Database) -> Result<(), sqlx::Error> {
    run_migration_file(db, "000-01_create_job_offers_table.sql").await?;
    run_migration_file(db, "000-02_create_table_history.sql").await?;
    run_migration_file(db, "000-03_seed_migrations_history.sql").await?;
    Ok(())
}

pub async fn run_migrations(
    db: &Database,
    last_version: Option<String>,
) -> Result<(), sqlx::Error> {
    let migrations = discover_migrations()
        .map_err(|e| sqlx::Error::Protocol(format!("Cannot read migrations directory: {e}")))?;

    for migration in migrations {
        if migration.version.starts_with(BOOTSTRAP_PREFIX) {
            continue;
        }
        if let Some(last) = &last_version {
            if migration.version.as_str() <= last.as_str() {
                continue;
            }
        }

        let script = fs::read_to_string(&migration.path).map_err(|e| {
            sqlx::Error::Protocol(format!("Cannot read {}: {e}", migration.path.display()))
        })?;
        
        if script.trim().is_empty() {
            println!(
                "Migration {} is empty, skipping execution.",
                migration.version
            );
        } else {
            println!("Applying migration {}...", migration.version);
            db.execute_script(&script).await?;
        }

        record_migration(db, &migration.version).await?;
    }

    Ok(())
}

// all first job :
// connection
// recuperation des histories
// users verifications
// creation db
// creation tables

// all about update thanks to migrationsScript
