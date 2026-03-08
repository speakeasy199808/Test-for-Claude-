# Acceptance — P0-006 LyraCodec Spec

## Acceptance Criteria
1. Canonical encoding spec exists for:
   - varints
   - structs
   - vectors
   - maps
2. Versioned schema rules are defined.
3. Canonical form explicitly prohibits floating-point representation.
4. Worked examples and fixture vectors are present.
5. Traceability from rules -> fixtures -> expected bytes is documented.

## Verification Method
- Spec consistency review
- Fixture/schema validation
- Example vector byte-level verification

## Evidence
- `artifacts/codec-traceability.md`
- `fixtures/codec/*.json`
- `fixtures/codec/*.hex`
