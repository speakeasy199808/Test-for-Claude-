# P0-004 — Repo Architecture

## Mission
Create all canonical root directories and initialize the Cargo workspace with all member crates declared.

## Scope
- Create all directories: `lyralang/`, `k0/`, `k1/`, `shells/`, `slices/`
- Initialize Cargo workspace with all member crates declared
- Establish ownership-aligned structure matching the canonical ownership zones

## Primary Archetype
Core Module Implementation

## Work Package Class
multi-module

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
`lyralang/`, `k1/`, `shells/`, `slices/`, `interfaces/`, `lyra/tasks/`

## Deliverables
- Root `Cargo.toml` workspace manifest
- Canonical crate stubs for all ownership roots
- Workspace manifest artifact
- Touched-roots manifest
- Task control-plane files
