use std::fs;
use std::path::PathBuf;

use lyralang::codegen::generate;
use serde_json::{json, Value};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/codegen")
        .join(name)
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../goldens/lyralang/codegen")
        .join(name)
}

#[test]
fn codegen_sample_fixture_matches_golden_program() {
    let source = fs::read_to_string(fixture_path("codegen_sample.lyra")).expect("fixture available");
    let output = generate(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let program = output.program.expect("program available");

    let summary = json!({
        "fixture": "fixtures/lyralang/codegen/codegen_sample.lyra",
        "format_version": program.format_version,
        "module": program.module,
        "result_type": program.result_type,
        "entry_register": program.entry_register,
        "register_count": program.register_count,
        "instructions": program.instructions,
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("codegen_sample.program.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}

#[test]
fn codegen_invalid_fixture_emits_expected_diagnostic() {
    let source = fs::read_to_string(fixture_path("codegen_invalid.lyra")).expect("fixture available");
    let output = generate(&source);

    assert!(output.program.is_none(), "unexpected successful program");
    assert_eq!(output.errors.len(), 1, "expected one diagnostic");

    let summary = json!({
        "fixture": "fixtures/lyralang/codegen/codegen_invalid.lyra",
        "expected_errors": output.errors.iter().map(|error| json!({
            "kind": error.kind.label(),
            "message": error.message,
            "line": error.span.start.line,
            "column": error.span.start.column,
        })).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("codegen_invalid.diagnostics.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}
