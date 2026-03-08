//! Interactive read-eval-print loop engine for LyraLang.
//!
//! This module provides the REPL engine as a library surface. It evaluates
//! expressions by running the full compiler pipeline and maintains session
//! state across multiple evaluations.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Categories of REPL error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplErrorKind {
    /// Source text failed to parse.
    ParseError,
    /// Type checking failed.
    TypeError,
    /// Semantic evaluation failed.
    EvalError,
}

impl ReplErrorKind {
    /// Returns a stable machine-readable label for this error kind.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::TypeError => "type_error",
            Self::EvalError => "eval_error",
        }
    }
}

/// A REPL-level diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct ReplError {
    /// Error category.
    pub kind: ReplErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
}

impl ReplError {
    /// Creates a new REPL diagnostic.
    #[must_use]
    pub fn new(kind: ReplErrorKind, message: impl Into<String>) -> Self {
        Self { kind, message: message.into() }
    }
}

/// A recorded let binding accumulated during a REPL session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplBinding {
    /// The bound identifier name.
    pub name: String,
    /// Canonical type annotation for the binding.
    pub type_info: String,
    /// Human-readable value summary.
    pub value_summary: String,
    /// The evaluation index at which this binding was introduced.
    pub eval_index: usize,
}

/// An entry in the REPL evaluation history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplHistoryEntry {
    /// Sequential evaluation index (0-based).
    pub index: usize,
    /// Original input string.
    pub input: String,
    /// Short summary of the evaluation result.
    pub result_summary: String,
}

/// A snapshot of the session state at the time of an evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplSessionSnapshot {
    /// Total number of evaluations performed so far.
    pub eval_count: usize,
    /// Number of active let bindings in the session.
    pub binding_count: usize,
    /// Canonical type of the most recent successful result, if any.
    pub last_type: Option<String>,
}

/// The result payload of a single REPL evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplResult {
    /// A non-unit evaluated value, rendered as a string.
    Value(String),
    /// The evaluation produced the unit value `()`.
    Unit,
    /// One or more error messages from the compiler pipeline.
    Error(Vec<String>),
    /// Response to a `:type` meta-command.
    TypeQuery(String),
    /// Response to a `:state` meta-command.
    StateQuery(ReplState),
    /// The `:reset` meta-command was executed.
    Reset,
}

/// The bundled output of one REPL evaluation round.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplOutput {
    /// The original input string.
    pub input: String,
    /// The evaluation result.
    pub result: ReplResult,
    /// Canonical type of the result, when available.
    pub type_info: Option<String>,
    /// Effect annotation for the result, when available.
    pub effect_info: Option<String>,
    /// Session snapshot captured after this evaluation.
    pub session_snapshot: ReplSessionSnapshot,
}

/// A snapshot of the accumulated REPL session state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplState {
    /// All let bindings currently in scope.
    pub bindings: Vec<ReplBinding>,
    /// Total number of evaluations performed.
    pub eval_count: usize,
}

/// The kind of a completion suggestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionKind {
    /// A language keyword.
    Keyword,
    /// A built-in function.
    Builtin,
    /// A user-defined binding from the current session.
    Binding,
    /// A type name.
    Type,
}

/// A single tab-completion suggestion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionSuggestion {
    /// The completion text to insert.
    pub text: String,
    /// The semantic category of the suggestion.
    pub kind: CompletionKind,
    /// Optional detail string (e.g., type signature).
    pub detail: Option<String>,
}

/// The live session state maintained by the REPL engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplSession {
    /// Full evaluation history in session order.
    pub history: Vec<ReplHistoryEntry>,
    /// Accumulated let bindings from the session.
    pub bindings: Vec<ReplBinding>,
    /// Total number of evaluations performed.
    pub eval_count: usize,
}

impl Default for ReplSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplSession {
    /// Creates a new, empty REPL session.
    #[must_use]
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            bindings: Vec::new(),
            eval_count: 0,
        }
    }

    /// Returns a snapshot of the current session state.
    #[must_use]
    pub fn snapshot(&self, last_type: Option<String>) -> ReplSessionSnapshot {
        ReplSessionSnapshot {
            eval_count: self.eval_count,
            binding_count: self.bindings.len(),
            last_type,
        }
    }
}

/// The REPL engine. Maintains session state across evaluations.
#[derive(Debug, Clone)]
pub struct Repl {
    /// Internal session state.
    session: ReplSession,
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}

impl Repl {
    /// Creates a new REPL engine with an empty session.
    #[must_use]
    pub fn new() -> Self {
        Self {
            session: ReplSession::new(),
        }
    }

