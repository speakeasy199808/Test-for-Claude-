# Traceability — P1-006 Modal Types

## Acceptance to Implementation Map

- Canonical modalities → `lyralang/src/types/ty.rs::{ModalKind,ModalType,Type::Modal}`
- Canonical evidence tokens → `lyralang/src/types/ty.rs::{EvidenceKind,Type::Evidence}`
- Explicit promotion law → `docs/lyralang/MODALITY.md`
- Builtin modal contracts → `lyralang/src/builtins.rs::{ModalBuiltinBehavior,ModalCallableSignature,builtin_modal_environment,builtin_type_environment}`
- Seed modal checker → `lyralang/src/modal/{mod.rs,checker.rs,error.rs}`
- Fixture-backed verification → `lyralang/tests/seed_modal_checker_integration.rs`
