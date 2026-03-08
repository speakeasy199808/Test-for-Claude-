use std::fs;
use std::path::PathBuf;

use lyralang::checker::check;
use serde_json::{json, Value};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/typechecker")
        .join(name)
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../goldens/lyralang/typechecker")
        .join(name)
}

#[test]
fn typechecker_sample_fixture_matches_golden_summary() {
    let source = fs::read_to_string(fixture_path("typechecker_sample.lyra")).expect("fixture available");
    let output = check(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let judgment = output.judgment.expect("program judgment available");

    let summary = json!({
        "fixture": "fixtures/lyralang/typechecker/typechecker_sample.lyra",
        "program_type": judgment.program_type.canonical_name(),
        "program_effects": judgment.program_effects.atoms.iter().map(|atom| atom.as_str()).collect::<Vec<_>>(),
        "bindings": judgment.bindings.iter().map(|binding| json!({
            "name": binding.name,
            "scheme": binding.scheme.canonical_name(),
        })).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("typechecker_sample.inference.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}

#[test]
fn typechecker_invalid_fixture_emits_expected_diagnostic() {
    let source = fs::read_to_string(fixture_path("typechecker_invalid.lyra")).expect("fixture available");
    let output = check(&source);

    assert!(output.judgment.is_none(), "unexpected successful judgment");
    assert_eq!(output.errors.len(), 1, "expected one diagnostic");

    let summary = json!({
        "fixture": "fixtures/lyralang/typechecker/typechecker_invalid.lyra",
        "expected_errors": output.errors.iter().map(|error| json!({
            "kind": error.kind.label(),
            "message": error.message,
            "line": error.span.start.line,
            "column": error.span.start.column,
        })).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("typechecker_invalid.diagnostics.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}
