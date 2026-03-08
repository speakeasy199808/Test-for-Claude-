use std::fs;
use std::path::PathBuf;

use lyralang::parser::{
    parse, BinaryOperator, ExpressionKind, PatternKind, Statement,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/parser")
        .join(name)
}

#[test]
fn parser_sample_fixture_builds_module_block_and_match_ast() {
    let source = fs::read_to_string(fixture_path("parser_sample.lyra")).expect("fixture available");
    let output = parse(&source);

    assert!(output.errors.is_empty(), "unexpected diagnostics: {:?}", output.errors);
    let program = output.program.expect("program available");

    let module_decl = program.module_decl.expect("module declaration present");
    assert_eq!(module_decl.name.text, "calc");
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::Let(statement) => {
            match &statement.pattern.kind {
                PatternKind::Identifier(identifier) => assert_eq!(identifier.text, "threshold"),
                other => panic!("unexpected pattern: {other:?}"),
            }
            assert!(matches!(statement.value.kind, ExpressionKind::Integer(_)));
        }
        other => panic!("unexpected statement: {other:?}"),
    }

    let tail = program.tail_expression.expect("tail expression present");
    let block = match tail.kind {
        ExpressionKind::Block(block) => block,
        other => panic!("unexpected tail expression: {other:?}"),
    };

    assert_eq!(block.statements.len(), 1);
    match &block.statements[0] {
        Statement::Let(statement) => {
            match &statement.pattern.kind {
                PatternKind::Identifier(identifier) => assert_eq!(identifier.text, "input"),
                other => panic!("unexpected block pattern: {other:?}"),
            }
            match &statement.value.kind {
                ExpressionKind::Binary { operator, .. } => {
                    assert_eq!(*operator, BinaryOperator::Add);
                }
                other => panic!("unexpected let initializer: {other:?}"),
            }
        }
        other => panic!("unexpected block statement: {other:?}"),
    }

    let block_tail = block.tail_expression.expect("block tail expression present");
    let match_expression = match block_tail.kind {
        ExpressionKind::Match(match_expression) => match_expression,
        other => panic!("unexpected block tail: {other:?}"),
    };

    assert_eq!(match_expression.arms.len(), 2);
    assert!(matches!(match_expression.arms[0].pattern.kind, PatternKind::Integer(_)));
    assert!(matches!(match_expression.arms[0].body.kind, ExpressionKind::String(_)));
    assert!(matches!(match_expression.arms[1].pattern.kind, PatternKind::Wildcard));
    assert!(matches!(match_expression.arms[1].body.kind, ExpressionKind::Block(_)));
}

#[test]
fn parser_invalid_fixture_emits_expected_syntax_diagnostic() {
    let source = fs::read_to_string(fixture_path("parser_invalid.lyra")).expect("fixture available");
    let output = parse(&source);

    assert!(!output.errors.is_empty(), "expected syntax diagnostics");
    assert_eq!(output.errors[0].kind.label(), "expected_token");
    assert_eq!(output.errors[0].message, "expected `=>` after match pattern");
}
