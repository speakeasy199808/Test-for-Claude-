//! Language Server Protocol engine for LyraLang.
//!
//! This module provides the LSP request/response handling logic as a library
//! surface. It does not start a real TCP or stdio server — it provides typed
//! request/response dispatch that can be wired up by any transport layer.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

// ── Error types ──────────────────────────────────────────────────────────────

/// Categories of LSP engine error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LspErrorKind {
    /// Source text failed to parse.
    ParseError,
    /// Type checking failed.
    TypeError,
    /// The server received an invalid or unsupported request.
    InvalidRequest,
    /// The requested document is not open.
    DocumentNotFound,
}

impl LspErrorKind {
    /// Returns a stable machine-readable label for this error kind.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::TypeError => "type_error",
            Self::InvalidRequest => "invalid_request",
            Self::DocumentNotFound => "document_not_found",
        }
    }
}

/// An LSP engine diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct LspError {
    /// Error category.
    pub kind: LspErrorKind,
    /// Human-readable diagnostic text.
    pub message: String,
}

impl LspError {
    /// Creates a new LSP diagnostic.
    #[must_use]
    pub fn new(kind: LspErrorKind, message: impl Into<String>) -> Self {
        Self { kind, message: message.into() }
    }
}

// ── LSP position and range ────────────────────────────────────────────────────

/// A zero-based line/character position in a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LspPosition {
    /// Zero-based line number.
    pub line: u32,
    /// Zero-based UTF-16 character offset.
    pub character: u32,
}

/// A half-open range in a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LspRange {
    /// Inclusive start position.
    pub start: LspPosition,
    /// Exclusive end position.
    pub end: LspPosition,
}

// ── Diagnostic types ──────────────────────────────────────────────────────────

/// Severity level for an LSP diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LspDiagnosticSeverity {
    /// An error diagnostic.
    Error,
    /// A warning diagnostic.
    Warning,
    /// An informational diagnostic.
    Information,
    /// A hint diagnostic.
    Hint,
}

/// A single LSP diagnostic entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LspDiagnostic {
    /// Source range for the diagnostic.
    pub range: LspRange,
    /// Diagnostic severity level.
    pub severity: LspDiagnosticSeverity,
    /// Human-readable diagnostic message.
    pub message: String,
    /// Diagnostic source identifier (always `"lyralang"`).
    pub source: String,
}

// ── Completion types ──────────────────────────────────────────────────────────

/// Semantic kind for an LSP completion item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LspCompletionKind {
    /// A language keyword.
    Keyword,
    /// A function or builtin.
    Function,
    /// A variable or binding.
    Variable,
    /// A type name.
    Type,
    /// A module name.
    Module,
}

/// A single LSP completion item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LspCompletionItem {
    /// Display label shown in the completion list.
    pub label: String,
    /// Semantic kind of this item.
    pub kind: LspCompletionKind,
    /// Short detail string (e.g., type signature).
    pub detail: Option<String>,
    /// Longer markdown documentation string.
    pub documentation: Option<String>,
    /// Text inserted when the completion is accepted.
    pub insert_text: String,
}

// ── Hover types ───────────────────────────────────────────────────────────────

/// The payload of a successful hover response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverResult {
    /// Markdown-formatted hover text.
    pub contents: String,
    /// Optional source range to highlight.
    pub range: Option<LspRange>,
}

// ── Location types ────────────────────────────────────────────────────────────

/// An LSP document location (URI + range).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LspLocation {
    /// Document URI.
    pub uri: String,
    /// Range within the document.
    pub range: LspRange,
}

// ── Capability and parameter types ───────────────────────────────────────────

/// Client capability flags communicated during initialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientCapabilities {
    /// Whether the client supports hover requests.
    pub hover: bool,
    /// Whether the client supports completion requests.
    pub completion: bool,
    /// Whether the client supports go-to-definition requests.
    pub goto_definition: bool,
    /// Whether the client supports push diagnostics.
    pub diagnostics: bool,
}

impl Default for ClientCapabilities {
    fn default() -> Self {
        Self {
            hover: true,
            completion: true,
            goto_definition: true,
            diagnostics: true,
        }
    }
}

/// Parameters for the `initialize` request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitializeParams {
    /// Optional human-readable client name.
    pub client_name: Option<String>,
    /// Client capabilities advertised during handshake.
    pub capabilities: ClientCapabilities,
}

/// Parameters for a hover request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverParams {
    /// Document URI.
    pub uri: String,
    /// Zero-based line of the cursor.
    pub line: u32,
    /// Zero-based character offset of the cursor.
    pub character: u32,
}

/// Parameters for a completion request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionParams {
    /// Document URI.
    pub uri: String,
    /// Zero-based line of the cursor.
    pub line: u32,
    /// Zero-based character offset of the cursor.
    pub character: u32,
    /// Optional trigger character that caused the completion request.
    pub trigger_character: Option<String>,
}

