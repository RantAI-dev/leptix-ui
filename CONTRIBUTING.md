# Contributing to Leptix

## Development Setup

```bash
# Clone the repo
git clone https://github.com/rantai-dev/leptix.git
cd leptix

# Build
cargo build --workspace

# Run tests
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace -- -D warnings

# Build for WASM
rustup target add wasm32-unknown-unknown
cargo build --workspace --target wasm32-unknown-unknown
```

## Adding a New Component

1. Create a new crate under `crates/primitives/<component-name>/`
2. Add the crate to the workspace `Cargo.toml`
3. Add a feature flag and optional dependency in `crates/leptix-ui/`
4. Follow existing component patterns (see `leptix-label` for a minimal example)

## Code Style

- Run `cargo fmt` before committing
- All clippy warnings must be resolved (`-D warnings`)
- Follow existing naming conventions: `leptix-<component>` for crate names
