# P1-019 — Seed Code Generator

## Mission
Deterministic Stage 0 register-VM IR generation with canonical format version and fixture-backed validation.

## Scope
- canonical Stage 0 IR format tag
- deterministic register and label allocation
- lowering for current executable AST forms
- fixture-backed success and failure validation
- normative codegen documentation

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/CODEGEN.md`
- `lyralang/src/codegen/`
- `lyralang/tests/seed_code_generator_integration.rs`
- `fixtures/lyralang/codegen/*`
- `goldens/lyralang/codegen/*`
- task control-plane records and traceability artifacts
