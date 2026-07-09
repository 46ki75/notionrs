# notionrs

A Notion API client implemented in Rust.

## Official Documentation Index

Notion provides an official documentation index at [llms.txt](https://developers.notion.com/llms.txt). You must check this documentation index and the documentation every single time you implement something.

## Directory Structure

- `notionrs/`: Main crate. Contains a client that sends API requests to Notion API endpoints.
- `notionrs_macro/`: Provides derive macros. It is published to crates.io, but its API is intended for internal use only.
- `notionrs_types/`: Defines structs used in other crates.
- `notionrs_webhooks/`: Verifies Notion webhook signatures and deserializes webhook events.

### Internal Dependencies

- `notionrs` ← `notionrs_types`
- `notionrs` ← `notionrs_macro`
- `notionrs_types` ← `notionrs_macro`
- `notionrs_webhooks` ← `notionrs_types`

### Other Files

- `scripts/check-upstream-release`: Checks releases in the `makenotion/notion-sdk-js` repository to stay up to date with API changes. If new releases are detected, create an issue in this repository.

## MSRV

MSRV (Minimum Supported Rust Version) is specified in `Cargo.toml` and `rust-toolchain.toml`.

## Git Tag Rules

- `notionrs` → without prefix (e.g., v0.1.0)
- `notionrs_macro`, `notionrs_types`, `notionrs_webhooks` → with prefix (e.g., macro-v0.1.0, webhooks-v0.2.0)
- Every release bumps only the minor version (`0.Y.0`), regardless of whether it includes breaking changes.

## Test Coverage

Test coverage uses [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov). Run via the `Justfile` recipe:

```bash
just coverage   # cargo llvm-cov --show-missing-lines
```

Treat coverage % as a guardrail (fail CI on drops), not a target to maximize.

## Formatting & Linting

- `cargo fmt --all -- --check` has pre-existing drift across many unrelated files (local rustfmt output differs from whatever produced the checked-in formatting) — format only the files you actually touched; don't reformat the repo as a side effect of a scoped change.
- `cargo clippy --workspace` cannot currently complete: a non-semver `since` in a `#[deprecated(...)]` on `FileUpload::archived` (`notionrs_types/src/object/file_upload.rs`) hard-errors the clippy driver for every dependent crate (tracked in #629). `cargo build`/`check`/`test` are unaffected — use those to verify, and check a clean `main` checkout before assuming a fmt/clippy failure is something you introduced.
