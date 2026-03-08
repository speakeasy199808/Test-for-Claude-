use std::fs;
use std::path::PathBuf;

use lyralang::effects::{check_with_policy, EffectPolicy};
use lyralang::types::{EffectAtom, EffectSet};
use serde_json::{json, Value};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/effects")
        .join(name)
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../goldens/lyralang/effects")
        .join(name)
}

fn broad_policy() -> EffectPolicy {
    let mut allowed = EffectSet::pure();
    allowed.insert(EffectAtom::Io);
    allowed.insert(EffectAtom::Time);
    allowed.insert(EffectAtom::Entropy);
    allowed.insert(EffectAtom::Proof);
    allowed.insert_linear(EffectAtom::State);
    EffectPolicy::new(allowed)
}

#[test]
fn effectchecker_sample_fixture_matches_golden_summary() {
    let source = fs::read_to_string(fixture_path("effects_sample.lyra")).expect("fixture available");
    let output = check_with_policy(&source, broad_policy());

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let judgment = output.judgment.expect("program judgment available");

    let summary = json!({
        "fixture": "fixtures/lyralang/effects/effects_sample.lyra",
        "program_effects": judgment.program_effects.canonical_entries(),
        "bindings": judgment.bindings.iter().map(|binding| json!({
            "name": binding.name,
            "initializer_effects": binding.initializer_effects.canonical_entries(),
        })).collect::<Vec<_>>(),
        "policy": output.policy.expect("policy recorded").canonical_entries(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("effects_sample.inference.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}

#[test]
fn effectchecker_invalid_fixture_emits_expected_policy_violation() {
    let source = fs::read_to_string(fixture_path("effects_invalid.lyra")).expect("fixture available");
    let mut allowed = EffectSet::pure();
    allowed.insert(EffectAtom::State);
    let output = check_with_policy(&source, EffectPolicy::new(allowed));

    assert!(output.judgment.is_none(), "unexpected successful judgment");
    assert_eq!(output.errors.len(), 1, "expected one diagnostic");

    let summary = json!({
        "fixture": "fixtures/lyralang/effects/effects_invalid.lyra",
        "expected_errors": output.errors.iter().map(|error| json!({
            "kind": error.kind.label(),
            "message": error.message,
            "line": error.span.start.line,
            "column": error.span.start.column,
        })).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("effects_invalid.diagnostics.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}
