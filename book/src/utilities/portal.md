# Portal

Renders a Leptos subtree in a different part of the DOM. Used by Dialog, Popover, Tooltip, and other overlay components to render content outside the normal DOM hierarchy.

## Features

- Renders children in a portal container (defaults to `document.body`)
- Preserves Leptos reactive context across the portal boundary
- Useful for avoiding CSS overflow/stacking context issues

## Installation

Part of `leptix-core` (included automatically with `leptix-ui`).

## Anatomy

```rust
use leptix_core::portal::Portal;

view! {
    <Portal>
        // This content renders at the end of <body>
        <div class="floating-panel">"I'm portalled!"</div>
    </Portal>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_core::portal::Portal;

#[component]
fn FloatingPanel() -> impl IntoView {
    view! {
        <div style="overflow: hidden;">
            // Without Portal, this would be clipped by overflow:hidden
            <Portal>
                <div style="position: fixed; top: 10px; right: 10px;">
                    "I escape the overflow hidden parent!"
                </div>
            </Portal>
        </div>
    }
}
```

## API Reference

### Portal

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

Most overlay components (Dialog, Popover, Tooltip, etc.) include their own Portal sub-component, so you typically don't need to use this directly.
