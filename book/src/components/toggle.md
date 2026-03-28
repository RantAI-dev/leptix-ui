# Toggle

A two-state button that can be either on or off.

## Features

- Can be controlled or uncontrolled
- Full keyboard navigation
- Accessible pressed state for assistive technology

## Installation

```toml
[dependencies]
leptix-toggle = "1.0.0"
```

## Anatomy

```rust
use leptix_toggle::*;

view! {
    <Toggle />
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_toggle::*;

#[component]
fn ItalicToggle() -> impl IntoView {
    view! {
        <Toggle attr:class="toggle" attr:aria-label="Toggle italic">
            "I"
        </Toggle>
    }
}

#[component]
fn ControlledToggle() -> impl IntoView {
    let (pressed, set_pressed) = signal(false);

    view! {
        <Toggle
            pressed=pressed
            on_pressed_change=Callback::new(move |v: bool| set_pressed.set(v))
            attr:class="toggle"
        >
            {move || if pressed.get() { "ON" } else { "OFF" }}
        </Toggle>
    }
}
```

## Styling

```css
.toggle {
    all: unset;
    padding: 8px 12px;
    border-radius: 4px;
    background-color: white;
    border: 1px solid #d1d5db;
    cursor: pointer;
    font-size: 14px;
}

.toggle[data-state="on"] {
    background-color: #dbeafe;
    border-color: #3b82f6;
    color: #1d4ed8;
}

.toggle[data-disabled] {
    opacity: 0.5;
    cursor: not-allowed;
}

.toggle:hover:not([data-disabled]) {
    background-color: #f3f4f6;
}

.toggle[data-state="on"]:hover:not([data-disabled]) {
    background-color: #bfdbfe;
}
```

## API Reference

### Toggle

The toggle button.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `pressed` | `MaybeProp<bool>` | - | The controlled state of the toggle. |
| `default_pressed` | `MaybeProp<bool>` | `false` | The state of the toggle when initially rendered. Use when you do not need to control the state. |
| `on_pressed_change` | `Option<Callback<bool>>` | - | Event handler called when the state of the toggle changes. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the toggle. |
| `on_click` | `Option<Callback<MouseEvent>>` | - | Event handler called on click. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"on"` \| `"off"` |
| `[data-disabled]` | Present when disabled |

## Accessibility

Adheres to the [Toggle Button WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/button).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Toggles the pressed state. |
| `Enter` | Toggles the pressed state. |
