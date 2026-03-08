
use lyralang::temporal::check;
use lyralang::testing::{assert_json_golden, read_fixture};
use serde_json::json;

#[test]
fn temporal_sample_fixture_matches_golden_summary() -> Result<(), Box<dyn std::error::Error>> {
    let source = read_fixture("temporal", "temporal_sample.lyra")?;
    let output = check(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let program = output.judgment.expect("temporal judgment available");

    let summary = json!({
        "fixture": "fixtures/lyralang/temporal/temporal_sample.lyra",
        "module": program.module,
        "program_type": program.program_type.canonical_name(),
        "binding_count": program.bindings.len(),
        "operators": program.formulas.iter().map(|formula| formula.operator.clone()).collect::<Vec<_>>(),
        "normalized_formulas": program.formulas.iter().map(|formula| formula.normalized_formula.clone()).collect::<Vec<_>>(),
    });

    assert_json_golden(&summary, "temporal", "temporal_sample.program.json")
}

#[test]
fn temporal_invalid_fixture_emits_expected_diagnostic() -> Result<(), Box<dyn std::error::Error>> {
    let source = read_fixture("temporal", "temporal_invalid.lyra")?;
    let output = check(&source);

    assert!(output.judgment.is_none(), "unexpected temporal judgment");
    assert_eq!(output.errors.len(), 1, "expected one diagnostic");

    let summary = json!({
        "fixture": "fixtures/lyralang/temporal/temporal_invalid.lyra",
        "expected_errors": output.errors.iter().map(|error| json!({
            "kind": error.kind.label(),
            "message": error.message,
            "line": error.span.start.line,
            "column": error.span.start.column,
        })).collect::<Vec<_>>(),
    });

    assert_json_golden(&summary, "temporal", "temporal_invalid.errors.json")
}
