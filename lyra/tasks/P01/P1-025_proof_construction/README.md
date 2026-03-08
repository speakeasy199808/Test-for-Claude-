# P1-025 — Proof Construction

## Mission
Proof blocks in Lyra, automated theorem proving integration, proof extraction to verifiable artifacts.

## Scope
- normative specification and versioned contracts where required
- executable compiler pipeline surface in `lyralang/`
- fixtures, goldens, and integration validation
- task control-plane records and traceability artifacts

## Primary Ownership Root
`lyralang/`

## Deliverables
- `docs/lyralang/PROOF.md`
- `interfaces/specs/lyralang_proof_construction_v1.json`
- `lyralang/src/proof/mod.rs`
- `fixtures/lyralang/proof/proof_valid.lyra`
- `fixtures/lyralang/proof/proof_undischarged.lyra`
- `goldens/lyralang/proof/proof_valid.json`
- `goldens/lyralang/proof/proof_undischarged.json`

## Key Design Decisions
- Builds on `EffectAtom::Proof` and `Type::Evidence(EvidenceKind::Proof)` from the existing type kernel
- Proof blocks may be implicit (sequential `assume`/`assert_eq`/`qed` calls) or explicit (`proof_begin`/`proof_end` markers)
- A block is valid iff it has at least one hypothesis, at least one claim, and `qed()` is present
- `UndischargedObligation` errors are recovered (checking continues) but `all_discharged` is `false`
- `ProofArtifact` entries are only produced for discharged (valid) proof blocks
- Stable `"PO-{n}"` / `"PA-{n}"` identifiers enable deterministic golden comparison
