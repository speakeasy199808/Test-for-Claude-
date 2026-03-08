# LyraLang LSP Server (P1-030)

## Overview

The `lyralang::lsp` module provides Language Server Protocol request/response
handling as a library. It does **not** start a real TCP or stdio server — it
exposes an `LspServer` struct that routes typed `LspRequest` values to typed
`LspResponse` values. Any transport layer can wrap this engine.

## Request dispatch table

| Request variant | Response variant | Notes |
|-----------------|-----------------|-------|
| `Initialize(_)` | `Initialized { server_name, server_version }` | Always `"lyralang-lsp" / "0.1.0"` |
| `DidOpen { uri, content }` | `Ok` | Stores document; refreshes diagnostics |
| `DidChange { uri, content }` | `Ok` | Updates document; refreshes diagnostics |
| `Hover(params)` | `Hover(Option<HoverResult>)` | Token-at-cursor type info |
| `Completion(params)` | `Completion(Vec<LspCompletionItem>)` | Keywords + builtins |
| `GotoDefinition(params)` | `GotoDefinition(Option<LspLocation>)` | `let` binding sites |
| `PublishDiagnostics { uri }` | `Diagnostics(Vec<LspDiagnostic>)` | Runs `checker::check` |
| `Shutdown` | `Ok` | No-op |

## Quick start

```rust
use lyralang::lsp::{LspServer, LspRequest, InitializeParams, ClientCapabilities};

let mut server = LspServer::new();

// Handshake
server.handle_request(LspRequest::Initialize(InitializeParams {
    client_name: Some("my-editor".into()),
    capabilities: ClientCapabilities::default(),
}));

// Open a document
server.handle_request(LspRequest::DidOpen {
    uri: "file:///workspace/main.lyra".into(),
    content: "let x = 42".into(),
});

// Diagnostics
let diags = server.get_diagnostics("file:///workspace/main.lyra");
// diags is empty for valid source
```

## Document store

Documents are stored in a `BTreeMap<String, String>` keyed by URI.
Diagnostics are cached in a parallel `BTreeMap<String, Vec<LspDiagnostic>>`.
Both maps are ordered for deterministic iteration.

## Hover

`Hover` finds the alphanumeric token at the given (line, character) position,
runs `checker::check` on it, and returns a markdown snippet:

```
```lyra
x : Int
```
```

## Go-to-definition

`GotoDefinition` searches the document lines for a `let <token> =` binding
matching the token under the cursor and returns the line/column range of the
bound name.

## Diagnostics

`PublishDiagnostics` (and `update_document`) run `checker::check` and convert
every `TypeError` to an `LspDiagnostic` with `severity = Error` and
`source = "lyralang"`. Line/column offsets are converted from 1-based
(lexer convention) to 0-based (LSP convention).

## Completion items

Completion always returns the full set of LyraLang keywords and builtins.
Position-sensitive filtering (triggered by a prefix) is the caller's
responsibility.

## Types reference

See `interfaces/specs/lyralang_lsp_v1.json`.

## Fixtures

- `fixtures/lyralang/lsp/lsp_hover.json`
- `fixtures/lyralang/lsp/lsp_diagnostics.json`

## Goldens

- `goldens/lyralang/lsp/lsp_hover.json`