/// Parameters for a go-to-definition request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GotoDefinitionParams {
    /// Document URI.
    pub uri: String,
    /// Zero-based line of the cursor.
    pub line: u32,
    /// Zero-based character offset of the cursor.
    pub character: u32,
}

// ── Request and response enums ────────────────────────────────────────────────

/// An inbound LSP request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LspRequest {
    /// Client-initiated server handshake.
    Initialize(InitializeParams),
    /// A new document was opened.
    DidOpen {
        /// Document URI.
        uri: String,
        /// Full document text.
        content: String,
    },
    /// A document's content changed.
    DidChange {
        /// Document URI.
        uri: String,
        /// Updated full document text.
        content: String,
    },
    /// Request hover information at a position.
    Hover(HoverParams),
    /// Request completion items at a position.
    Completion(CompletionParams),
    /// Request the definition location of a symbol.
    GotoDefinition(GotoDefinitionParams),
    /// Publish diagnostics for a document.
    PublishDiagnostics {
        /// Document URI to diagnose.
        uri: String,
    },
    /// Shut down the server.
    Shutdown,
}

/// An outbound LSP response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LspResponse {
    /// Successful initialization handshake.
    Initialized {
        /// Server implementation name.
        server_name: String,
        /// Server implementation version.
        server_version: String,
    },
    /// Hover response (None when no hover info is available).
    Hover(Option<HoverResult>),
    /// Completion items list.
    Completion(Vec<LspCompletionItem>),
    /// Go-to-definition result (None when no definition found).
    GotoDefinition(Option<LspLocation>),
    /// Diagnostics list for a document.
    Diagnostics(Vec<LspDiagnostic>),
    /// Generic success acknowledgment.
    Ok,
    /// Generic error response.
    Error {
        /// LSP JSON-RPC error code.
        code: i32,
        /// Human-readable error message.
        message: String,
    },
}

// ── LSP server ────────────────────────────────────────────────────────────────

