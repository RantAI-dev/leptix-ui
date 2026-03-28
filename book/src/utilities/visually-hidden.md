# Visually Hidden

Hides content from the screen in an accessible way. The content remains available to screen readers.

## Features

- Content is visually hidden but remains in the accessibility tree
- Useful for providing context to screen reader users
- No layout impact on surrounding elements

## Installation

Part of `leptix-core` (included automatically with `leptix-ui`).

## Anatomy

```rust
use leptix_core::visually_hidden::VisuallyHidden;

view! {
    <VisuallyHidden>"Screen reader only text"</VisuallyHidden>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_core::visually_hidden::VisuallyHidden;

#[component]
fn IconButton() -> impl IntoView {
    view! {
        <button>
            // Visible icon
            <svg>/* gear icon */</svg>
            // Accessible label
            <VisuallyHidden>"Settings"</VisuallyHidden>
        </button>
    }
}
```

## API Reference

### VisuallyHidden

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<span>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

The component applies CSS that visually hides the element while keeping it accessible:
- `position: absolute`
- `width: 1px; height: 1px`
- `overflow: hidden`
- `clip: rect(0, 0, 0, 0)`
