# P1-030 — LSP Server

## Summary

Implements Language Server Protocol request/response handling as a library
module (`lyralang::lsp`). Provides document management, hover, completion,
go-to-definition, and push diagnostics. No TCP or stdio transport is included —
the engine can be wired up to any transport layer.

## Module

`lyralang/src/lsp/mod.rs`

## Public surface

| Item | Kind | Description |
|------|------|-------------|
| `LspServer` | struct | LSP engine maintaining document state |
| `LspServer::new()` | fn | Create a fresh server |
| `LspServer::handle_request(&mut self, LspRequest) -> LspResponse` | fn | Dispatch one request |
| `LspServer::update_document(&mut self, uri, content)` | fn | Store document + refresh diagnostics |
| `LspServer::get_diagnostics(&self, uri) -> Vec<LspDiagnostic>` | fn | Return cached diagnostics |
| `LspRequest` | enum | All inbound request variants |
| `LspResponse` | enum | All outbound response variants |
| `InitializeParams` | struct | Handshake params |
| `ClientCapabilities` | struct | Feature flags from client |
| `HoverParams` | struct | Hover cursor location |
| `CompletionParams` | struct | Completion cursor location |
| `GotoDefinitionParams` | struct | Definition cursor location |
| `HoverResult` | struct | Hover markdown + optional range |
| `LspCompletionItem` | struct | Single completion candidate |
| `LspCompletionKind` | enum | Keyword / Function / Variable / Type / Module |
| `LspLocation` | struct | URI + range |
| `LspRange` | struct | start/end positions |
| `LspPosition` | struct | line/character |
| `LspDiagnostic` | struct | Compiler diagnostic in LSP format |
| `LspDiagnosticSeverity` | enum | Error / Warning / Information / Hint |
| `LspError` | struct | Engine-level error |
| `LspErrorKind` | enum | ParseError / TypeError / InvalidRequest / DocumentNotFound |

## Request dispatch

| Request | Response |
|---------|----------|
| `Initialize` | `Initialized { server_name: "lyralang-lsp", server_version: "0.1.0" }` |
| `DidOpen` / `DidChange` | `Ok` (document stored; diagnostics refreshed) |
| `Hover` | `Hover(Some(HoverResult))` or `Hover(None)` |
| `Completion` | `Completion(Vec<LspCompletionItem>)` |
| `GotoDefinition` | `GotoDefinition(Some(LspLocation))` or `GotoDefinition(None)` |
| `PublishDiagnostics` | `Diagnostics(Vec<LspDiagnostic>)` |
| `Shutdown` | `Ok` |

## Document store

- `BTreeMap<String, String>` — ordered by URI for determinism
- Diagnostics cached in `BTreeMap<String, Vec<LspDiagnostic>>`

## Dependencies

- `crate::checker::check` — diagnostics and type information for hover

## Fixtures

- `fixtures/lyralang/lsp/lsp_hover.json`
- `fixtures/lyralang/lsp/lsp_diagnostics.json`

## Goldens

- `goldens/lyralang/lsp/lsp_hover.json`
