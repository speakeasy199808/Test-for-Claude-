use std::fs;
use std::path::PathBuf;

use lyralang::linear::check;
use serde_json::{json, Value};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/linear")
        .join(name)
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../goldens/lyralang/linear")
        .join(name)
}

#[test]
fn linear_checker_sample_fixture_matches_golden_summary() {
    let source = fs::read_to_string(fixture_path("linear_sample.lyra")).expect("fixture available");
    let output = check(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let judgment = output.judgment.expect("program judgment available");

    let summary = json!({
        "fixture": "fixtures/lyralang/linear/linear_sample.lyra",
        "bindings": judgment.bindings.iter().map(|binding| json!({
            "name": binding.name,
            "resource": binding.resource.as_str(),
        })).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("linear_sample.inference.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}

#[test]
fn linear_checker_invalid_fixture_emits_expected_diagnostic() {
    let source = fs::read_to_string(fixture_path("linear_invalid.lyra")).expect("fixture available");
    let output = check(&source);

    assert!(output.judgment.is_none(), "unexpected successful judgment");
    assert_eq!(output.errors.len(), 1, "expected one diagnostic");

    let summary = json!({
        "fixture": "fixtures/lyralang/linear/linear_invalid.lyra",
        "expected_errors": output.errors.iter().map(|error| json!({
            "kind": error.kind.label(),
            "message": error.message,
            "line": error.span.start.line,
            "column": error.span.start.column,
        })).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("linear_invalid.diagnostics.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}
