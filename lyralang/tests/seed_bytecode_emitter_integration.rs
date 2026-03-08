use std::fs;
use std::path::PathBuf;

use k0::codec::decode;
use k0::codec::Value;
use lyralang::bytecode::emit;
use serde_json::{json, Value as JsonValue};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/bytecode")
        .join(name)
}

fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../goldens/lyralang/bytecode")
        .join(name)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn bytecode_sample_fixture_matches_golden_summary() {
    let source = fs::read_to_string(fixture_path("bytecode_sample.lyra")).expect("fixture available");
    let output = emit(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let program = output.program.expect("bytecode program available");

    let decoded = decode(&program.encoded).expect("encoded bytecode decodes");
    let decoded_field_count = match decoded {
        Value::Struct { fields, .. } => fields.len(),
        other => panic!("unexpected decoded top-level value: {other:?}"),
    };

    let summary = json!({
        "fixture": "fixtures/lyralang/bytecode/bytecode_sample.lyra",
        "format_version": program.format_version,
        "ir_format_version": program.ir_format_version,
        "result_type": program.result_type,
        "register_count": program.register_count,
        "entry_register": program.entry_register,
        "instruction_count": program.instruction_count,
        "opcodes": program.instructions.iter().map(|instruction| instruction.opcode.clone()).collect::<Vec<_>>(),
        "top_level_field_count": decoded_field_count,
        "encoded_hex": hex(&program.encoded),
    });

    let golden: JsonValue = serde_json::from_str(
        &fs::read_to_string(golden_path("bytecode_sample.program.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}

#[test]
fn bytecode_invalid_fixture_emits_expected_diagnostic() {
    let source = fs::read_to_string(fixture_path("bytecode_invalid.lyra")).expect("fixture available");
    let output = emit(&source);

    assert!(output.program.is_none(), "unexpected successful bytecode program");
    assert_eq!(output.errors.len(), 1, "expected one diagnostic");

    let summary = json!({
        "fixture": "fixtures/lyralang/bytecode/bytecode_invalid.lyra",
        "expected_errors": output.errors.iter().map(|error| json!({
            "kind": error.kind.label(),
            "message": error.message,
            "line": error.span.start.line,
            "column": error.span.start.column,
        })).collect::<Vec<_>>(),
    });

    let golden: JsonValue = serde_json::from_str(
        &fs::read_to_string(golden_path("bytecode_invalid.errors.json"))
            .expect("golden available"),
    )
    .expect("golden json parses");

    assert_eq!(summary, golden);
}
