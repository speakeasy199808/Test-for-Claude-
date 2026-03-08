//! Deterministic test-framework helpers for LyraLang Stage 0.
//!
//! This module centralizes fixture/golden lookup and stable JSON comparison so
//! the integration suite can remain under `cargo test` while sharing canonical
//! helper behavior.

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

/// Returns the workspace root path from the `lyralang` crate.
#[must_use]
pub fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root available")
        .to_path_buf()
}

/// Returns the canonical fixture path for a suite-local file.
#[must_use]
pub fn fixture_path(group: &str, name: &str) -> PathBuf {
    workspace_root().join("fixtures").join("lyralang").join(group).join(name)
}

/// Returns the canonical golden path for a suite-local file.
#[must_use]
pub fn golden_path(group: &str, name: &str) -> PathBuf {
    workspace_root().join("goldens").join("lyralang").join(group).join(name)
}

/// Reads a fixture file to string.
pub fn read_fixture(group: &str, name: &str) -> std::io::Result<String> {
    fs::read_to_string(fixture_path(group, name))
}

/// Parses a JSON golden file.
pub fn read_json_golden(group: &str, name: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let value = serde_json::from_str(&fs::read_to_string(golden_path(group, name))?)?;
    Ok(value)
}

/// Returns a stable pretty JSON rendering.
#[must_use]
pub fn canonical_json(value: &Value) -> String {
    serde_json::to_string_pretty(value).expect("json serialization succeeds")
}

/// Asserts that a JSON value matches its golden file.
pub fn assert_json_golden(
    actual: &Value,
    group: &str,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let expected = read_json_golden(group, name)?;
    if actual != &expected {
        return Err(format!(
            "golden mismatch for {group}/{name}\nexpected:\n{}\nactual:\n{}",
            canonical_json(&expected),
            canonical_json(actual),
        )
        .into());
    }
    Ok(())
}
