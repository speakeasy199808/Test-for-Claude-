# Changelog

All notable changes to the Lyra project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **docs/lyralang/SEMANTICS.md** — Normative Stage 0 denotational and operational semantics with soundness statement (P1-008)
- **lyralang/semantics** — Executable semantics pass with canonical denotation/binding/step judgments (P1-008)
- **docs/lyralang/BYTECODE.md** — Canonical Stage 0 bytecode object and LyraCodec field law (P1-020)
- **interfaces/specs/lyravm_bytecode_v1.json** — Versioned LyraVM bytecode schema contract (P1-020)
- **lyralang/bytecode** — Canonical LyraCodec bytecode emitter over deterministic Stage 0 IR (P1-020)
- **Semantics fixtures/goldens** — Shared success/failure semantic artifacts for the formal semantics pass (P1-008)
- **Bytecode fixtures/goldens** — Shared success/failure canonical bytecode artifacts for the seed emitter (P1-020)
- **docs/lyralang/SELF_REFERENCE.md** — Normative Stage 0 self-reference primitive law and metadata descriptors (P1-007)
- **lyralang/codegen** — Deterministic Stage 0 register-VM IR generator with canonical instruction rendering (P1-019)
- **Self-reference fixtures/goldens** — Shared self-reference validation artifacts covering parser/type/codegen agreement (P1-007)
- **Codegen fixtures/goldens** — Shared success/failure artifacts for the seed code generator (P1-019)
- **docs/lyralang/MODALITY.md** — Normative Stage 0 modal typing law with explicit evidence-backed promotion (P1-006)
- **lyralang/modal** — Deterministic modal analyzer for typed AST promotion tracing and modal binding summaries (P1-006)
- **Modal fixtures/goldens** — Shared success/failure modal artifacts for the seed modal checker (P1-006)
- **docs/lyralang/LINEARITY.md** — Normative Stage 0 linear ownership law for `File`, `Socket`, and `Capability` (P1-005)
- **lyralang/linear** — Deterministic exact-once ownership checker for Stage 0 linear resources (P1-005)
- **Linear resource fixtures/goldens** — Shared success/failure ownership artifacts for the seed linear checker (P1-005)
- **docs/lyralang/GRAMMAR.md** — Normative Stage 0 lexical structure specification with Unicode identifiers, reserved words, comments, and whitespace normalization (P1-001)
- **lyralang/lexer** — Deterministic seed lexer with Unicode-aware identifiers, comments, recoverable diagnostics, and span tracking (P1-015)
- **Phase 1 fixtures/goldens** — Shared lexer fixtures and golden token/diagnostic artifacts (P1-001, P1-015)
- **k0/genesis** — Initial repository state, trust roots, constitutional hash (P0-001)
- **k0/codec** — Canonical LyraCodec encoder/decoder with varint, struct, vector, map support (P0-007)
- **k0/digest** — SHA-3-256 primary and BLAKE3 secondary hash routing (P0-008)
- **k0/time** — Monotonic virtual clock with causal ordering enforcement (P0-009)
- **k0/entropy** — Seeded deterministic randomness with hash-chained pool (P0-010)
- **k0/verifier** — Double-run determinism verifier (P0-011)
- **k0/drift** — Runtime nondeterminism detection via statistical tests (P0-012)
- **k0/incident** — Incident taxonomy: determinism, resource, policy, logic, epistemic, evolution (P0-013)
- **k0/recovery** — Recovery protocol state machine: rollback, quarantine, halt, escalate (P0-014)
- **CI/CD** — GitHub Actions pipeline: build, test, clippy, fmt, cargo-deny (P0-015)
- **Versioning** — Semantic versioning policy, git tagging, conventional commits (P0-016)
- **Workspace** — Cargo workspace with k0, k1, lyralang, shells, slices crates (P0-004)
- **Specs** — Constitutional math specification (P0-003), LyraCodec specification (P0-006)

## [0.1.0] — Unreleased (Phase 0 In Progress)

Initial development release. Foundation bring-up phase.

## v0.0.13

- complete P1-011 Concurrency Primitives with builtin-backed `spawn` / `join` / `select`, typed `Channel[Int]`, deterministic scheduling summaries, and static no-linear-capture race checks
- add `docs/lyralang/CONCURRENCY.md`, `interfaces/specs/lyralang_concurrency_model_v1.json`, and `lyralang/concurrency`
- complete P1-023 Temporal Logic with builtin-backed `always` / `eventually` / `until` / `since`, explicit `Temporal[T]` kernel type, and deterministic normalized formula judgments
- add `docs/lyralang/TEMPORAL.md`, `interfaces/specs/lyralang_temporal_logic_v1.json`, and `lyralang/temporal`

## v0.0.12

- complete P1-010 Error Handling with canonical `Option` / `Result` / `Error` kernel types, postfix `?` propagation, deterministic error-label composition, and panic-free subset enforcement
- add `docs/lyralang/ERRORS.md` and `interfaces/specs/lyralang_error_model_v1.json`
- add `lyralang/errors` analyzer with stack-trace summaries and panic-style call rejection
- complete P1-022 Seed Test Framework with shared fixture/golden helpers and property-based lexer determinism coverage under `cargo test`
- add `docs/lyralang/TESTING.md`, `interfaces/specs/lyralang_test_framework_v1.json`, and framework self-validation fixtures/goldens

## v0.0.11

- complete P1-009 Trait System with internal registry, coherence/orphan validation, default implementation styles, and deterministic derive manifests
- add Stage 0 overloaded trait-backed builtin call surfaces for `eq` and `print`
- complete P1-021 Seed Stdlib Minimal with Lyra-written source modules compiled through checker → codegen → bytecode
- add trait and stdlib docs, interface contracts, fixtures, goldens, integration tests, and task control planes
