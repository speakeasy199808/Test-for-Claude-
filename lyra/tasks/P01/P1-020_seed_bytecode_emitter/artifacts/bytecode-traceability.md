# Traceability — P1-020 Seed Bytecode Emitter

## Acceptance to Implementation Map

- canonical bytecode object law → `docs/lyralang/BYTECODE.md`
- versioned interface contract → `interfaces/specs/lyravm_bytecode_v1.json`
- canonical LyraCodec emission → `lyralang/src/bytecode/{mod.rs,error.rs,emitter.rs}`
- dependency on deterministic Stage 0 IR → `lyralang/src/codegen/mod.rs`, `lyralang/src/bytecode/emitter.rs`
- fixture-backed validation → `fixtures/lyralang/bytecode/*`, `goldens/lyralang/bytecode/*`, `lyralang/tests/seed_bytecode_emitter_integration.rs`
