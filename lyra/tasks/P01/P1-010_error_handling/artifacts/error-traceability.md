# P1-010 Traceability

- `Option` / `Result` / `Error` kernel extensions → `lyralang/src/types/ty.rs`
- postfix `?` parser surface → `lyralang/src/parser/ast.rs`, `lyralang/src/parser/parser.rs`
- propagation inference → `lyralang/src/checker/infer.rs`
- panic-free subset + stack-trace summary → `lyralang/src/errors/analyzer.rs`
- success/failure evidence → `fixtures/lyralang/errors/*`, `goldens/lyralang/errors/*`, `lyralang/tests/error_handling_integration.rs`
