use std::fs;
use std::path::PathBuf;

use lyralang::modal::check;
use serde_json::{json, Value};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/modal")
        .join(name)
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../goldens/lyralang/modal")
        .join(name)
}

#[test]
fn modal_sample_fixture_matches_golden_summary() {
    let source = fs::read_to_string(fixture_path("modal_sample.lyra")).expect("fixture available");
    let output = check(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let judgment = output.judgment.expect("program judgment available");

    let summary = json!({
        "fixture": "fixtures/lyralang/modal/modal_sample.lyra",
        "program_type": judgment.program_type.canonical_name(),
        "bindings": judgment.bindings.iter().map(|binding| json!({
            "name": binding.name,
            "modality": binding.modality.as_str(),
            "payload": binding.payload_type.canonical_name(),
        })).collect::<Vec<_>>(),
        "promotions": judgment.promotions.iter().map(|promotion| json!({
            "name": promotion.name,
            "from": promotion.from.as_str(),
            "to": promotion.to.as_str(),
            "evidence": promotion.evidence.as_str(),
        })).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("modal_sample.inference.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}

#[test]
fn modal_invalid_fixture_emits_expected_diagnostic() {
    let source = fs::read_to_string(fixture_path("modal_invalid.lyra")).expect("fixture available");
    let output = check(&source);

    assert!(output.judgment.is_none(), "unexpected successful judgment");
    assert_eq!(output.errors.len(), 1, "expected one diagnostic");

    let summary = json!({
        "fixture": "fixtures/lyralang/modal/modal_invalid.lyra",
        "expected_errors": output.errors.iter().map(|error| json!({
            "kind": error.kind.label(),
            "message": error.message,
            "line": error.span.start.line,
            "column": error.span.start.column,
        })).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("modal_invalid.diagnostics.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}