    /// Evaluates one line of input and returns the structured output.
    ///
    /// Handles meta-commands (`:type`, `:state`, `:reset`) as well as
    /// ordinary expressions and let bindings.
    #[must_use]
    pub fn evaluate(&mut self, input: &str) -> ReplOutput {
        let trimmed = input.trim();

        // --- :reset ---
        if trimmed == ":reset" {
            self.reset();
            let snapshot = self.session.snapshot(None);
            return ReplOutput {
                input: input.to_owned(),
                result: ReplResult::Reset,
                type_info: None,
                effect_info: None,
                session_snapshot: snapshot,
            };
        }

        // --- :state ---
        if trimmed == ":state" || trimmed.starts_with(":state ") {
            let state = self.inspect_state();
            let snapshot = self.session.snapshot(None);
            return ReplOutput {
                input: input.to_owned(),
                result: ReplResult::StateQuery(state),
                type_info: None,
                effect_info: None,
                session_snapshot: snapshot,
            };
        }

        // --- :type <expr> ---
        if let Some(expr) = trimmed.strip_prefix(":type ") {
            let type_output = crate::checker::check(expr);
            let type_str = if type_output.errors.is_empty() {
                type_output
                    .judgment
                    .map(|j| format!("{}", j.program_type))
                    .unwrap_or_else(|| "Unknown".to_owned())
            } else {
                type_output
                    .errors
                    .iter()
                    .map(|e| e.message.as_str())
                    .collect::<Vec<_>>()
                    .join("; ")
            };
            let snapshot = self.session.snapshot(None);
            return ReplOutput {
                input: input.to_owned(),
                result: ReplResult::TypeQuery(type_str),
                type_info: None,
                effect_info: None,
                session_snapshot: snapshot,
            };
        }

        // --- ordinary expression or let binding ---
        self.session.eval_count += 1;
        let eval_index = self.session.eval_count;

        // Type-check first to get type annotation.
        let type_output = crate::checker::check(trimmed);
        if !type_output.errors.is_empty() {
            let messages: Vec<String> =
                type_output.errors.iter().map(|e| e.message.clone()).collect();
            let summary = messages.join("; ");
            self.session.history.push(ReplHistoryEntry {
                index: eval_index,
                input: input.to_owned(),
                result_summary: format!("error: {summary}"),
            });
            let snapshot = self.session.snapshot(None);
            return ReplOutput {
                input: input.to_owned(),
                result: ReplResult::Error(messages),
                type_info: None,
                effect_info: None,
                session_snapshot: snapshot,
            };
        }

        let inferred_type = type_output
            .judgment
            .as_ref()
            .map(|j| format!("{}", j.program_type));

        let effect_info = type_output
            .judgment
            .as_ref()
            .map(|j| format!("{}", j.program_effects));

        // Run semantic evaluation for the value.
        let sem_output = crate::semantics::analyze(trimmed);
        if !sem_output.errors.is_empty() {
            let messages: Vec<String> =
                sem_output.errors.iter().map(|e| e.message.clone()).collect();
            let summary = messages.join("; ");
            self.session.history.push(ReplHistoryEntry {
                index: eval_index,
                input: input.to_owned(),
                result_summary: format!("error: {summary}"),
            });
            let snapshot = self.session.snapshot(inferred_type.clone());
            return ReplOutput {
                input: input.to_owned(),
                result: ReplResult::Error(messages),
                type_info: inferred_type,
                effect_info,
                session_snapshot: snapshot,
            };
        }

        let (result, value_summary) = match sem_output.judgment {
            None => {
                let msg = "semantic evaluation produced no judgment".to_owned();
                (ReplResult::Error(vec![msg.clone()]), msg)
            }
            Some(ref judgment) => {
                let rendered = judgment.denotation_rendered.clone();
                // Record any let bindings from this evaluation.
                for binding in &judgment.bindings {
                    let type_str = inferred_type.clone().unwrap_or_else(|| "Unknown".to_owned());
                    // Find binding-specific type if available.
                    let binding_type = type_output
                        .judgment
                        .as_ref()
                        .and_then(|j| {
                            j.bindings.iter().find(|b| b.name == binding.name).map(|b| {
                                format!("{}", b.scheme.body)
                            })
                        })
                        .unwrap_or_else(|| type_str);
                    self.session.bindings.push(ReplBinding {
                        name: binding.name.clone(),
                        type_info: binding_type,
                        value_summary: binding.rendered.clone(),
                        eval_index,
                    });
                }
                let result = if rendered == "()" {
                    ReplResult::Unit
                } else {
                    ReplResult::Value(rendered.clone())
                };
                (result, rendered)
            }
        };

        self.session.history.push(ReplHistoryEntry {
            index: eval_index,
            input: input.to_owned(),
            result_summary: value_summary,
        });

        let snapshot = self.session.snapshot(inferred_type.clone());
        ReplOutput {
            input: input.to_owned(),
            result,
            type_info: inferred_type,
            effect_info,
            session_snapshot: snapshot,
        }
    }

    /// Returns completion suggestions for the given partial input.
    ///
    /// Includes keywords, known builtins, and any session bindings that
    /// start with the partial string.
    #[must_use]
    pub fn get_completions(&self, partial: &str) -> Vec<CompletionSuggestion> {
        const KEYWORDS: &[&str] = &["let", "if", "match", "fn", "true", "false"];
        const BUILTINS: &[&str] = &["add", "eq", "some", "ok", "err", "spawn", "join"];

        let mut suggestions = Vec::new();

        for &kw in KEYWORDS {
            if kw.starts_with(partial) {
                suggestions.push(CompletionSuggestion {
                    text: kw.to_owned(),
                    kind: CompletionKind::Keyword,
                    detail: None,
                });
            }
        }

        for &builtin in BUILTINS {
            if builtin.starts_with(partial) {
                suggestions.push(CompletionSuggestion {
                    text: builtin.to_owned(),
                    kind: CompletionKind::Builtin,
                    detail: Some("builtin function".to_owned()),
                });
            }
        }

        for binding in &self.session.bindings {
            if binding.name.starts_with(partial) {
                suggestions.push(CompletionSuggestion {
                    text: binding.name.clone(),
                    kind: CompletionKind::Binding,
                    detail: Some(binding.type_info.clone()),
                });
            }
        }

        suggestions
    }

    /// Returns the current session state (bindings and evaluation count).
    #[must_use]
    pub fn inspect_state(&self) -> ReplState {
        ReplState {
            bindings: self.session.bindings.clone(),
            eval_count: self.session.eval_count,
        }
    }

    /// Resets the REPL session, clearing all bindings and history.
    pub fn reset(&mut self) {
        self.session = ReplSession::new();
    }
}
