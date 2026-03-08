use serde_json::{json, Value};

use lyralang::errors::check;
use lyralang::testing::read_fixture;

fn golden(group: &str, name: &str) -> Value {
    lyralang::testing::read_json_golden(group, name)
        .expect("golden available")
}

#[test]
fn error_sample_fixture_matches_golden_summary() {
    let source = read_fixture("errors", "error_sample.lyra").expect("fixture available");
    let output = check(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let judgment = output.judgment.expect("error-handling judgment available");

    let summary = json!({
        "fixture": "fixtures/lyralang/errors/error_sample.lyra",
        "program_type": judgment.program_type.canonical_name(),
        "propagation_mode": judgment.propagation_mode,
        "propagated_error_type": judgment.propagated_error_type,
        "panic_free": judgment.panic_free,
        "bindings": judgment.bindings.iter().map(|binding| json!({
            "name": binding.name,
            "uses_try": binding.uses_try,
        })).collect::<Vec<_>>(),
        "stack_trace": judgment.stack_trace.iter().map(|frame| json!({
            "line": frame.line,
            "column": frame.column,
            "snippet": frame.snippet,
        })).collect::<Vec<_>>(),
    });

    assert_eq!(summary, golden("errors", "error_sample.summary.json"));
}

#[test]
fn panic_fixture_emits_expected_diagnostic() {
    let source = read_fixture("errors", "error_invalid_panic.lyra").expect("fixture available");
    let output = check(&source);

    assert!(output.judgment.is_none(), "unexpected successful error judgment");
    assert_eq!(output.errors.len(), 1, "expected one diagnostic");

    let summary = json!({
        "fixture": "fixtures/lyralang/errors/error_invalid_panic.lyra",
        "expected_errors": output.errors.iter().map(|error| json!({
            "kind": error.kind.label(),
            "message": error.message,
            "line": error.span.start.line,
            "column": error.span.start.column,
        })).collect::<Vec<_>>(),
    });

    assert_eq!(summary, golden("errors", "error_invalid_panic.diagnostics.json"));
}
