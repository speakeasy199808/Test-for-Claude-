# P1-029 Acceptance Criteria

## Functional

- [ ] `Repl::evaluate("let x = 42")` returns `ReplResult::Value("42")` with `type_info = Some("Int")`
- [ ] `Repl::evaluate(":type 42")` returns `ReplResult::TypeQuery("Int")`
- [ ] `Repl::evaluate(":state")` returns `ReplResult::StateQuery` with current bindings
- [ ] `Repl::evaluate(":reset")` returns `ReplResult::Reset` and clears session
- [ ] After a let binding, `inspect_state()` lists the new binding
- [ ] `get_completions("l")` includes `CompletionSuggestion { text: "let", kind: Keyword, .. }`
- [ ] `get_completions("ad")` includes `CompletionSuggestion { text: "add", kind: Builtin, .. }`
- [ ] After `let foo = 1`, `get_completions("f")` includes the `foo` binding
- [ ] A type-check error returns `ReplResult::Error` with non-empty messages
- [ ] `session_snapshot.eval_count` increments with each ordinary evaluation

## Non-functional

- [ ] All public items have `///` doc comments
- [ ] All public types derive `Clone, Debug, Serialize, Deserialize`
- [ ] Module is a library surface (no `main()`)
- [ ] Module builds without warnings under `#![deny(missing_docs)]`

## Fixtures match

- [ ] `fixtures/lyralang/repl/repl_session.json` matches golden at `goldens/lyralang/repl/repl_session.json`
- [ ] `fixtures/lyralang/repl/repl_completions.json` is well-formed JSON
