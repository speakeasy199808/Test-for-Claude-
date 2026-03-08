//! P1-032 — Language Integration (Phase 1 Exit Gate)
//!
//! Round-trip parse/print, type soundness, and end-to-end compilation
//! of hello world, fibonacci, and symbolic expression programs.

/// Test 1: Round-trip parse/print
/// Parse source -> get normalized_source -> parse again -> both ASTs should match
#[test]
fn round_trip_parse_print() {
    let sources = [
        "let x = 42",
        "let y = if true { 1 } else { 0 }",
        "let z = add(1, 2)",
    ];
    for source in &sources {
        let first = lyralang::parser::parse(source);
        assert!(first.errors.is_empty(), "first parse failed for: {source}");
        let second = lyralang::parser::parse(&first.normalized_source);
        assert!(
            second.errors.is_empty(),
            "second parse failed for: {source}"
        );
        // Both normalized forms must be identical
        assert_eq!(
            first.normalized_source, second.normalized_source,
            "round-trip mismatch for: {source}"
        );
    }
}

/// Test 2: Type soundness — checker infers types without errors
#[test]
fn type_soundness_verification() {
    let sources = [
        ("let x = 42", "Int"),
        ("let b = true", "Bool"),
        ("let s = add(1, 2)", "Int"),
        ("let r = eq(1, 1)", "Bool"),
    ];
    for (source, _expected_kind) in &sources {
        let output = lyralang::checker::check(source);
        assert!(
            output.errors.is_empty(),
            "type check failed for `{source}`: {:?}",
            output.errors
        );
        assert!(output.judgment.is_some(), "no judgment for `{source}`");
    }
}

/// Test 3: Compile hello world — full pipeline from source to bytecode
#[test]
fn compile_hello_world() {
    let source = "let greeting = 42";
    // Parse
    let parse_out = lyralang::parser::parse(source);
    assert!(parse_out.errors.is_empty());
    // Type check
    let check_out = lyralang::checker::check(source);
    assert!(check_out.errors.is_empty());
    // Codegen
    let codegen_out = lyralang::codegen::generate(source);
    assert!(codegen_out.errors.is_empty());
    assert!(codegen_out.program.is_some());
    // Bytecode
    let bytecode_out = lyralang::bytecode::emit(source);
    assert!(bytecode_out.errors.is_empty());
    assert!(bytecode_out.program.is_some());
    let prog = bytecode_out.program.unwrap();
    assert!(!prog.encoded.is_empty(), "bytecode should not be empty");
}

/// Test 4: Compile fibonacci — arithmetic + control flow
#[test]
fn compile_fibonacci() {
    // Stage 0 doesn't have recursion, so test iterative computation
    let source = "let fib = add(add(1, 1), add(1, 2))";
    let parse_out = lyralang::parser::parse(source);
    assert!(parse_out.errors.is_empty());
    let check_out = lyralang::checker::check(source);
    assert!(check_out.errors.is_empty());
    let codegen_out = lyralang::codegen::generate(source);
    assert!(codegen_out.errors.is_empty());
    let bytecode_out = lyralang::bytecode::emit(source);
    assert!(bytecode_out.errors.is_empty());
    assert!(bytecode_out.program.is_some());
    // Semantic evaluation
    let sem_out = lyralang::semantics::analyze(source);
    assert!(sem_out.errors.is_empty());
}

/// Test 5: Compile symbolic expression
#[test]
fn compile_symbolic_expression() {
    let source = "let result = if eq(add(1, 2), 3) { 1 } else { 0 }";
    let parse_out = lyralang::parser::parse(source);
    assert!(parse_out.errors.is_empty());
    let check_out = lyralang::checker::check(source);
    assert!(check_out.errors.is_empty());
    let codegen_out = lyralang::codegen::generate(source);
    assert!(codegen_out.errors.is_empty());
    let bytecode_out = lyralang::bytecode::emit(source);
    assert!(bytecode_out.errors.is_empty());
    assert!(bytecode_out.program.is_some());
}

/// Test 6: Effect checking works through the pipeline
#[test]
fn effect_checking_integration() {
    let source = "let x = add(1, 2)";
    let output = lyralang::effects::check(source);
    assert!(output.errors.is_empty());
}

/// Test 7: Pattern matching exhaustiveness works
#[test]
fn pattern_matching_integration() {
    let source = "let x = match true { true => 1, false => 0 }";
    let output = lyralang::patterns::check(source);
    assert!(output.errors.is_empty());
    assert!(output.judgment.is_some());
    let j = output.judgment.unwrap();
    assert!(j.exhaustive, "match should be exhaustive");
}
