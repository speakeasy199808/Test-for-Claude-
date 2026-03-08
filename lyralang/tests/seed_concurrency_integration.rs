
use lyralang::concurrency::check;
use lyralang::testing::{assert_json_golden, read_fixture};
use serde_json::json;

#[test]
fn concurrency_sample_fixture_matches_golden_summary() -> Result<(), Box<dyn std::error::Error>> {
    let source = read_fixture("concurrency", "concurrency_sample.lyra")?;
    let output = check(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let program = output.judgment.expect("concurrency judgment available");

    let summary = json!({
        "fixture": "fixtures/lyralang/concurrency/concurrency_sample.lyra",
        "module": program.module,
        "program_type": program.program_type.canonical_name(),
        "scheduling_policy": program.scheduling_policy,
        "spawn_count": program.spawns.len(),
        "spawn_expressions": program.spawns.iter().map(|spawn| spawn.expression.clone()).collect::<Vec<_>>(),
        "join_count": program.joins.len(),
        "select_count": program.selects.len(),
        "channel_operations": program.channel_operations.iter().map(|operation| operation.operation.clone()).collect::<Vec<_>>(),
        "race_free": program.race_free,
    });

    assert_json_golden(&summary, "concurrency", "concurrency_sample.program.json")
}

#[test]
fn concurrency_invalid_fixture_emits_expected_diagnostic() -> Result<(), Box<dyn std::error::Error>> {
    let source = read_fixture("concurrency", "concurrency_invalid.lyra")?;
    let output = check(&source);

    assert!(output.judgment.is_none(), "unexpected concurrency judgment");
    assert_eq!(output.errors.len(), 1, "expected one diagnostic");

    let summary = json!({
        "fixture": "fixtures/lyralang/concurrency/concurrency_invalid.lyra",
        "expected_errors": output.errors.iter().map(|error| json!({
            "kind": error.kind.label(),
            "message": error.message,
            "line": error.span.start.line,
            "column": error.span.start.column,
        })).collect::<Vec<_>>(),
    });

    assert_json_golden(&summary, "concurrency", "concurrency_invalid.errors.json")
}
