use lyralang::lexer::lex;
use lyralang::testing::{assert_json_golden, read_fixture};
use proptest::prelude::*;
use serde_json::json;

#[test]
fn framework_fixture_matches_golden_summary() {
    let source = read_fixture("test_framework", "framework_sample.lyra").expect("fixture available");
    let output = lex(&source);
    let significant = output.significant_tokens();

    let summary = json!({
        "fixture": "fixtures/lyralang/test_framework/framework_sample.lyra",
        "normalized_source": output.normalized_source,
        "significant_token_count": significant.len(),
        "first_token": significant.first().map(|token| token.lexeme.clone()).unwrap_or_default(),
        "last_token": significant.last().map(|token| token.lexeme.clone()).unwrap_or_default(),
    });

    assert_json_golden(&summary, "test_framework", "framework_sample.summary.json")
        .expect("golden comparison succeeds");
}

proptest! {
    #[test]
    fn lexer_is_deterministic_under_generated_source(
        source in "[A-Za-z0-9_ \t\n\r/\\*\\?\\+\\-\\(\\)\\{\\}=,]{0,64}"
    ) {
        let left = lex(&source);
        let right = lex(&source);
        prop_assert_eq!(left, right);
    }
}
