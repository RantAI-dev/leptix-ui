# leptix-ui

Umbrella crate for [Leptix](https://github.com/RantAI-dev/leptix-ui) -- 33 Radix-quality accessible UI primitives for Leptos.

[![Crates.io](https://img.shields.io/crates/v/leptix-ui.svg)](https://crates.io/crates/leptix-ui)
[![docs.rs](https://img.shields.io/docsrs/leptix-ui)](https://docs.rs/leptix-ui)

## Installation

```toml
[dependencies]
leptix-ui = "0.1"
```

All 33 primitives are enabled by default. To reduce compile times, disable defaults and pick what you need:

```toml
[dependencies]
leptix-ui = { version = "0.1", default-features = false, features = ["dialog", "tabs", "tooltip"] }
```

## Usage

Each primitive is available as a module:

```rust
use leptix_ui::dialog::*;
use leptix_ui::tabs::*;
use leptix_ui::tooltip::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"Open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay />
                <DialogContent>
                    <DialogTitle>"Hello"</DialogTitle>
                    <DialogClose>"Close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}
```

## Alternatively: individual crates

If you prefer not to use the umbrella, depend on primitives directly:

```toml
[dependencies]
leptix-dialog = "0.1"
leptix-tabs = "0.1"
```

## Full documentation

See the [Leptix README](https://github.com/RantAI-dev/leptix-ui) for the complete component list, features, and architecture overview.

## License

MIT
