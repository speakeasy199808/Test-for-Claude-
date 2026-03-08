# Traceability — P1-003 Type System Kernel

## Acceptance to Implementation Map

- Primitive kernel types → `lyralang/src/types/ty.rs::PrimitiveType`
- Product, sum, and function types → `lyralang/src/types/ty.rs::{Type,FunctionType}`
- Hindley-Milner variables and schemes → `lyralang/src/types/ty.rs::{TypeVariableId,TypeScheme}`
- Canonical effect slot on function types → `lyralang/src/types/effect.rs`, `lyralang/src/types/ty.rs::FunctionType`
- Normative kernel law → `docs/lyralang/TYPES.md`
- Seed consumer → `lyralang/src/checker/`
