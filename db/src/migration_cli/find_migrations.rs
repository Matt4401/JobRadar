use std::fs;
use std::path::{Path, PathBuf};
use shared::workspace_root_join;

pub(crate) struct Migration {
    pub(crate) version: String,
    pub(crate) path: PathBuf,
}

pub fn migrations_dir() -> PathBuf {
    workspace_root_join("migrations")
}

/// Reads the migrations directory and returns every `.sql` file paired with the
/// version encoded at the start of its name, sorted ascending by version.
pub(crate) fn discover_migrations() -> std::io::Result<Vec<Migration>> {
    let mut migrations = Vec::new();

    for entry in fs::read_dir(migrations_dir())? {
        let path = entry?.path();

        if path.extension().and_then(|e| e.to_str()) != Some("sql") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let version = match stem.split_once('_') {
            Some((v, _)) => v.to_string(),
            None => stem.to_string(),
        };

        migrations.push(Migration { version, path });
    }
    migrations.sort_by(|a, b| a.version.cmp(&b.version));
    Ok(migrations)
}
