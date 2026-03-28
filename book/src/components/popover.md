# Popover

Displays rich content in a portal, triggered by a button.

## Features

- Can be controlled or uncontrolled
- Focus is managed and trapped within the popover
- Supports custom anchor elements
- Dismisses on escape or click outside
- Customizable arrow element

## Installation

```toml
[dependencies]
leptix-popover = "1.0.0"
```

## Anatomy

```rust
use leptix_popover::*;

view! {
    <Popover>
        <PopoverTrigger />
        <PopoverAnchor />
        <PopoverPortal>
            <PopoverContent>
                <PopoverArrow />
                <PopoverClose />
            </PopoverContent>
        </PopoverPortal>
    </Popover>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_popover::*;

#[component]
fn DimensionsPopover() -> impl IntoView {
    view! {
        <Popover>
            <PopoverTrigger attr:class="popover-trigger">
                "Edit dimensions"
            </PopoverTrigger>
            <PopoverPortal>
                <PopoverContent attr:class="popover-content">
                    <div style="display: flex; flex-direction: column; gap: 10px;">
                        <p style="margin: 0; font-weight: 500;">"Dimensions"</p>
                        <fieldset style="display: flex; align-items: center; gap: 8px;">
                            <label r#for="width">"Width"</label>
                            <input id="width" value="100%" />
                        </fieldset>
                        <fieldset style="display: flex; align-items: center; gap: 8px;">
                            <label r#for="height">"Height"</label>
                            <input id="height" value="25px" />
                        </fieldset>
                    </div>
                    <PopoverArrow attr:class="popover-arrow" />
                    <PopoverClose attr:class="popover-close">"X"</PopoverClose>
                </PopoverContent>
            </PopoverPortal>
        </Popover>
    }
}
```

## Styling

```css
.popover-content {
    background: white;
    border-radius: 6px;
    padding: 20px;
    width: 260px;
    box-shadow: 0 10px 38px -10px rgba(22, 23, 24, 0.35);
}

.popover-content[data-state="open"] {
    animation: scaleIn 100ms ease-out;
}

.popover-content[data-state="closed"] {
    animation: scaleOut 100ms ease-in;
}

.popover-arrow {
    fill: white;
}

.popover-close {
    position: absolute;
    top: 5px;
    right: 5px;
}
```

## API Reference

### Popover

Contains all the parts of a popover.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `MaybeProp<bool>` | - | The controlled open state. |
| `default_open` | `MaybeProp<bool>` | `false` | The open state when initially rendered. |
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |

### PopoverTrigger

The button that toggles the popover.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### PopoverAnchor

An optional element to position the popover against. If not used, the trigger will be used as the anchor.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### PopoverPortal

When used, portals the content part into the `body`.

### PopoverContent

The component that pops out when the popover is open.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `on_open_auto_focus` | `Option<Callback<Event>>` | - | Event handler called when focus moves into the component after opening. Can be prevented. |
| `on_close_auto_focus` | `Option<Callback<Event>>` | - | Event handler called when focus moves to the trigger after closing. Can be prevented. |
| `on_escape_key_down` | `Option<Callback<KeyboardEvent>>` | - | Event handler called when the escape key is down. Can be prevented. |
| `on_pointer_down_outside` | `Option<Callback<PointerEvent>>` | - | Event handler called when a pointer event happens outside. Can be prevented. |
| `on_interact_outside` | `Option<Callback<Event>>` | - | Event handler called when an interaction happens outside the component. Can be prevented. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### PopoverClose

The button that closes the popover.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### PopoverArrow

An optional arrow element to render alongside the popover.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `width` | `MaybeProp<f64>` | `10.0` | The width of the arrow in pixels. |
| `height` | `MaybeProp<f64>` | `5.0` | The height of the arrow in pixels. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Accessibility

Adheres to the [Dialog WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Opens/closes the popover when focus is on the trigger. |
| `Enter` | Opens/closes the popover when focus is on the trigger. |
| `Tab` | Moves focus to the next focusable element within the popover. |
| `Shift + Tab` | Moves focus to the previous focusable element within the popover. |
| `Esc` | Closes the popover and moves focus to the trigger. |
