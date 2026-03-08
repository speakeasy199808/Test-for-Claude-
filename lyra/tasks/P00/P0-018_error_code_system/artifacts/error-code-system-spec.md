# Error Code System Specification — Evidence Artifact

## Task: P0-018 Error Code System

## Code Format
`E{NNNN}` — zero-padded 4-digit number, range E0001–E9999.

## Category Map
| Range | Category | Seed Count |
|---|---|---|
| E0001–E0999 | Constitutional | 5 |
| E1000–E1999 | Codec | 5 |
| E2000–E2999 | Digest | 2 |
| E3000–E3999 | Time | 2 |
| E4000–E4999 | Entropy | 2 |
| E5000–E5999 | Incident | 2 |
| E6000–E6999 | Verification | 2 |
| E7000–E7999 | Resource | 1 |
| E8000–E8999 | Policy | 1 |
| E9000–E9999 | Reserved | 0 |
| **Total** | | **22** |

## Test Results
```
test errors::tests::all_codes_are_unique ... ok
test errors::tests::all_codes_in_valid_range ... ok
test errors::tests::category_matches_code_range ... ok
test errors::tests::default_catalog_is_not_empty ... ok
test errors::tests::error_code_above_9999_is_rejected ... ok
test errors::tests::error_code_display_format ... ok
test errors::tests::error_code_zero_is_rejected ... ok
test errors::tests::error_entry_to_json_is_deterministic ... ok
test errors::tests::lookup_by_code_works ... ok
test errors::tests::lookup_missing_code_returns_none ... ok
```

## Acceptance Verification
All 11 criteria passed. See ACCEPTANCE.md for details.
