# P1-030 Acceptance Criteria

## Functional

- [ ] `handle_request(Initialize(_))` returns `LspResponse::Initialized { server_name: "lyralang-lsp", server_version: "0.1.0" }`
- [ ] `handle_request(DidOpen { uri, content })` stores the document and returns `Ok`
- [ ] `handle_request(DidChange { uri, content })` updates the stored document and returns `Ok`
- [ ] After `DidOpen`, `get_diagnostics(uri)` returns type-checker errors for invalid programs
- [ ] After `DidOpen` with valid source, `get_diagnostics(uri)` returns an empty list
- [ ] `handle_request(Hover(_))` returns `Hover(Some(HoverResult))` when cursor is on a valid token
- [ ] `handle_request(Completion(_))` returns at least the 6 LyraLang keywords as `LspCompletionKind::Keyword` items
- [ ] `handle_request(GotoDefinition(_))` returns `GotoDefinition(Some(location))` when cursor is on a bound name
- [ ] `handle_request(GotoDefinition(_))` returns `GotoDefinition(None)` for an unknown identifier
- [ ] `handle_request(PublishDiagnostics { uri })` returns `Diagnostics(...)` populated from type checker
- [ ] `handle_request(Shutdown)` returns `Ok`

## Non-functional

- [ ] All public items have `///` doc comments
- [ ] All public types derive `Clone, Debug, Serialize, Deserialize`
- [ ] Document store uses `BTreeMap` (ordered, deterministic)
- [ ] Module is a library surface (no `main()`)
- [ ] Module builds without warnings under `#![deny(missing_docs)]`

## Fixtures match

- [ ] `fixtures/lyralang/lsp/lsp_hover.json` matches golden at `goldens/lyralang/lsp/lsp_hover.json`
- [ ] `fixtures/lyralang/lsp/lsp_diagnostics.json` is well-formed JSON
