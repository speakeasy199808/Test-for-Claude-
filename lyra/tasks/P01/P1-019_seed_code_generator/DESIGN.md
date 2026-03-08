# Design — P1-019 Seed Code Generator

## Design Summary

The Stage 0 code generator lowers the currently executable AST into a pre-serialization IR instead of raw bytes.
This keeps P1-019 dependency-safe with the existing parser and type checker while preserving a clean handoff to P1-020.

## Why Canonical Rendered Instructions

A rendered instruction stream is easy to diff, easy to validate in fixtures, and deterministic across hosts.
The byte-level encoder remains a separate concern for the next task.
