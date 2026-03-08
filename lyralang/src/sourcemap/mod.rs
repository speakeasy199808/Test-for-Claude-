//! Source map generation for LyraLang.
//!
//! This module provides bidirectional mapping between source locations (line,
//! column, byte span) and bytecode instruction offsets, enabling debugger
//! integration and source-level stepping.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::span::SourceSpan;
use crate::parser::ast::{ExpressionKind, Statement};

// ── Error types ───────────────────────────────────────────────────────────────

/// Categories of source-map error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceMapErrorKind {
    /// Source text failed to parse.
    ParseError,
    /// Code generation failed.
    CodegenError,
    /// A mapping between source and bytecode could not be established.
    MappingInconsistency,
}

impl SourceMapErrorKind {
    /// Returns a stable machine-readable label for this error kind.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::CodegenError => "codegen_error",
            Self::MappingInconsistency => "mapping_inconsistency",
        }
    }
}

/// A source-map diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct SourceMapError {
    /// Error category.
    pub kind: SourceMapErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
    /// Source span associated with the error.
    pub span: SourceSpan,
    /// Whether generation recovered and continued past this error.
    pub recovered: bool,
}

impl SourceMapError {
    /// Creates a new source-map diagnostic.
    #[must_use]
    pub fn new(
        kind: SourceMapErrorKind,
        message: impl Into<String>,
        span: SourceSpan,
        recovered: bool,
    ) -> Self {
        Self { kind, message: message.into(), span, recovered }
    }
}

// ── Entry kind ────────────────────────────────────────────────────────────────

/// The semantic category of a source-map entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceMapEntryKind {
    /// A `let` binding statement.
    LetBinding,
    /// A general expression.
    Expression,
    /// A function or builtin call.
    FunctionCall,
    /// A control-flow construct (`if`, `match`).
    ControlFlow,
    /// An integer, boolean, or string literal.
    Literal,
}

// ── Source map entry ──────────────────────────────────────────────────────────

/// A single bidirectional mapping record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMapEntry {
    /// One-based source line number.
    pub source_line: u32,
    /// One-based source column number.
    pub source_column: u32,
    /// Byte offset of the start of the source span.
    pub source_span_start: usize,
    /// Byte offset of the end of the source span (exclusive).
    pub source_span_end: usize,
    /// Index of the corresponding IR instruction.
    pub bytecode_offset: u32,
    /// Canonical textual IR instruction.
    pub ir_instruction: String,
    /// Semantic category of the mapped construct.
    pub entry_kind: SourceMapEntryKind,
}

// ── Debugger hints ────────────────────────────────────────────────────────────

/// Category of a debugger hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebuggerHintKind {
    /// A valid breakpoint site.
    BreakpointSite,
    /// A single-step point.
    StepPoint,
    /// Entry into a lexical scope.
    ScopeEntry,
    /// Exit from a lexical scope.
    ScopeExit,
}

/// A hint for debugger tooling attached to a bytecode offset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerHint {
    /// Kind of debugger hint.
    pub kind: DebuggerHintKind,
    /// Human-readable description of the hint.
    pub description: String,
    /// One-based source line number.
    pub source_line: u32,
    /// Index of the corresponding IR instruction.
    pub bytecode_offset: u32,
}

// ── Source map index ──────────────────────────────────────────────────────────

/// Bidirectional index for fast lookup in either direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMapIndex {
    /// Pairs of (bytecode_offset, entries_index) sorted by bytecode offset.
    pub by_bytecode_offset: Vec<(u32, usize)>,
    /// Pairs of (source_line, entries_index) sorted by source line.
    pub by_source_line: Vec<(u32, usize)>,
}

// ── Source map ────────────────────────────────────────────────────────────────

