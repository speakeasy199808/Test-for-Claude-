//! lyralang — Lyra Language Definition and Compiler Toolchain
//!
//! This crate owns the Lyra language definition, compiler pipeline,
//! language services, and self-hosting infrastructure.
//!
//! # Module Map (Phase 1 — LyraLang Stage 0 Foundations)
//! - [`lexer`]     — Unicode identifiers, reserved words, comments, and seed tokenization (P1-001, P1-015)
//! - [`parser`]    — Recursive descent + Pratt parser, AST output, and span-carrying syntax tree (P1-002, P1-016)
//! - [`types`]     — Type system kernel: primitives, product, sum, function, and schemes (P1-003)
//! - [`checker`]   — Hindley-Milner type checker with explicit function effects (P1-017)
//! - [`effects`]   — Effect algebra, policy checking, and seed effect inference (P1-004, P1-018)
//! - [`linear`]    — Linear resource typing and exact-once ownership discharge (P1-005)
//! - [`modal`]     — Epistemic modal typing and evidence-backed promotion tracing (P1-006)
//! - [`errors`]    — Result/Option propagation, panic-free subset enforcement, and stack traces (P1-010)
//! - [`concurrency`] — Structured concurrency, channel surfaces, schedule summaries, and race checks (P1-011)
//! - [`traits`]    — Internal trait/typeclass registry, coherence, orphan prevention, and derive expansion (P1-009)
//! - [`semantics`] — Executable denotational/operational semantics for Stage 0 (P1-008)
//! - [`codegen`]   — Deterministic register-VM IR generation for Stage 0 programs (P1-019)
//! - [`bytecode`]  — LyraVM bytecode encoding from canonical Stage 0 IR (P1-020)
//! - [`stdlib`]    — Minimal standard-library manifest and seed compilation pipeline (P1-021)
//! - [`testing`]   — Shared fixture/golden helpers for unit, property, and integration tests (P1-022)
//! - [`temporal`]  — Linear temporal-logic operator analysis and canonical formula summaries (P1-023)
//! - [`patterns`]  — Pattern matching exhaustiveness checking (P1-012)
//! - [`lifetimes`] — Borrow-checking semantics and lifetime annotations (P1-013)
//! - [`ffi`]       — Foreign function interface specification and safety boundaries (P1-014)
//! - [`probabilistic`] — Symbolic probability distributions and Bayesian updates (P1-024)
//! - [`proof`]     — Proof blocks, obligations, and verifiable artifact extraction (P1-025)
//! - [`macros`]    — Hygienic syntax extension macros (P1-026)
//! - [`meta`]      — Compile-time code execution and quasiquotation (P1-027)
//! - [`typelevel`] — Const generics, type families, and termination checking (P1-028)
//! - [`repl`]      — Interactive read-eval-print loop (P1-029)
//! - [`lsp`]       — Language Server Protocol implementation (P1-030)
//! - [`sourcemap`] — Bidirectional source-to-bytecode mapping and debugger hints (P1-031)

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]

mod builtins;
pub mod bytecode;
pub mod checker;
pub mod codegen;
pub mod concurrency;
pub mod effects;
pub mod errors;
pub mod ffi;
pub mod lexer;
pub mod lifetimes;
pub mod linear;
pub mod lsp;
pub mod macros;
pub mod meta;
pub mod modal;
pub mod parser;
pub mod patterns;
pub mod probabilistic;
pub mod proof;
pub mod repl;
pub mod semantics;
pub mod sourcemap;
pub mod stdlib;
pub mod syntax_ext;
pub mod temporal;
pub mod testing;
pub mod traits;
pub mod typelevel;
pub mod types;
