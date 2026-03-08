# Implementation — P1-019 Seed Code Generator

## Implemented Module Family

- `docs/lyralang/CODEGEN.md` — normative Stage 0 code-generation law
- `lyralang/src/codegen/mod.rs` — public generator surface and result types
- `lyralang/src/codegen/error.rs` — deterministic codegen diagnostics
- `lyralang/src/codegen/generator.rs` — internal lowering engine and canonical instruction rendering
- `lyralang/tests/seed_code_generator_integration.rs` — fixture-backed verification

## Strategy

- depend on successful parse and type-check outputs
- allocate registers and labels in stable traversal order
- render instructions canonically for fixture comparison
- keep byte serialization out of scope until P1-020

## Follow-on Tasks Enabled

- P1-020 seed bytecode emitter
- P1-021 seed stdlib minimal
- P1-022 seed test framework
