# LyraOS — Project Context for AI Agents

## Project Overview

LyraOS is a **deterministic, sovereignty-first operating system substrate** built in Rust. It is part of the NeuroOS initiative and implements a layered architecture where a trusted foundational kernel (`k0`) provides constitutional guarantees — determinism, tamper detection, canonical serialization, and verifiable state transitions — upon which higher symbolic systems (`k1`), a custom programming language (`lyralang`), interaction shells, and integration slices are composed.

The project follows a phased build canon (documented in `LYRA_v4_2_AGENT_EXECUTABLE_BUILD_CANON.md`) with 544 tasks organized across multiple phases. Each task is a formal **work package** with its own control-plane directory containing specs, acceptance criteria, design docs, tests, fixtures, and evidence artifacts.

**License:** MIT OR Apache-2.0
**Rust Edition:** 2021
**Workspace Resolver:** 2

---

## Architecture

### Canonical Ownership Zones

| Crate / Directory | Role | Status |
|---|---|---|
| `k0/` | Trusted deterministic foundation: canonicalization, verification, runtime law, security-critical substrate | **Active** — 12 modules, 331 tests |
| `k1/` | Higher symbolic systems: memory, reasoning, planning, retrieval, coordination, domain engines | Stub (depends on k0) |
| `lyralang/` | Lyra language definition, compiler/toolchain, language services, self-hosting pipeline | Stub (Phase 1) |
| `shells/` | Interaction shells and environment-facing orchestration surfaces (CLI, agent, REPL, API) | Stub |
| `slices/` | Vertical compositions and end-to-end feature slices spanning subsystem roots | Stub |
| `interfaces/specs/` | Shared IRs, schemas, APIs, versioned contracts, boundary definitions | Contains `lyracodec.md` spec |
| `lyra/tasks/` | Task control plane: specs, acceptance, tests, fixtures, scripts, artifacts per task | 13+ task directories under `P00/` |

### Dependency Graph

```
shells ──► k1 ──► k0
slices ──► k1 ──► k0
lyralang (standalone, future k0 dependency)
```

### k0 Module Map (Foundational Substrate)

| Module | Purpose | Task ID |
|---|---|---|
| `genesis` | Initial state, trust roots (threshold policy, ceremony, HSM stubs), constitutional hash | P0-001, P0-002 |
| `self_verify` | Runtime code integrity verification loop (SelfVerifier, VerificationReceipt) | P0-005 |
| `codec` | Canonical LyraCodec encoder/decoder (varints, structs, vectors, maps) | P0-007 |
| `digest` | SHA-3-256 (primary) + BLAKE3 (secondary) hash routing | P0-008 |
| `time` | Monotonic virtual clock — no wall clock, event-driven only | P0-009 |
| `entropy` | Seeded deterministic randomness, hash-chained pool, receipted consumption | P0-010 |
| `verifier` | Double-run determinism checker, constitutional violation detection | P0-011 |
| `drift` | Runtime nondeterminism detection via statistical tests | P0-012 |
| `incident` | Incident taxonomy (determinism, resource, policy, logic, epistemic, evolution) | P0-013 |
| `recovery` | Recovery protocols: rollback, quarantine, halt, escalate (state machine) | P0-014 |
| `errors` | Globally unique error code system (E0001–E9999, categorized) | P0-018 |
| `logging` | Structured deterministic logging (LogEntry, LogSink, CorrelationId) | P0-019 |

---

## Building and Running

### Prerequisites

- **Rust stable toolchain** with `rustfmt` and `clippy` components
- No external CI service dependency — designed for self-hosted runners

### Core Commands

```bash
# Build the entire workspace
cargo build --workspace

# Run all tests across the workspace
cargo test --workspace

# Run only k0 unit tests
cargo test -p k0 --lib

# Check formatting (CI gate)
cargo fmt --all -- --check

# Lint with clippy (CI denies all warnings)
cargo clippy --workspace --all-targets -- -D warnings

# Dependency audit (requires cargo-deny)
cargo install cargo-deny --locked
cargo deny check advisories licenses sources
```

### CI Pipeline

Defined in `.github/workflows/ci.yml`. Triggers on pushes to `main` and `blackboxai/**` branches, and on PRs to `main`. Three jobs:

1. **build-and-test** — fmt check → clippy (deny warnings) → build → test
2. **test-k0** — focused k0 unit tests
3. **deny** — dependency audit via `cargo-deny` (advisory, license, source checks)

---

## Development Conventions

### Constitutional Invariants (MUST be preserved)

1. **Determinism and stable ordering** — identical inputs always produce identical outputs
2. **Offline-first sovereignty** — core truth requires no network
3. **Canonical, versioned representations at boundaries** — all serialization is deterministic
4. **No ambient nondeterminism** — no wall clock, no `std::time`, no unseeded randomness in k0
5. **Explicit subsystem ownership** — code lives in the crate that owns the behavior
6. **Proof-bearing completion** — every task emits verifiable evidence artifacts

### Rust Style