/// The LSP server engine. Handles document state and request dispatch.
#[derive(Debug, Clone)]
pub struct LspServer {
    /// Open document contents keyed by URI.
    documents: BTreeMap<String, String>,
    /// Cached diagnostics per document URI.
    diagnostics: BTreeMap<String, Vec<LspDiagnostic>>,
    /// Whether the server has been initialized.
    initialized: bool,
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LspServer {
    /// Creates a new LSP server in the uninitialized state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            documents: BTreeMap::new(),
            diagnostics: BTreeMap::new(),
            initialized: false,
        }
    }

    /// Dispatches an inbound request and returns the appropriate response.
    pub fn handle_request(&mut self, request: LspRequest) -> LspResponse {
        match request {
            LspRequest::Initialize(_params) => {
                self.initialized = true;
                LspResponse::Initialized {
                    server_name: "lyralang-lsp".to_owned(),
                    server_version: "0.1.0".to_owned(),
                }
            }

            LspRequest::DidOpen { uri, content } | LspRequest::DidChange { uri, content } => {
                self.update_document(&uri, &content);
                LspResponse::Ok
            }

            LspRequest::Hover(params) => {
                let hover = self.compute_hover(&params);
                LspResponse::Hover(hover)
            }

            LspRequest::Completion(params) => {
                let items = self.compute_completions(&params);
                LspResponse::Completion(items)
            }

            LspRequest::GotoDefinition(params) => {
                let location = self.compute_goto_definition(&params);
                LspResponse::GotoDefinition(location)
            }

            LspRequest::PublishDiagnostics { uri } => {
                let diagnostics = self.get_diagnostics(&uri);
                LspResponse::Diagnostics(diagnostics)
            }

            LspRequest::Shutdown => LspResponse::Ok,
        }
    }

    /// Stores updated document content and refreshes cached diagnostics.
    pub fn update_document(&mut self, uri: &str, content: &str) {
        self.documents.insert(uri.to_owned(), content.to_owned());
        let diags = Self::run_diagnostics(content);
        self.diagnostics.insert(uri.to_owned(), diags);
    }

    /// Returns cached diagnostics for `uri`, computing them on demand if needed.
    #[must_use]
    pub fn get_diagnostics(&self, uri: &str) -> Vec<LspDiagnostic> {
        if let Some(cached) = self.diagnostics.get(uri) {
            return cached.clone();
        }
        // If the document is open but diagnostics haven't been cached yet,
        // compute them now without mutating self (callers expect &self).
        if let Some(content) = self.documents.get(uri) {
            return Self::run_diagnostics(content);
        }
        Vec::new()
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    /// Runs the type-checker and converts errors to LSP diagnostics.
    fn run_diagnostics(content: &str) -> Vec<LspDiagnostic> {
        let type_output = crate::checker::check(content);
        type_output
            .errors
            .iter()
            .map(|error| {
                let start_line = error.span.start.line.saturating_sub(1) as u32;
                let start_char = error.span.start.column.saturating_sub(1) as u32;
                let end_line = error.span.end.line.saturating_sub(1) as u32;
                let end_char = error.span.end.column.saturating_sub(1) as u32;
                LspDiagnostic {
                    range: LspRange {
                        start: LspPosition { line: start_line, character: start_char },
                        end: LspPosition { line: end_line, character: end_char },
                    },
                    severity: LspDiagnosticSeverity::Error,
                    message: error.message.clone(),
                    source: "lyralang".to_owned(),
                }
            })
            .collect()
    }

    /// Computes a hover response for a given position.
    fn compute_hover(&self, params: &HoverParams) -> Option<HoverResult> {
        let content = self.documents.get(&params.uri)?;

        // Find the token at the requested position by walking source lines.
        let token = find_token_at(content, params.line, params.character)?;

        // Run the type checker on the token text to get type info.
        let type_output = crate::checker::check(&token);
        let type_str = if type_output.errors.is_empty() {
            type_output
                .judgment
                .map(|j| j.program_type.canonical_name())
                .unwrap_or_else(|| "Unknown".to_owned())
        } else {
            return Some(HoverResult {
                contents: format!("```lyra\n{token}\n```"),
                range: None,
            });
        };

        Some(HoverResult {
            contents: format!("```lyra\n{token} : {type_str}\n```"),
            range: None,
        })
    }

    /// Returns completion items for a position (keywords + builtins).
    fn compute_completions(&self, _params: &CompletionParams) -> Vec<LspCompletionItem> {
        const KEYWORDS: &[(&str, &str)] = &[
            ("let", "Introduce a let binding"),
            ("if", "Conditional expression"),
            ("match", "Pattern match expression"),
            ("fn", "Function definition"),
            ("true", "Boolean true literal"),
            ("false", "Boolean false literal"),
            ("module", "Module declaration"),
        ];

        const BUILTINS: &[(&str, &str)] = &[
            ("add", "add(a, b) — arithmetic addition"),
            ("eq", "eq(a, b) — equality comparison"),
            ("some", "some(x) — wrap in Option::Some"),
            ("ok", "ok(x) — wrap in Result::Ok"),
            ("err", "err(e) — wrap in Result::Err"),
            ("spawn", "spawn(f) — spawn a concurrent task"),
            ("join", "join(handle) — join a spawned task"),
        ];

        let mut items: Vec<LspCompletionItem> = Vec::new();

        for &(kw, doc) in KEYWORDS {
            items.push(LspCompletionItem {
                label: kw.to_owned(),
                kind: LspCompletionKind::Keyword,
                detail: None,
                documentation: Some(doc.to_owned()),
                insert_text: kw.to_owned(),
            });
        }

        for &(name, doc) in BUILTINS {
            items.push(LspCompletionItem {
                label: name.to_owned(),
                kind: LspCompletionKind::Function,
                detail: Some("builtin".to_owned()),
                documentation: Some(doc.to_owned()),
                insert_text: name.to_owned(),
            });
        }

        items
    }

    /// Finds a let-binding definition site at the given cursor position.
    fn compute_goto_definition(&self, params: &GotoDefinitionParams) -> Option<LspLocation> {
        let content = self.documents.get(&params.uri)?;
        let token = find_token_at(content, params.line, params.character)?;

        // Search through document lines for a `let <token> =` binding.
        for (line_idx, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("let ") {
                let after_let = trimmed[4..].trim_start();
                if after_let.starts_with(token.as_str()) {
                    let remaining = &after_let[token.len()..];
                    let next_char = remaining.chars().next();
                    if next_char.map_or(true, |c| !c.is_alphanumeric() && c != '_') {
                        // Found the binding site.
                        let col = line.find(token.as_str()).unwrap_or(0) as u32;
                        let end_col = col + token.len() as u32;
                        return Some(LspLocation {
                            uri: params.uri.clone(),
                            range: LspRange {
                                start: LspPosition {
                                    line: line_idx as u32,
                                    character: col,
                                },
                                end: LspPosition {
                                    line: line_idx as u32,
                                    character: end_col,
                                },
                            },
                        });
                    }
                }
            }
        }
        None
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Extracts the identifier or keyword token at the given (line, character) position.
///
/// Returns `None` if the position is out of range or falls on whitespace/punctuation.
fn find_token_at(content: &str, line: u32, character: u32) -> Option<String> {
    let target_line = content.lines().nth(line as usize)?;
    let chars: Vec<char> = target_line.chars().collect();
    let char_idx = character as usize;

    if char_idx >= chars.len() {
        return None;
    }

    let c = chars[char_idx];
    if !c.is_alphanumeric() && c != '_' {
        return None;
    }

    // Expand left and right to find full token.
    let start = {
        let mut i = char_idx;
        while i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_') {
            i -= 1;
        }
        i
    };
    let end = {
        let mut i = char_idx + 1;
        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
            i += 1;
        }
        i
    };

    Some(chars[start..end].iter().collect())
}
