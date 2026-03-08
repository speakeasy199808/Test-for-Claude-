# Acceptance — P1-003 Type System Kernel

## Acceptance Criteria

1. The kernel defines primitive types `Unit`, `Bool`, `Int`, `Nat`, and `Rational`.
2. The kernel defines canonical product, sum, and function type representations.
3. Function types carry an explicit effect slot for later extension.
4. Hindley-Milner type variables and polymorphic schemes are represented deterministically.
5. Canonical textual forms for types and schemes are stable.
6. The normative law is recorded in `docs/lyralang/TYPES.md`.
7. The seed type checker can consume the kernel without inventing private type representations.

## Verification Method
- Review `docs/lyralang/TYPES.md`
- Inspect `lyralang/src/types/`
- Confirm P1-017 consumes these types directly

## Evidence Required
- `docs/lyralang/TYPES.md`
- `lyralang/src/types/*`
- `lyra/tasks/P01/P1-003_type_system_kernel/artifacts/type-system-traceability.md`
