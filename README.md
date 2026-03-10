# primitives

`primitives` is a Rust workspace for building small systems components for learning and experimentation.

The goal is to learn core storage and systems ideas by implementing focused primitives by hand and growing them incrementally over time.

## Approach

- implement core ideas directly
- keep APIs small and explicit
- prioritize correctness and understanding over speed
- add complexity incrementally

AI usage is intentionally limited:

- write the code myself
- use AI primarily for learning, explanation, and debugging help
- avoid outsourcing the core implementation work
- keep the feedback loop grounded in direct understanding

## Current workspace

Project Ideas:

Core data primitives:
- [X] `ring-buffer`
- [ ] `bitmap`
- [ ] `bloom-filter`
- [ ] `heap`
- [ ] `arena`
- [ ] `lru-cache`
- [ ] `hyperloglog`
- [ ] `skiplist`
- [ ] `b-tree`

Storage engine primitives:
- [ ] `wal`
- [ ] `segment-log`
- [ ] `memtable`
- [ ] `sstable`
- [ ] `block-cache`
- [ ] `kv-store`

ML primitives:
- [ ] `tensor`
- [ ] `tokenizer`
- [ ] `sampler`
- [ ] `kv-cache`
- [ ] `quantization`
- [ ] `matmul-kernel`

## Quick start

All crates are managed from the workspace root.

Run tests for a single crate:

```bash
cargo test -p ring-buffer
```

Run all workspace tests:

```bash
cargo test
```
