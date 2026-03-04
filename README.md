# primitives

A collection of small, focused systems/data-structure primitives built for learning and experimentation.

## Goal

Build many tiny projects that each teach one core idea, with real hands-on practice.

This repo is intentionally "minimal to no AI coding":

- write the code myself
- use AI only for learning (concepts, debugging guidance, explanations)
- prioritize correctness first
- keep APIs small and tests simple
- increase complexity incrementally

## Potential Projects

- `bloom-filter`
- `block-cache`
- `kv-store`
- `lru-cache`
- `radix-sort`
- `ring-buffer`
- `thread-pool`
- `wal`
- `b-tree`

## Quick start

All primitives are managed from the repo root with a single Cargo workspace manifest.

Example (Rust project):

```bash
cargo test -p bloom-filter
```

