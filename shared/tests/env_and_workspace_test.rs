// Integration tests for environment variable helpers and workspace path helpers.

use shared::env::env::{get_env_variable, get_env_variables};
use shared::workspace::workspace::{workspace_root, workspace_root_join};

#[test]
fn get_env_variable_reads_existing_value() {
    // SAFETY: single-threaded per test binary invocation for this unique key.
    std::env::set_var("JOBRADAR_TEST_ENV_PRESENT", "hello");
    assert_eq!(get_env_variable("JOBRADAR_TEST_ENV_PRESENT"), "hello");
}

#[test]
fn get_env_variable_missing_returns_empty_string() {
    std::env::remove_var("JOBRADAR_TEST_ENV_ABSENT");
    assert_eq!(get_env_variable("JOBRADAR_TEST_ENV_ABSENT"), "");
}

#[test]
fn get_env_variables_collects_only_present_keys() {
    std::env::set_var("JOBRADAR_TEST_ENV_A", "a");
    std::env::remove_var("JOBRADAR_TEST_ENV_B");

    let map = get_env_variables(vec!["JOBRADAR_TEST_ENV_A", "JOBRADAR_TEST_ENV_B"]);
    assert_eq!(map.get("JOBRADAR_TEST_ENV_A"), Some(&"a".to_string()));
    assert!(!map.contains_key("JOBRADAR_TEST_ENV_B"));
}

#[test]
fn workspace_root_points_to_repository_root() {
    let root = workspace_root();
    assert!(root.is_dir());
    // The `shared` crate lives directly under the workspace root.
    assert!(root.join("shared").is_dir());
    assert!(root.join("Cargo.toml").is_file());
}

#[test]
fn workspace_root_join_is_relative_to_shared_crate() {
    let manifest = workspace_root_join("Cargo.toml");
    assert!(manifest.is_file());
    assert!(manifest.ends_with("shared/Cargo.toml"));
}
