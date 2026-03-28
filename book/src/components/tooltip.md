# Tooltip

A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it.

## Features

- Opens on hover or focus, closes on pointer leave or blur
- Provider component for global delay configuration
- Can be controlled or uncontrolled
- Supports custom positioning via `side` prop
- Customizable open/close delays

## Installation

```toml
[dependencies]
leptix-tooltip = "1.0.0"
```

## Anatomy

```rust
use leptix_tooltip::*;

view! {
    <TooltipProvider>
        <Tooltip>
            <TooltipTrigger />
            <TooltipPortal>
                <TooltipContent>
                    <TooltipArrow />
                </TooltipContent>
            </TooltipPortal>
        </Tooltip>
    </TooltipProvider>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_tooltip::*;

#[component]
fn SaveButton() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger attr:class="icon-button">
                    "💾"
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent attr:class="tooltip-content" side="top">
                        "Save document"
                        <TooltipArrow attr:class="tooltip-arrow" />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}
```

## Styling

```css
.tooltip-content {
    background-color: #111827;
    color: white;
    border-radius: 4px;
    padding: 8px 12px;
    font-size: 13px;
    line-height: 1;
    box-shadow: 0 10px 38px -10px rgba(22, 23, 24, 0.35);
}

.tooltip-content[data-state="delayed-open"] {
    animation: fadeIn 200ms ease-out;
}

.tooltip-content[data-state="closed"] {
    animation: fadeOut 100ms ease-in;
}

.tooltip-arrow {
    fill: #111827;
}

@keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
}

@keyframes fadeOut {
    from { opacity: 1; }
    to { opacity: 0; }
}
```

## API Reference

### TooltipProvider

Wraps your app to provide global tooltip delay behavior.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `delay_duration` | `MaybeProp<i32>` | `700` | The duration in milliseconds from when the mouse enters a trigger until the tooltip opens. |
| `skip_delay_duration` | `MaybeProp<i32>` | `300` | How much time a user has to enter another trigger without incurring a delay again. |

### Tooltip

Contains all the parts of a tooltip.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `MaybeProp<bool>` | - | The controlled open state. |
| `default_open` | `MaybeProp<bool>` | `false` | The open state when initially rendered. |
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |
| `delay_duration` | `MaybeProp<i32>` | `700` | Override the provider's delay duration for this specific tooltip. |

### TooltipTrigger

The element that triggers the tooltip. By default, it wraps a button.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"delayed-open"` \| `"closed"` |

### TooltipPortal

When used, portals the content part into the `body`.

### TooltipContent

The component that pops out when the tooltip is open.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `side` | `MaybeProp<String>` | `"top"` | The preferred side of the trigger to render against. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"delayed-open"` \| `"closed"` |

### TooltipArrow

An optional arrow element to render alongside the tooltip.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `width` | `MaybeProp<f64>` | `10.0` | The width of the arrow in pixels. |
| `height` | `MaybeProp<f64>` | `5.0` | The height of the arrow in pixels. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Accessibility

Adheres to the [Tooltip WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/tooltip).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Tab` | Opens/closes the tooltip when the trigger receives/loses focus. |
| `Esc` | Closes the tooltip if open. |
