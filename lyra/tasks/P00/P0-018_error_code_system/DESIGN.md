# Design — P0-018 Error Code System

## Architecture

### Type Hierarchy
```
ErrorCode (u16 newtype, 1–9999)
  └── Display: "E{:04}" format
ErrorCategory (enum, 10 variants)
  └── from_code(): derives category from code range
ErrorEntry (struct)
  ├── code: ErrorCode
  ├── category: ErrorCategory
  ├── message: &'static str
  ├── explanation: &'static str
  └── suggestion: &'static str
ErrorCatalog (Vec<ErrorEntry>)
  ├── register()
  ├── lookup()
  ├── by_category()
  └── default_catalog()
```

### Category Ranges
| Range | Category | Domain |
|---|---|---|
| E0001–E0999 | Constitutional | Determinism, trust, genesis |
| E1000–E1999 | Codec | Serialization, encoding |
| E2000–E2999 | Digest | Hashing, cryptographic |
| E3000–E3999 | Time | Virtual clock, causal ordering |
| E4000–E4999 | Entropy | Randomness, pool |
| E5000–E5999 | Incident | Incident handling, recovery |
| E6000–E6999 | Verification | Determinism proofs, drift |
| E7000–E7999 | Resource | Memory, capacity |
| E8000–E8999 | Policy | Governance, access control |
| E9000–E9999 | Reserved | Future extension |

### Default Catalog Seed Entries
- 5 Constitutional errors (E0001–E0005)
- 5 Codec errors (E1000–E1004)
- 2 Digest errors (E2000–E2001)
- 2 Time errors (E3000–E3001)
- 2 Entropy errors (E4000–E4001)
- 2 Incident errors (E5000–E5001)
- 2 Verification errors (E6000–E6001)
- 1 Resource error (E7000)
- 1 Policy error (E8000)
- **Total: 22 seed entries**

## Design Decisions
1. **Vec-based catalog** — avoids HashMap iteration nondeterminism; O(n) lookup is acceptable for <1000 entries
2. **`&'static str` for messages** — zero-allocation error metadata; entries are compile-time constants
3. **Serde derive** — enables JSON serialization for machine-readable error output
4. **Category from range** — deterministic category assignment, no manual tagging errors
5. **Newtype ErrorCode** — prevents raw u16 confusion, enforces valid range at construction