/// A complete source map for one compiled program.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMap {
    /// Canonical URI used to identify the source.
    pub source_uri: String,
    /// Format version string for this source-map format.
    pub format_version: String,
    /// All mapping entries in source order.
    pub entries: Vec<SourceMapEntry>,
    /// Bidirectional index over `entries`.
    pub index: SourceMapIndex,
    /// Debugger hints in source order.
    pub debugger_hints: Vec<DebuggerHint>,
}

// ── Output bundle ─────────────────────────────────────────────────────────────

/// Result bundle returned by the source-map generator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMapOutput {
    /// Normalized source passed through earlier compiler stages.
    pub normalized_source: String,
    /// Generated source map when generation succeeded.
    pub source_map: Option<SourceMap>,
    /// Diagnostics emitted during generation.
    pub errors: Vec<SourceMapError>,
}

// ── Generator ─────────────────────────────────────────────────────────────────

/// Deterministic source-map generator.
#[derive(Debug, Clone, Default)]
pub struct SourceMapGenerator;

impl SourceMapGenerator {
    /// Creates a new source-map generator.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Parses, lowers, and generates a source map for the given source text.
    #[must_use]
    pub fn generate_source(&self, source: &str) -> SourceMapOutput {
        // --- Parse ---
        let parse_output = crate::parser::parse(source);
        let normalized_source = parse_output.normalized_source.clone();

        if !parse_output.errors.is_empty() {
            let errors = parse_output
                .errors
                .iter()
                .map(|e| {
                    SourceMapError::new(
                        SourceMapErrorKind::ParseError,
                        e.message.clone(),
                        e.span,
                        e.recovered,
                    )
                })
                .collect();
            return SourceMapOutput { normalized_source, source_map: None, errors };
        }

        let Some(program) = parse_output.program else {
            return SourceMapOutput {
                normalized_source,
                source_map: None,
                errors: vec![SourceMapError::new(
                    SourceMapErrorKind::ParseError,
                    "parser completed without a program AST",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        // --- Codegen ---
        let codegen_output = crate::codegen::generate(source);

        if !codegen_output.errors.is_empty() {
            let errors = codegen_output
                .errors
                .iter()
                .map(|e| {
                    SourceMapError::new(
                        SourceMapErrorKind::CodegenError,
                        e.message.clone(),
                        e.span,
                        e.recovered,
                    )
                })
                .collect();
            return SourceMapOutput { normalized_source, source_map: None, errors };
        }

        let Some(codegen_program) = codegen_output.program else {
            return SourceMapOutput {
                normalized_source,
                source_map: None,
                errors: vec![SourceMapError::new(
                    SourceMapErrorKind::CodegenError,
                    "code generator completed without a program",
                    SourceSpan::default(),
                    false,
                )],
            };
        };

        // --- Build entries by walking the AST alongside the instruction stream ---
        let instructions = &codegen_program.instructions;
        let mut entries: Vec<SourceMapEntry> = Vec::new();
        let mut debugger_hints: Vec<DebuggerHint> = Vec::new();
        let mut offset: u32 = 0;

        // Walk program-level statements.
        for statement in &program.statements {
            offset = collect_statement_entries(
                statement,
                instructions,
                offset,
                &mut entries,
                &mut debugger_hints,
            );
        }

        // Walk tail expression.
        if let Some(tail_expr) = &program.tail_expression {
            let instr = instructions.get(offset as usize).cloned().unwrap_or_default();
            let kind = classify_expression_kind(&tail_expr.kind);
            entries.push(SourceMapEntry {
                source_line: tail_expr.span.start.line as u32,
                source_column: tail_expr.span.start.column as u32,
                source_span_start: tail_expr.span.start.offset,
                source_span_end: tail_expr.span.end.offset,
                bytecode_offset: offset,
                ir_instruction: instr,
                entry_kind: kind,
            });
            debugger_hints.push(DebuggerHint {
                kind: DebuggerHintKind::StepPoint,
                description: "tail expression evaluation".to_owned(),
                source_line: tail_expr.span.start.line as u32,
                bytecode_offset: offset,
            });
        }

        // Add block-level scope entry/exit hints.
        if !entries.is_empty() {
            let first_line = entries.first().map(|e| e.source_line).unwrap_or(1);
            let last_line = entries.last().map(|e| e.source_line).unwrap_or(1);
            let first_offset = entries.first().map(|e| e.bytecode_offset).unwrap_or(0);
            let last_offset = entries.last().map(|e| e.bytecode_offset).unwrap_or(0);

            debugger_hints.insert(
                0,
                DebuggerHint {
                    kind: DebuggerHintKind::ScopeEntry,
                    description: "program scope entry".to_owned(),
                    source_line: first_line,
                    bytecode_offset: first_offset,
                },
            );
            debugger_hints.push(DebuggerHint {
                kind: DebuggerHintKind::ScopeExit,
                description: "program scope exit".to_owned(),
                source_line: last_line,
                bytecode_offset: last_offset,
            });
        }

        // --- Build the bidirectional index ---
        let mut by_bytecode_offset: Vec<(u32, usize)> = entries
            .iter()
            .enumerate()
            .map(|(i, e)| (e.bytecode_offset, i))
            .collect();
        by_bytecode_offset.sort_by_key(|&(off, _)| off);

        let mut by_source_line: Vec<(u32, usize)> = entries
            .iter()
            .enumerate()
            .map(|(i, e)| (e.source_line, i))
            .collect();
        by_source_line.sort_by_key(|&(line, _)| line);

        let source_map = SourceMap {
            source_uri: "source://lyra".to_owned(),
            format_version: "lyrasourcemap/v1".to_owned(),
            entries,
            index: SourceMapIndex { by_bytecode_offset, by_source_line },
            debugger_hints,
        };

        SourceMapOutput { normalized_source, source_map: Some(source_map), errors: Vec::new() }
    }
}

/// Parses, lowers, and generates a source map with the default generator.
#[must_use]
pub fn generate(source: &str) -> SourceMapOutput {
    SourceMapGenerator::new().generate_source(source)
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Walks one statement, creating entries and hints, and returns the updated offset.
fn collect_statement_entries(
    statement: &Statement,
    instructions: &[String],
    mut offset: u32,
    entries: &mut Vec<SourceMapEntry>,
    hints: &mut Vec<DebuggerHint>,
) -> u32 {
    match statement {
        Statement::Let(let_stmt) => {
            let instr = instructions.get(offset as usize).cloned().unwrap_or_default();
            let span = let_stmt.span;
            entries.push(SourceMapEntry {
                source_line: span.start.line as u32,
                source_column: span.start.column as u32,
                source_span_start: span.start.offset,
                source_span_end: span.end.offset,
                bytecode_offset: offset,
                ir_instruction: instr,
                entry_kind: SourceMapEntryKind::LetBinding,
            });
            hints.push(DebuggerHint {
                kind: DebuggerHintKind::BreakpointSite,
                description: format!(
                    "let binding: {}",
                    let_stmt.pattern.identifier_text().unwrap_or("_")
                ),
                source_line: span.start.line as u32,
                bytecode_offset: offset,
            });
            offset = offset.saturating_add(1);

            // Recurse into the value expression.
            offset =
                collect_expression_entries(&let_stmt.value, instructions, offset, entries, hints);
        }
        Statement::Expr(expr_stmt) => {
            offset = collect_expression_entries(
                &expr_stmt.expression,
                instructions,
                offset,
                entries,
                hints,
            );
        }
    }
    offset
}

/// Walks one expression, creating entries and hints, and returns the updated offset.
fn collect_expression_entries(
    expression: &crate::parser::ast::Expression,
    instructions: &[String],
    mut offset: u32,
    entries: &mut Vec<SourceMapEntry>,
    hints: &mut Vec<DebuggerHint>,
) -> u32 {
    let instr = instructions.get(offset as usize).cloned().unwrap_or_default();
    let kind = classify_expression_kind(&expression.kind);
    let span = expression.span;

    entries.push(SourceMapEntry {
        source_line: span.start.line as u32,
        source_column: span.start.column as u32,
        source_span_start: span.start.offset,
        source_span_end: span.end.offset,
        bytecode_offset: offset,
        ir_instruction: instr,
        entry_kind: kind,
    });

    hints.push(DebuggerHint {
        kind: DebuggerHintKind::StepPoint,
        description: format!("{kind:?} at line {}", span.start.line),
        source_line: span.start.line as u32,
        bytecode_offset: offset,
    });

    offset = offset.saturating_add(1);

    // Recurse into sub-expressions.
    match &expression.kind {
        ExpressionKind::Group(group) => {
            offset = collect_expression_entries(&group.expression, instructions, offset, entries, hints);
        }
        ExpressionKind::Try(try_expr) => {
            offset = collect_expression_entries(&try_expr.operand, instructions, offset, entries, hints);
        }
        ExpressionKind::Prefix(prefix) => {
            offset = collect_expression_entries(&prefix.operand, instructions, offset, entries, hints);
        }
        ExpressionKind::Binary { left, right, .. } => {
            offset = collect_expression_entries(left, instructions, offset, entries, hints);
            offset = collect_expression_entries(right, instructions, offset, entries, hints);
        }
        ExpressionKind::Call(call) => {
            offset = collect_expression_entries(&call.callee, instructions, offset, entries, hints);
            for arg in &call.arguments {
                offset = collect_expression_entries(arg, instructions, offset, entries, hints);
            }
        }
        ExpressionKind::Block(block) => {
            hints.push(DebuggerHint {
                kind: DebuggerHintKind::ScopeEntry,
                description: "block scope entry".to_owned(),
                source_line: span.start.line as u32,
                bytecode_offset: offset,
            });
            for stmt in &block.statements {
                offset = collect_statement_entries(stmt, instructions, offset, entries, hints);
            }
            if let Some(tail) = &block.tail_expression {
                offset = collect_expression_entries(tail, instructions, offset, entries, hints);
            }
            hints.push(DebuggerHint {
                kind: DebuggerHintKind::ScopeExit,
                description: "block scope exit".to_owned(),
                source_line: span.end.line as u32,
                bytecode_offset: offset.saturating_sub(1),
            });
        }
        ExpressionKind::If(if_expr) => {
            offset =
                collect_expression_entries(&if_expr.condition, instructions, offset, entries, hints);
            offset =
                collect_expression_entries(&if_expr.then_branch, instructions, offset, entries, hints);
            if let Some(else_branch) = &if_expr.else_branch {
                offset =
                    collect_expression_entries(else_branch, instructions, offset, entries, hints);
            }
        }
        ExpressionKind::Match(match_expr) => {
            offset =
                collect_expression_entries(&match_expr.scrutinee, instructions, offset, entries, hints);
            for arm in &match_expr.arms {
                offset =
                    collect_expression_entries(&arm.body, instructions, offset, entries, hints);
            }
        }
        // Leaf nodes: Identifier, Integer, String, Boolean, SelfReference — no sub-expressions.
        ExpressionKind::Identifier(_)
        | ExpressionKind::Integer(_)
        | ExpressionKind::String(_)
        | ExpressionKind::Boolean(_)
        | ExpressionKind::SelfReference(_) => {}
    }

    offset
}

/// Maps an expression kind to a `SourceMapEntryKind`.
fn classify_expression_kind(kind: &ExpressionKind) -> SourceMapEntryKind {
    match kind {
        ExpressionKind::Integer(_)
        | ExpressionKind::String(_)
        | ExpressionKind::Boolean(_) => SourceMapEntryKind::Literal,
        ExpressionKind::Call(_) => SourceMapEntryKind::FunctionCall,
        ExpressionKind::If(_) | ExpressionKind::Match(_) => SourceMapEntryKind::ControlFlow,
        _ => SourceMapEntryKind::Expression,
    }
}
