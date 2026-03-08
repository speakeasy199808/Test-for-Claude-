use std::fs;
use std::path::PathBuf;

use lyralang::checker::check;
use lyralang::codegen::generate;
use serde_json::{json, Value};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/selfref")
        .join(name)
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../goldens/lyralang/selfref")
        .join(name)
}

#[test]
fn self_reference_fixture_matches_golden_summary() {
    let source = fs::read_to_string(fixture_path("self_reference_sample.lyra")).expect("fixture available");
    let type_output = check(&source);
    assert!(type_output.errors.is_empty(), "unexpected diagnostics: {:?}", type_output.errors);

    let codegen_output = generate(&source);
    assert!(codegen_output.errors.is_empty(), "unexpected codegen diagnostics: {:?}", codegen_output.errors);

    let judgment = type_output.judgment.expect("program judgment available");
    let program = codegen_output.program.expect("codegen program available");

    let summary = json!({
        "fixture": "fixtures/lyralang/selfref/self_reference_sample.lyra",
        "program_type": judgment.program_type.canonical_name(),
        "bindings": judgment.bindings.iter().map(|binding| json!({
            "name": binding.name,
            "scheme": binding.scheme.canonical_name(),
        })).collect::<Vec<_>>(),
        "instructions": program.instructions,
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("self_reference_sample.summary.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}
