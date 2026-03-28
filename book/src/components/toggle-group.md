# Toggle Group

A set of two-state buttons that can be toggled on or off as a group.

## Features

- Supports single and multiple selection modes
- Can be controlled or uncontrolled
- Full keyboard navigation
- Supports disabled state per item or for the entire group

## Installation

```toml
[dependencies]
leptix-toggle-group = "1.0.0"
```

## Anatomy

```rust
use leptix_toggle_group::*;

view! {
    <ToggleGroup r#type=ToggleGroupType::Single>
        <ToggleGroupItem value="a" />
        <ToggleGroupItem value="b" />
        <ToggleGroupItem value="c" />
    </ToggleGroup>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_toggle_group::*;

#[component]
fn TextAlignment() -> impl IntoView {
    view! {
        <ToggleGroup
            r#type=ToggleGroupType::Single
            default_value=vec!["center".to_string()]
            attr:class="toggle-group"
            attr:aria-label="Text alignment"
        >
            <ToggleGroupItem value="left" attr:class="toggle-item" attr:aria-label="Align left">
                "L"
            </ToggleGroupItem>
            <ToggleGroupItem value="center" attr:class="toggle-item" attr:aria-label="Align center">
                "C"
            </ToggleGroupItem>
            <ToggleGroupItem value="right" attr:class="toggle-item" attr:aria-label="Align right">
                "R"
            </ToggleGroupItem>
        </ToggleGroup>
    }
}
```

## Styling

```css
.toggle-group {
    display: inline-flex;
    background-color: #f0f0f0;
    border-radius: 4px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.toggle-item {
    background: transparent;
    border: none;
    color: #666;
    height: 35px;
    width: 35px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 15px;
    line-height: 1;
    cursor: default;
}

.toggle-item:first-child {
    border-top-left-radius: 4px;
    border-bottom-left-radius: 4px;
}

.toggle-item:last-child {
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
}

.toggle-item:hover {
    background-color: #e0e0e0;
}

.toggle-item[data-state="on"] {
    background-color: #4c9aff;
    color: white;
}

.toggle-item[data-disabled] {
    opacity: 0.5;
    pointer-events: none;
}

.toggle-item:focus {
    position: relative;
    box-shadow: 0 0 0 2px black;
}
```

## API Reference

### ToggleGroup

The root container. Renders a `<div>` with `role="group"`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `r#type` | `ToggleGroupType` | *required* | The selection mode: `ToggleGroupType::Single` or `ToggleGroupType::Multiple`. |
| `value` | `MaybeProp<Vec<String>>` | - | The controlled value(s). For single mode, use a single-element vector. |
| `default_value` | `MaybeProp<Vec<String>>` | - | The value(s) when initially rendered. |
| `on_value_change` | `Option<Callback<Vec<String>>>` | - | Event handler called when the value changes. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with any item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<div>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-disabled]` | Present when the entire group is disabled. |

### ToggleGroupItem

An individual toggle button within the group.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | *required* | A unique value that identifies this item. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with this item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"on"` \| `"off"` |
| `[data-disabled]` | Present when disabled. |

## Data Attributes

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"on"` \| `"off"` -- present on each item. |
| `[data-disabled]` | Present on group when all items are disabled, or on individual disabled items. |

## Accessibility

Uses `role="radio"` with `aria-checked` for single mode, and `role="button"` with `aria-pressed` for multiple mode.

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Toggles the focused item. |
| `Enter` | Toggles the focused item. |
| `Tab` | Moves focus to the next focusable element. |
