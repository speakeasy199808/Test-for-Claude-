use std::fs;
use std::path::PathBuf;

use lyralang::lexer::{lex, Keyword, LexErrorKind, TokenKind};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/lyralang/lexer")
        .join(name)
}

#[test]
fn lexical_sample_fixture_produces_expected_significant_tokens() {
    let source = fs::read_to_string(fixture_path("lexical_sample.lyra")).expect("fixture available");
    let output = lex(&source);
    let significant = output.significant_tokens();

    assert_eq!(significant[0].kind, TokenKind::Keyword(Keyword::Module));
    assert_eq!(significant[1].lexeme, "Δemo");
    assert_eq!(significant[2].kind, TokenKind::Keyword(Keyword::Let));
    assert_eq!(significant[3].lexeme, "café");
    assert_eq!(significant[4].kind, TokenKind::Assign);
    assert_eq!(significant[5].kind, TokenKind::Integer);
    assert_eq!(significant[6].kind, TokenKind::Keyword(Keyword::Let));
    assert_eq!(significant[7].lexeme, "_shadow");
    assert_eq!(significant[10].kind, TokenKind::Keyword(Keyword::Match));
    assert_eq!(significant[12].kind, TokenKind::LBrace);
    assert_eq!(significant[13].kind, TokenKind::Underscore);
    assert_eq!(significant[14].kind, TokenKind::FatArrow);
    assert_eq!(significant[15].kind, TokenKind::String);
    assert_eq!(significant[16].kind, TokenKind::RBrace);
}

#[test]
fn invalid_fixture_emits_recoverable_diagnostics() {
    let source = fs::read_to_string(fixture_path("lexical_invalid.lyra")).expect("fixture available");
    let output = lex(&source);

    assert_eq!(output.errors.len(), 2);
    assert_eq!(output.errors[0].kind, LexErrorKind::InvalidCharacter);
    assert_eq!(output.errors[1].kind, LexErrorKind::UnterminatedBlockComment);
    assert!(output.tokens.iter().any(|token| token.kind == TokenKind::Error));
}
