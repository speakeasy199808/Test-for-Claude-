use std::fs;
use std::path::PathBuf;

use lyralang::stdlib::compile_minimal_stdlib;
use serde_json::{json, Value};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/stdlib/modules")
        .join(name)
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../goldens/lyralang/stdlib")
        .join(name)
}

#[test]
fn minimal_stdlib_manifest_and_compilation_match_golden_summary() {
    let output = compile_minimal_stdlib();

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    assert_eq!(output.manifest.len(), 4, "expected four seed stdlib modules");

    for module in &output.manifest {
        let fixture_name = module
            .source_path
            .rsplit('/')
            .next()
            .expect("fixture filename available");
        let fixture_source = fs::read_to_string(fixture_path(fixture_name)).expect("fixture source available");
        assert_eq!(module.source, fixture_source, "embedded stdlib source drifted from fixture copy");
    }

    let summary = json!({
        "manifest_version": output.manifest_version,
        "compiled_modules": output.compiled_modules.iter().map(|module| json!({
            "name": module.name,
            "category": module.category,
            "source_path": module.source_path,
            "program_type": module.program_type,
            "codegen_format_version": module.codegen_format_version,
            "bytecode_format_version": module.bytecode_format_version,
        })).collect::<Vec<_>>(),
    });

    let golden: Value = serde_json::from_str(
        &fs::read_to_string(golden_path("minimal_stdlib.summary.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}
