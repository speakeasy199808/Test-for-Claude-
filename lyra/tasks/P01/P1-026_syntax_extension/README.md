# P1-026 — Syntax Extension

## Mission
`syntax!` macros — define new surface syntax in .lyra itself, hygienic macro system.

## Scope
- normative specification and versioned contracts where required
- executable compiler pipeline surface in `lyralang/`
- fixtures, goldens, and integration validation
- task control-plane records and traceability artifacts

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/MACROS.md`
- `interfaces/specs/lyralang_syntax_extension_v1.json`
- `lyralang/src/macros/mod.rs`
- `lyralang/src/syntax_ext/mod.rs`
- `fixtures/lyralang/macros/macros_valid.lyra`
- `fixtures/lyralang/macros/macros_undefined.lyra`
- `goldens/lyralang/macros/macros_valid.json`
- `goldens/lyralang/macros/macros_undefined.json`

## Surface Forms (Stage 0)
- `syntax_define(name, template)` — defines a new macro with a template.
- `syntax_expand(name, args...)` — expands a previously defined macro.

## Hygiene Model
Macro-introduced bindings are renamed `{original}#gensym{N}` where N is a
monotonically increasing counter. An expansion is `hygienic = true` iff no
`HygieneViolation` error was produced.

## Notes
`macros` is a reserved identifier in Rust proc-macro contexts. The module
source lives in `src/macros/mod.rs` (plain module, not a proc-macro crate) and
is exposed as `pub mod macros` from `lib.rs`. An equivalent implementation is
also in `src/syntax_ext/mod.rs` for contexts that require a different name.
