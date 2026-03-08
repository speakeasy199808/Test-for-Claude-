# Acceptance — P1-006 Modal Types

## Acceptance Criteria

1. The Stage 0 type kernel defines canonical modalities `Fact`, `Hypothesis`, `Unknown`, `Necessary`, and `Possible`.
2. The type kernel defines canonical evidence token types for modal promotion.
3. Modal promotion is explicit and evidence-backed; no implicit promotion is permitted.
4. The shared builtin environment exposes deterministic introduction, promotion, and elimination contracts.
5. A seed modal checker records modal bindings and witnessed promotions.
6. Fixture-backed verification covers both a successful promotion chain and an evidence mismatch failure.
7. The normative law is recorded in `docs/lyralang/MODALITY.md`.

## Verification Method
- Review `docs/lyralang/MODALITY.md`
- Inspect `lyralang/src/types/ty.rs`
- Inspect `lyralang/src/builtins.rs`
- Inspect `lyralang/src/modal/`
- Run `lyralang/tests/seed_modal_checker_integration.rs`

## Evidence Required
- `docs/lyralang/MODALITY.md`
- `lyralang/src/types/ty.rs`
- `lyralang/src/builtins.rs`
- `lyralang/src/modal/`
- `fixtures/lyralang/modal/*`
- `goldens/lyralang/modal/*`
- `lyra/tasks/P01/P1-006_modal_types/artifacts/modal-types-traceability.md`
