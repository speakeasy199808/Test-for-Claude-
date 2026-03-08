# Design — P1-022 Seed Test Framework

## Design Summary

The seed test framework centralizes fixture/golden lookup and canonical JSON comparison while keeping property tests inside `cargo test`.

## Constraint Notes

- no external runner assumptions
- deterministic helper behavior only
- property tests prove determinism, not probabilistic quality metrics
