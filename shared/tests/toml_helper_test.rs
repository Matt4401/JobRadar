// Integration tests for `TomlHelper` read/write, using a temporary file.

use serde::{Deserialize, Serialize};
use shared::toml::toml_interactions::TomlHelper;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Sample {
    name: String,
    count: u32,
    tags: Vec<String>,
}

// Returns a unique temporary file path scoped to this test process.
fn temp_path(label: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("jobradar_toml_{}_{}.toml", label, std::process::id()));
    p
}

#[test]
fn write_then_read_roundtrip() {
    let path = temp_path("roundtrip");
    let helper = TomlHelper::new(&path);

    let data = Sample {
        name: "JobRadar".to_string(),
        count: 3,
        tags: vec!["rust".to_string(), "scraper".to_string()],
    };

    helper.write(&data).expect("write should succeed");
    let read_back: Sample = helper.read().expect("read should succeed");

    assert_eq!(read_back, data);

    let _ = std::fs::remove_file(&path);
}

#[test]
fn read_missing_file_returns_error() {
    let path = temp_path("missing");
    let _ = std::fs::remove_file(&path);

    let helper = TomlHelper::new(&path);
    let result: Result<Sample, String> = helper.read();
    assert!(result.is_err());
}

#[test]
fn read_or_create_default_creates_file_when_absent() {
    let path = temp_path("default");
    let _ = std::fs::remove_file(&path);

    let helper = TomlHelper::new(&path);
    let default = Sample {
        name: "default".to_string(),
        count: 0,
        tags: vec![],
    };

    let value: Sample = helper
        .read_or_create_default(default.clone())
        .expect("should create and return default");
    assert_eq!(value, default);
    assert!(path.exists());

    // A second call reads the file that now exists instead of recreating it.
    let again: Sample = helper
        .read_or_create_default(Sample {
            name: "other".to_string(),
            count: 99,
            tags: vec!["ignored".to_string()],
        })
        .expect("should read existing file");
    assert_eq!(again, default);

    let _ = std::fs::remove_file(&path);
}
