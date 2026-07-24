# Agent Instructions

This document provides build, test, and convention guidance for agents working on `rusTy`.

## Overview

`rusTy` is a Rust crate providing idiomatic bindings to the Python spaCy NLP library. It uses:
- **pyo3** 0.21 for Python interop
- **rust-numpy** 0.21 for zero-copy vector access
- **thiserror** for error handling
- **serde/serde_json** for JSON serialization

## Build Requirements

- Rust toolchain (stable, 2021 edition)
- Python 3.8+ with spaCy installed
- A spaCy model (e.g., `en_core_web_sm`)

## Setup

```bash
# Install Python dependencies
pip install spacy
python -m spacy download en_core_web_sm

# Build
cargo build

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test
```

## Architecture

```
src/
  lib.rs       # Crate root, re-exports
  error.rs     # SpaCyError enum (no unwrap in library code)
  utils.rs     # GIL helpers, numpy extraction, kwargs builder
  language.rs  # Language (model loading, pipeline, batch processing)
  doc.rs       # Doc (tokens, spans, vectors, serialization)
  token.rs     # Token (all linguistic attributes)
  span.rs      # Span (sub-doc slices, vector access)
  vocab.rs     # Vocab & StringStore
```

## Conventions

- **No `.unwrap()` in library code.** All operations return `Result<T, SpaCyError>`.
- **GIL acquisition:** Use `utils::with_gil()` or `Python::with_gil()` directly.
- **pyo3 API:** Prefer `import_bound()` over deprecated `import()`. Use `Bound<'_, PyAny>`.
- **Numpy:** Use `extract_vec_f32()` in `utils.rs` for vector extraction.
- **Structs:** All wrapper structs (`Doc`, `Token`, `Span`, `Language`, `Vocab`) hold `Py<PyAny>` internally.
- **Tests:** Integration tests go in `tests/integration_tests.rs`. They require `en_core_web_sm` installed.

## CI

GitHub Actions runs on every push/PR:
1. Set up Python 3.11
2. Install spaCy + `en_core_web_sm`
3. `cargo fmt --check`
4. `cargo clippy -- -D warnings`
5. `cargo build`
6. `cargo test`

## Adding Features

When adding new spaCy attributes or methods:
1. Add to the appropriate struct (`Doc`, `Token`, `Span`, or `Language`)
2. Use `with_gil(|py| { ... })` for Python access
3. Return `Result<T, SpaCyError>`
4. Add an integration test in `tests/integration_tests.rs`
5. Update `README.md` with the new API
