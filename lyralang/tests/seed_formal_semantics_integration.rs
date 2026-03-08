use std::fs;
use std::path::PathBuf;

use lyralang::semantics::analyze;
use serde_json::{json, Value};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/semantics")
        .join(name)
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../goldens/lyralang/semantics")
        .join(name)
}

#[test]
fn formal_semantics_sample_fixture_matches_golden_summary() {
    let source = fs::read_to_string(fixture_path("semantics_sample.lyra")).expect("fixture available");
    let output = analyze(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let judgment = output.judgment.expect("semantic judgment available");

    let summary = json!({
        "fixture": "fixtures/lyralang/semantics/semantics_sample.lyra",
        "program_type": judgment.program_type,
        "denotation": judgment.denotation_rendered,
        "bindings": judgment.bindings.iter().map(|binding| json!({
            "name": binding.name,
            "rendered": binding.rendered,
        })).collect::<Vec<_>>(),
        "rules": judgment.steps.iter().map(|step| step.rule.clone()).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("semantics_sample.summary.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}

#[test]
fn formal_semantics_invalid_fixture_emits_expected_diagnostic() {
    let source = fs::read_to_string(fixture_path("semantics_invalid.lyra")).expect("fixture available");
    let output = analyze(&source);

    assert!(output.judgment.is_none(), "unexpected successful semantics judgment");
    assert_eq!(output.errors.len(), 1, "expected one diagnostic");

    let summary = json!({
        "fixture": "fixtures/lyralang/semantics/semantics_invalid.lyra",
        "expected_errors": output.errors.iter().map(|error| json!({
            "kind": error.kind.label(),
            "message": error.message,
            "line": error.span.start.line,
            "column": error.span.start.column,
        })).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("semantics_invalid.errors.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}
