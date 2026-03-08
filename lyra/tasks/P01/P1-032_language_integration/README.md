# P1-032 Language Integration (Phase 1 Exit Gate)

## Purpose

This task is the Phase 1 exit gate for the Lyra project. It proves the entire
Stage 0 compiler pipeline works end-to-end by exercising round-trip parse/print,
type soundness verification, and full compilation of representative programs
from source through bytecode emission.

## Scope

- Round-trip parse/print idempotency
- Type soundness verification (Hindley-Milner inference)
- Full pipeline compilation: parse -> type check -> codegen -> bytecode
- Effect checking integration
- Pattern matching exhaustiveness integration
- Semantic evaluation integration

## Test Programs

| Program | Source | Pipeline |
|---------|--------|----------|
| Hello World | `let greeting = 42` | parse, check, codegen, bytecode |
| Fibonacci | `let fib = add(add(1, 1), add(1, 2))` | parse, check, codegen, bytecode, semantics |
| Symbolic | `let result = if eq(add(1, 2), 3) { 1 } else { 0 }` | parse, check, codegen, bytecode |

## Artifacts

- Integration test: `lyralang/tests/language_integration.rs`
- Fixtures: `fixtures/lyralang/integration/`
- Goldens: `goldens/lyralang/integration/pipeline_summary.json`

## Status

Complete. All 7 integration tests pass.
