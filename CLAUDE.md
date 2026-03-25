# Gradient Tool — Project Guide

## Project Overview

A web application where users can:
1. Upload a photo → extract dominant colors → generate a gradient
2. Manually pick colors → generate a gradient

The project is split into a **Rust backend** (this repo) and a **frontend** (separate, TBD).

---

## Architecture

```
Gradient-Tool-with-Rust/
├── CLAUDE.md
├── Cargo.toml          # workspace root
├── crates/
│   └── api/            # Axum HTTP API server
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── routes/
│           │   ├── mod.rs
│           │   ├── gradient.rs   # POST /gradient/from-colors
│           │   └── image.rs      # POST /image/extract-colors
│           ├── models.rs         # shared request/response types
│           └── error.rs          # unified error handling
└── crates/
    └── gradient/        # pure gradient logic (no HTTP)
        ├── Cargo.toml
        └── src/
            ├── lib.rs
            ├── color.rs          # Color type, conversions, blending
            ├── extractor.rs      # dominant color extraction from image
            └── gradient.rs       # gradient stop generation
```

## Tech Stack

| Layer | Choice | Why |
|---|---|---|
| HTTP framework | [Axum](https://github.com/tokio-rs/axum) | async, ergonomic, tower-compatible |
| Async runtime | Tokio | de-facto standard |
| Image decoding | `image` crate | wide format support |
| Color extraction | k-means clustering (`kmeans_colors`) | fast, accurate |
| Serialization | `serde` + `serde_json` | standard |
| CORS | `tower-http` | needed for browser frontend |

---

## API Endpoints

### `POST /gradient/from-colors`
Generate a gradient from a list of hex colors.

**Request**
```json
{
  "colors": ["#ff0000", "#0000ff"],
  "steps": 10
}
```

**Response**
```json
{
  "stops": [
    { "position": 0.0, "hex": "#ff0000" },
    { "position": 0.5, "hex": "#7f007f" },
    { "position": 1.0, "hex": "#0000ff" }
  ]
}
```

### `POST /image/extract-colors`
Upload an image, get back dominant colors + gradient.

**Request**: `multipart/form-data` with field `image` (JPEG/PNG/WEBP).

**Response**
```json
{
  "dominant_colors": ["#c23b22", "#4a90d9", "#f5f0e8"],
  "gradient": {
    "stops": [...]
  }
}
```

---

## Development

```bash
# Run the API server (default port 3000)
cargo run -p api

# Run all tests
cargo test

# Format + lint
cargo fmt && cargo clippy
```

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `PORT` | `3000` | HTTP listen port |
| `RUST_LOG` | `info` | Log level (`debug`, `info`, `warn`, `error`) |
| `MAX_IMAGE_BYTES` | `10485760` (10 MB) | Max upload size |

---

## Coding Conventions

- Use `thiserror` for error types, `anyhow` only in `main.rs` / test code.
- All public API types derive `serde::Serialize` + `serde::Deserialize`.
- Keep HTTP handler functions thin — business logic lives in the `gradient` crate.
- No `unwrap()` / `expect()` outside of tests.
- Run `cargo clippy -- -D warnings` before committing.
