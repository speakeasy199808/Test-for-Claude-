# Acceptance — P1-026 Syntax Extension

## Acceptance Criteria

1. Stage 0 exposes `syntax_define` and `syntax_expand` surface forms recognized by the checker.
2. `syntax_define(name, template)` records a `MacroDefinition` with inferred `template_arity`.
3. `syntax_expand(name, args...)` records a `MacroExpansion` with hygienic bindings (gensym naming).
4. Expansion of an undefined macro produces an `UndefinedMacro` error and sets `hygienic = false`.
5. Arity mismatch between definition and expansion produces an `ArityMismatch` error.
6. The top-level `hygienic` flag is `true` iff no `HygieneViolation` error occurred.
7. Fixtures and goldens cover both success and error behavior.

## Verification Method
- Review `docs/lyralang/MACROS.md` and `interfaces/specs/lyralang_syntax_extension_v1.json`.
- Inspect `lyralang/src/macros/mod.rs` implementation.
- Run checker over `fixtures/lyralang/macros/macros_valid.lyra` and compare to golden.
- Run checker over `fixtures/lyralang/macros/macros_undefined.lyra` and compare to golden.

## Evidence Required
- `docs/lyralang/MACROS.md`
- `interfaces/specs/lyralang_syntax_extension_v1.json`
- `lyralang/src/macros/mod.rs`
- `fixtures/lyralang/macros/macros_valid.lyra`
- `fixtures/lyralang/macros/macros_undefined.lyra`
- `goldens/lyralang/macros/macros_valid.json`
- `goldens/lyralang/macros/macros_undefined.json`
