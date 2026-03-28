# Leptix

Radix-style accessible UI primitives for [Leptos](https://leptos.dev/).

Leptix provides unstyled, accessible components that you can customize with your own styles. Each component follows WAI-ARIA patterns and handles keyboard navigation, focus management, and screen reader support.

## Installation

```toml
[dependencies]
leptix-ui = "1.0.0"
```

## Quick Start

```rust
use leptos::prelude::*;
use leptix_ui::label::Label;

#[component]
fn App() -> impl IntoView {
    view! {
        <Label>"Name"</Label>
    }
}
```
