# Implementation Notes — P0-006 LyraCodec Spec

## Work Package Shape

Spec-plus-fixtures (multi-module). This task is a Formal Specification work package. It does not produce production Rust code — that is the responsibility of P0-007 (`canonical_encoder`). This task produces the normative specification, schema examples, fixture vectors, and traceability artifacts that downstream implementation tasks consume.

## Produced Components

| Component | Location | Description |
|---|---|---|
| Canonical codec specification | `interfaces/specs/lyracodec.md` | Normative encoding rules for all Lyra data types |
| Varint fixture vectors (JSON) | `fixtures/codec/varint_examples.json` | Encoding cases with expected hex |
| Varint fixture vectors (hex) | `fixtures/codec/varint_examples.hex` | Raw canonical byte sequences |
| Struct fixture (JSON) | `fixtures/codec/struct_example.json` | Struct encoding example with expected hex |
| Struct fixture (hex) | `fixtures/codec/struct_example.hex` | Raw canonical byte sequence |
| Map ordering negative fixture | `fixtures/codec/map_ordering_invalid.json` | Rejected payload: non-canonical map ordering |
| Codec traceability artifact | `artifacts/codec-traceability.md` | Rule → fixture → expected outcome mapping |

## Ownership Placement

- **Primary**: `interfaces/` — normative ownership of canonical encoding contracts
- **Task control-plane evidence**: `lyra/tasks/P00/P0-006_lyracodec_spec/`
- **Shared fixture compatibility**: `fixtures/` mirrored references when promoted to shared corpus

## Dependency Posture

Spec-first. This task enables:
- **P0-007** `canonical_encoder` — consumes spec to implement Rust encoder/decoder
- **P0-011** `determinism_verifier` — uses fixture vectors as golden regression corpus
- **P0-020** `code_review_protocol` — references codec boundary contracts
- **P0-023** `foundation_integration` — validates codec round-trip in end-to-end test

## Acceptance Evidence Checklist

- [x] `interfaces/specs/lyracodec.md` — normative spec covering all declared types
- [x] `fixtures/codec/varint_examples.json` — varint encoding cases with hex vectors
- [x] `fixtures/codec/struct_example.json` — struct encoding example
- [x] `fixtures/codec/map_ordering_invalid.json` — negative fixture for map ordering
- [x] `fixtures/codec/varint_examples.hex` — raw hex byte sequences
- [x] `fixtures/codec/struct_example.hex` — raw hex byte sequence
- [x] `artifacts/codec-traceability.md` — rule-to-fixture traceability table
- [x] `DESIGN.md` — codec design rationale and layer model
- [x] `IMPLEMENTATION.md` — this file
- [x] `task.toml` — task metadata and completion gate
