# Acceptance — P1-007 Self Reference Primitives

## Acceptance Criteria

1. The grammar reserves the `@current_program()`, `@current_receipt()`, and `@ledger_state()` surface.
2. The shared type kernel defines canonical metadata descriptor types for all three primitives.
3. The seed type checker assigns deterministic types to each primitive.
4. The seed code generator lowers each primitive into a dedicated canonical self-reference IR instruction.
5. The normative law is recorded in `docs/lyralang/SELF_REFERENCE.md`.
6. Fixture-backed validation proves the parser, type checker, and code generator agree on the sample surface.

## Verification Method
- Review `docs/lyralang/SELF_REFERENCE.md`
- Review `interfaces/specs/lyralang_self_reference_primitives_v1.json`
- Inspect parser/type/codegen updates in `lyralang/`
- Run `lyralang/tests/seed_self_reference_integration.rs`

## Evidence Required
- `docs/lyralang/SELF_REFERENCE.md`
- `interfaces/specs/lyralang_self_reference_primitives_v1.json`
- `fixtures/lyralang/selfref/*`
- `goldens/lyralang/selfref/*`
- `lyralang/tests/seed_self_reference_integration.rs`
- `lyra/tasks/P01/P1-007_self_reference_primitives/artifacts/self-reference-traceability.md`
