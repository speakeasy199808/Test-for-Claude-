# Acceptance — P1-025 Proof Construction

## Acceptance Criteria

1. `assume(expr)` calls add hypotheses to the current proof block.
2. `assert_eq(lhs, rhs)` calls add claims to the current proof block.
3. `qed()` marks the current block as discharged.
4. `proof_begin()` / `proof_end()` provide explicit block delimiters.
5. A block with claims but no `qed()` produces an `UndischargedObligation` error (recovered = `true`).
6. `ProofObligation` entries carry stable `"PO-{n}"` identifiers.
7. `ProofArtifact` entries are only produced for valid (discharged) proof blocks.
8. `all_discharged` is `true` iff every obligation was discharged.
9. Fixtures and goldens cover both valid and undischarged error cases.
10. `ProofCheckOutput` serializes deterministically.

## Verification Method
- Review `lyralang/src/proof/mod.rs`
- Inspect fixture `fixtures/lyralang/proof/proof_valid.lyra` against golden `goldens/lyralang/proof/proof_valid.json`
- Inspect fixture `fixtures/lyralang/proof/proof_undischarged.lyra` against golden `goldens/lyralang/proof/proof_undischarged.json`

## Evidence Required
- `docs/lyralang/PROOF.md`
- `interfaces/specs/lyralang_proof_construction_v1.json`
- `lyralang/src/proof/mod.rs`
- `fixtures/lyralang/proof/proof_valid.lyra`
- `fixtures/lyralang/proof/proof_undischarged.lyra`
- `goldens/lyralang/proof/proof_valid.json`
- `goldens/lyralang/proof/proof_undischarged.json`
