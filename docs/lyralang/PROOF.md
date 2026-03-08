# Proof Construction — LyraLang P1-025

## Overview

The proof construction module recognizes proof blocks in Lyra programs, extracts proof obligations, and produces verifiable proof artifacts. It integrates with the existing type system's `EffectAtom::Proof` and `Type::Evidence(EvidenceKind::Proof)` primitives.

Proof blocks may be written in two styles:

1. **Implicit blocks** — sequential `assume`, `assert_eq`, and `qed` calls accumulate into a single block, finalized when `qed()` appears.
2. **Explicit blocks** — delimited by `proof_begin()` / `proof_end()` markers.

## Builtin Operators

### `assume(expr)`

Records a hypothesis for the current proof block.

- Appends `"assume(<expr>)"` to the block's `hypotheses` list.
- Symbolic: the expression is captured as a textual snippet; no evaluation occurs.

### `assert_eq(lhs, rhs)`

Records a claim for the current proof block.

- Appends `"assert_eq(<lhs> == <rhs>)"` to the block's `claims` list.

### `qed()`

Discharges the current proof block.

- Sets `qed_present = true` on the block.
- For implicit blocks, triggers immediate finalization.
- For explicit blocks, marks discharge; `proof_end()` finalizes.

### `proof_begin()`

Opens an explicit proof block. Any prior implicit block is first flushed and finalized.

### `proof_end()`

Closes the current explicit proof block and finalizes it. Calling `proof_end()` without a preceding `proof_begin()` produces an `InvalidProofBlock` error.

## Validity Rule

A `ProofBlock` is **valid** iff:
- It has at least one `assume(...)` hypothesis, **and**
- It has at least one `assert_eq(...)` claim, **and**
- `qed()` is present.

## Obligations and Artifacts

For each `assert_eq(...)` claim in a block, a `ProofObligation` is created:
- **Discharged** when `qed_present = true`.
- **`obligation_id`** follows the scheme `"PO-{zero-based-index}"`.

For each valid (discharged) block, `ProofArtifact` entries are produced:
1. One `AssumptionRecord` per hypothesis.
2. One `ClaimVerification` per claim.
3. One `DischargeReceipt` for the block.
4. One `ProofSummary` for the block.

Artifact identifiers follow the scheme `"PA-{zero-based-index}"` in source order.

## Output Types

### `ProofCheckOutput`

```
normalized_source: String
judgment: Option<ProofProgramJudgment>
errors: Vec<ProofError>
```

### `ProofProgramJudgment`

```
module: Option<String>
proof_blocks: Vec<ProofBlock>
obligations: Vec<ProofObligation>
artifacts: Vec<ProofArtifact>
all_discharged: bool          -- true iff every obligation is discharged
span: SourceSpan
```

### `ProofBlock`

```
index: usize                  -- 0-based source order
hypotheses: Vec<String>
claims: Vec<String>
qed_present: bool
valid: bool
span: SourceSpan
```

### `ProofObligation`

```
obligation_id: String         -- "PO-{n}"
statement: String
discharged: bool
discharge_evidence: Option<String>
```

### `ProofArtifact`

```
artifact_id: String           -- "PA-{n}"
kind: ProofArtifactKind
description: String
verifiable: bool
```

## Error Kinds

| Kind | Label | Recovered |
|------|-------|-----------|
| `ParseError` | `parse_error` | varies |
| `UndischargedObligation` | `undischarged_obligation` | `true` |
| `MissingQed` | `missing_qed` | `true` |
| `InvalidProofBlock` | `invalid_proof_block` | `true` |

## Entry Point

```rust
use lyralang::proof::check;
let output = check(source);
```

Or via the checker struct:

```rust
use lyralang::proof::ProofChecker;
let checker = ProofChecker::new();
let output = checker.check_source(source);
```

## Relation to Type System

The type system anchors for proof construction are:

- `EffectAtom::Proof` — marks function effects as proof-producing.
- `EvidenceKind::Proof` — evidence token required for `Hypothesis → Fact` modal promotion.
- `Type::Evidence(EvidenceKind::Proof)` — the concrete evidence type.

The proof construction checker operates at the syntactic level, recognizing proof builtins and building structural artifacts. Full integration with the modal promotion pathway (e.g., `promote_hypothesis_to_fact`) is intended as a higher-stage feature.
