# Traceability — P1-007 Self Reference Primitives

## Acceptance to Implementation Map

- `@name()` grammar surface → `docs/lyralang/GRAMMAR.md`, `lyralang/src/parser/{ast.rs,parser.rs}`
- canonical metadata types → `lyralang/src/types/ty.rs`
- self-reference type assignment → `lyralang/src/checker/infer.rs`, `lyralang/src/builtins.rs`
- dedicated IR lowering → `lyralang/src/codegen/generator.rs`
- normative law → `docs/lyralang/SELF_REFERENCE.md`
- interface contract → `interfaces/specs/lyralang_self_reference_primitives_v1.json`
- fixture-backed validation → `lyralang/tests/seed_self_reference_integration.rs`
