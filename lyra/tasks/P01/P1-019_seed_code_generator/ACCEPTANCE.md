# Acceptance — P1-019 Seed Code Generator

## Acceptance Criteria

1. The seed code generator emits a canonical Stage 0 register-VM IR program bundle.
2. The IR bundle records a stable format version, register count, entry register, result type, and instruction stream.
3. Register allocation is deterministic and stable for repeated lowering of the same source.
4. Self-reference primitives lower into dedicated IR instructions.
5. Fixture-backed verification covers both a successful program and an upstream type-error failure.
6. The normative code-generation law is recorded in `docs/lyralang/CODEGEN.md`.

## Verification Method
- Review `docs/lyralang/CODEGEN.md`
- Inspect `lyralang/src/codegen/`
- Run `lyralang/tests/seed_code_generator_integration.rs`

## Evidence Required
- `docs/lyralang/CODEGEN.md`
- `lyralang/src/codegen/*`
- `fixtures/lyralang/codegen/*`
- `goldens/lyralang/codegen/*`
- `lyralang/tests/seed_code_generator_integration.rs`
- `lyra/tasks/P01/P1-019_seed_code_generator/artifacts/seed-code-generator-traceability.md`
