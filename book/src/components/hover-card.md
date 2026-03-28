# Hover Card

A card that appears when a user hovers over or focuses a trigger element, useful for previewing content behind a link.

## Features

- Can be controlled or uncontrolled
- Configurable open and close delays
- Opens on hover and focus
- Supports custom positioning via arrow

## Installation

```toml
[dependencies]
leptix-hover-card = "1.0.0"
```

## Anatomy

```rust
use leptix_hover_card::*;

view! {
    <HoverCard>
        <HoverCardTrigger />
        <HoverCardPortal>
            <HoverCardContent>
                <HoverCardArrow />
            </HoverCardContent>
        </HoverCardPortal>
    </HoverCard>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_hover_card::*;

#[component]
fn UserCard() -> impl IntoView {
    view! {
        <HoverCard>
            <HoverCardTrigger attr:href="https://github.com/leptos-rs" attr:class="hover-trigger">
                "@leptos-rs"
            </HoverCardTrigger>
            <HoverCardPortal>
                <HoverCardContent attr:class="hover-content">
                    <div style="display: flex; flex-direction: column; gap: 7px;">
                        <img
                            src="https://avatars.githubusercontent.com/u/118383788"
                            alt="Leptos"
                            style="width: 60px; height: 60px; border-radius: 50%;"
                        />
                        <div>
                            <strong>"Leptos"</strong>
                            <p>"Full-stack Rust web framework."</p>
                        </div>
                    </div>
                    <HoverCardArrow attr:class="hover-arrow" />
                </HoverCardContent>
            </HoverCardPortal>
        </HoverCard>
    }
}
```

## Styling

```css
.hover-trigger {
    cursor: pointer;
    color: #4c9aff;
    text-decoration: underline;
}

.hover-content {
    border-radius: 6px;
    padding: 20px;
    width: 300px;
    background: white;
    box-shadow: 0 10px 38px rgba(0, 0, 0, 0.35), 0 10px 20px rgba(0, 0, 0, 0.2);
}

.hover-content[data-state="open"] {
    animation: fadeIn 200ms ease;
}

.hover-content[data-state="closed"] {
    animation: fadeOut 150ms ease;
}

.hover-arrow {
    fill: white;
}
```

## API Reference

### HoverCard

Contains all the parts of a hover card.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `MaybeProp<bool>` | - | The controlled open state. |
| `default_open` | `MaybeProp<bool>` | `false` | The open state when initially rendered. Use when you do not need to control the open state. |
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |
| `open_delay` | `MaybeProp<i32>` | `700` | The duration in milliseconds from when the mouse enters the trigger until the hover card opens. |
| `close_delay` | `MaybeProp<i32>` | `300` | The duration in milliseconds from when the mouse leaves the trigger or content until the hover card closes. |

### HoverCardTrigger

The link or element that opens the hover card on hover. Renders an `<a>` by default.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<a>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### HoverCardPortal

When used, portals the content part into the `body`.

### HoverCardContent

The component that pops out when the hover card is open.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### HoverCardArrow

An optional arrow element to render alongside the content.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `width` | `MaybeProp<f64>` | `10.0` | The width of the arrow in pixels. |
| `height` | `MaybeProp<f64>` | `5.0` | The height of the arrow in pixels. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Data Attributes

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

## Accessibility

The hover card is intended for sighted users who can hover or focus. The trigger renders as an `<a>` element by default, which is keyboard focusable. The content opens on `pointerenter`/`focus` and closes on `pointerleave`/`blur`.

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Tab` | Opens the hover card when focus moves to the trigger. |
| `Shift + Tab` | Closes the hover card when focus moves away from the trigger. |
