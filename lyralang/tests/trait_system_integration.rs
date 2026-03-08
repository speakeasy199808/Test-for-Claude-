use std::fs;
use std::path::PathBuf;

use lyralang::checker::check;
use lyralang::traits::{seed_registry, TraitRegistry};
use lyralang::types::Type;
use serde_json::{json, Value};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/traits")
        .join(name)
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../goldens/lyralang/traits")
        .join(name)
}

#[test]
fn trait_sample_fixture_matches_golden_summary() {
    let source = fs::read_to_string(fixture_path("trait_sample.lyra")).expect("fixture available");
    let output = check(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let judgment = output.judgment.expect("program judgment available");
    let registry = seed_registry();
    registry.validate().expect("seed registry validates");

    let eq_resolution = registry
        .resolve_method("eq", &[Type::int(), Type::int()])
        .expect("eq resolution present");
    let print_resolution = registry
        .resolve_method("print", &[Type::bool()])
        .expect("print resolution present");

    let summary = json!({
        "fixture": "fixtures/lyralang/traits/trait_sample.lyra",
        "program_type": judgment.program_type.canonical_name(),
        "bindings": judgment.bindings.iter().map(|binding| json!({
            "name": binding.name,
            "scheme": binding.scheme.canonical_name(),
        })).collect::<Vec<_>>(),
        "registry_version": registry.version,
        "instance_count": registry.instances.len(),
        "eq_resolution": {
            "target": eq_resolution.target,
            "style": eq_resolution.implementation_style.label(),
        },
        "print_resolution": {
            "target": print_resolution.target,
            "style": print_resolution.implementation_style.label(),
        },
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("trait_sample.summary.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}

#[test]
fn invalid_registry_fixture_emits_expected_orphan_diagnostic() {
    let registry: TraitRegistry = serde_json::from_str(
        &fs::read_to_string(fixture_path("trait_invalid_registry.json"))
            .expect("fixture available"),
    )
    .expect("registry fixture parses");

    let error = registry.validate().expect_err("registry must fail orphan rule");

    let summary = json!({
        "fixture": "fixtures/lyralang/traits/trait_invalid_registry.json",
        "expected_error": {
            "kind": error.kind.label(),
            "message": error.message,
        }
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("trait_invalid_registry.diagnostics.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}
