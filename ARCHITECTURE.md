# Architecture

Keep this file short (~1 page). Project-specific invariants live here; universal Rust workflow lives in `AGENTS.md`.

## Purpose
`primitives` is a Cargo workspace (`resolver = "3"`, edition 2024) of small, hand-written systems data structures for learning (see `README.md`). Each crate is an independent library. It is not a production library, and the README states the author writes the code themselves, using AI mainly for explanation and debugging.

## Module Boundaries
- `crates/ring-buffer` (`RingBuffer<T>`): fixed-capacity FIFO over `Vec<Option<T>>` with `head`/`tail`/`size`; `push` on a full buffer returns `Err(value)` and never overwrites; `new(0)` is an error.
- `crates/vector` (`MyVec<T>`): hand-rolled growable vector over `NonNull<T>` with raw `std::alloc` calls. Work in progress at HEAD: there is no `Drop` impl, and `push` calls `self.grow()?` while `grow` returns `()`, so don't assume the crate builds.
- `crates/bloom-filter` (`BloomFilter`): placeholder at HEAD (a `Vec<u8>` with linear `contains`, not a real Bloom filter; private struct).
- No crate depends on another; every crate has an empty `[dependencies]` and inherits `version`/`edition` from `[workspace.package]`.

## Core Invariants
- Crates stay dependency-free (std only) and independent of each other.
- `ring-buffer`: `len() <= capacity()`, and a full buffer rejects pushes by returning the value.
- `vector`: documented constructor invariants at the top of `impl<T> MyVec<T>` in `crates/vector/src/lib.rs` (`len <= cap`, `buf` non-null, dangling when `cap == 0`, no initialized elements while `len == 0`).
- Fallible operations return `Result<_, T>` giving the value back (`push`, `set`) rather than panicking.

## Forbidden Patterns / Dependencies
- `unsafe` is confined to `crates/vector` (raw allocation and pointer reads/writes). Don't add it to other crates.
- Tests are unit tests inside each crate's `src/lib.rs`; HEAD has no `tests/` directories.
- The workspace root has no `[lints]` table at HEAD, so the strict-lint wording in `AGENTS.md` is not yet enforced here.
