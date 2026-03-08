# Acceptance — P0-018 Error Code System

## Acceptance Criteria
1. `ErrorCode` newtype validates range 1–9999 and rejects 0 and >9999.
2. `ErrorCode` displays as zero-padded `E0001`–`E9999` format.
3. `ErrorCategory` enum covers 10 categories with correct code range mapping.
4. `ErrorEntry` contains code, category, message, explanation, and suggestion.
5. `ErrorCatalog` supports register, lookup, len, is_empty, entries, and by_category.
6. Default catalog contains seed entries across Constitutional, Codec, Digest, Time, Entropy, Incident, Verification, Resource, and Policy categories.
7. All codes in the default catalog are unique.
8. All codes in the default catalog fall within valid range.
9. Category assignment matches code range for all entries.
10. `ErrorEntry` serializes to JSON deterministically via serde.
11. All 10 unit tests pass.

## Verification Method
- `cargo test -p k0 --lib` — all error module tests pass
- Code review against declared scope

## Evidence Required
- Test pass output (270 tests, 0 failures)
- `artifacts/error-code-system-spec.md`