- `#![forbid(unsafe_code)]` on all crates
- `#![deny(missing_docs)]` on all crates — every public item must be documented
- `#![deny(clippy::all)]` on all crates
- Workspace-level dependency management via `[workspace.dependencies]`
- Property-based testing with `proptest` (available as dev-dependency on all crates)
- Error handling via `thiserror` (typed errors) and `anyhow` (ad-hoc errors)
- Structured logging via `tracing`
- Serialization via `serde` + `serde_json`

### Task Control Plane Convention

Every task under `lyra/tasks/PXX/<TASKID>_<slug>/` contains:

```
README.md           — task overview
ACCEPTANCE.md       — acceptance criteria
DESIGN.md           — design decisions
IMPLEMENTATION.md   — implementation notes
task.toml           — structured task metadata
impl/               — implementation stubs (rust/, android/, web/, docs/, ops/)
interfaces/         — boundary definitions
tests/              — unit/ and integration/ test directories
fixtures/           — test fixtures
scripts/            — automation scripts
artifacts/          — evidence artifacts
```

Production code belongs in the **canonical ownership root** (e.g., `k0/src/`), not in the task folder. The task folder is the governance/evidence anchor.

### Work Package Rules

- One task ≠ one file. Tasks expand into full module families.
- Monolith prohibition: broad tasks must be decomposed into coherent modules.
- Completion requires: correct ownership placement + passing acceptance + emitted evidence.
- File existence alone is never sufficient proof of completion.

---

## Current Progress

**Active Phase:** Phase 0 — Foundation Bring-Up

### Completed Tasks (P0) — ALL 23 TASKS COMPLETE ✅

- [x] P0-001 Genesis State
- [x] P0-002 Trust Roots (ThresholdPolicy, TrustRootSet, ceremony stubs, HSM stubs)
- [x] P0-003 Constitutional Math
- [x] P0-004 Repo Architecture
- [x] P0-005 Self Verification Loop (SelfVerifier, VerificationReceipt)
- [x] P0-006 LyraCodec Spec
- [x] P0-007 Canonical Encoder
- [x] P0-008 Digest Algorithms
- [x] P0-009 Virtual Time
- [x] P0-010 Entropy Management
- [x] P0-011 Determinism Verifier
- [x] P0-012 Drift Detection
- [x] P0-013 Incident Taxonomy
- [x] P0-014 Recovery Protocols
- [x] P0-015 CI/CD Pipeline
- [x] P0-016 Versioning Strategy
- [x] P0-017 Benchmarking Harness
- [x] P0-018 Error Code System
- [x] P0-019 Structured Logging
- [x] P0-020 Code Review Protocol
- [x] P0-021 Dependency Audit
- [x] P0-022 Reproducible Builds
- [x] P0-023 Foundation Integration (phase exit gate)

### Phase 0 Summary

Phase 0 is **complete**. See `PHASE0_COMPLETE.md` for the full summary.
- 12 modules in k0, 316 unit tests + 15 integration tests = 331 total
- All constitutional invariants verified (determinism, no unsafe, no wall clock)
- CI pipeline with 3 jobs (build-and-test, test-k0, deny)

### Upcoming Phases

- **Phase 1** — LyraLang Stage 0 Foundations (lexer, parser, type system, effect system, codegen, etc.)
- **Phase 2+** — Higher symbolic systems, platform surfaces, integration slices

---

## Key Reference Documents

| File | Purpose |
|---|---|
| `LYRA_v4_2_AGENT_EXECUTABLE_BUILD_CANON.md` | **Authoritative build canon** — 36K+ lines defining all 544 tasks, archetypes, acceptance criteria, evidence requirements, and phase structure |
| `TODO.md` | Current Phase 0 execution checklist |
| `k0/specs/constitutional_math.md` | Formal specification of system invariants (P0-003) |
| `interfaces/specs/lyracodec.md` | Canonical encoding specification for all Lyra data types (P0-006) |
| `.github/workflows/ci.yml` | CI pipeline definition |

---

## Agent Guidance

When working on this project:

1. **Read the build canon** (`LYRA_v4_2_AGENT_EXECUTABLE_BUILD_CANON.md`) for the specific task record before implementing any task. It contains the mission, scope, archetype, decomposition guidance, acceptance criteria, and evidence requirements.
2. **Place production code in the correct ownership root** — not in the task folder. The task folder is for governance only.
3. **Preserve determinism** — never introduce wall clocks, unseeded randomness, or ambient nondeterministic inputs in k0.
4. **Maintain all lint gates** — `forbid(unsafe_code)`, `deny(missing_docs)`, `deny(clippy::all)`, and `cargo fmt`.
5. **Write tests** — unit tests inline with modules, integration tests in `tests/`, property-based tests with `proptest` where appropriate.
6. **Emit evidence** — every completed task must produce acceptance evidence (test outputs, reports, fixtures, etc.).
7. **Check TODO.md** for current execution order and progress.
8. **Run `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings`** after every change to verify nothing is broken.
