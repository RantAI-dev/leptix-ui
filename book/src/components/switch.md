# Switch

A control that allows the user to toggle between checked and not checked.

## Features

- Can be controlled or uncontrolled
- Full keyboard navigation
- Automatically integrates with native HTML forms
- Labeling support for assistive technology

## Installation

```toml
[dependencies]
leptix-switch = "1.0.0"
```

## Anatomy

```rust
use leptix_switch::*;

view! {
    <Switch>
        <SwitchThumb />
    </Switch>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_switch::*;

#[component]
fn AirplaneMode() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; gap: 8px;">
            <Switch attr:class="switch-root" attr:id="airplane-mode">
                <SwitchThumb attr:class="switch-thumb" />
            </Switch>
            <label r#for="airplane-mode">"Airplane Mode"</label>
        </div>
    }
}
```

## Styling

```css
.switch-root {
    all: unset;
    width: 42px;
    height: 25px;
    background-color: #e5e7eb;
    border-radius: 9999px;
    position: relative;
    cursor: pointer;
    -webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

.switch-root[data-state="checked"] {
    background-color: #3b82f6;
}

.switch-root[data-disabled] {
    opacity: 0.5;
    cursor: not-allowed;
}

.switch-thumb {
    display: block;
    width: 21px;
    height: 21px;
    background-color: white;
    border-radius: 9999px;
    transition: transform 100ms;
    transform: translateX(2px);
    will-change: transform;
}

.switch-thumb[data-state="checked"] {
    transform: translateX(19px);
}
```

## API Reference

### Switch

The root switch control.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | `MaybeProp<String>` | - | The name of the switch. Submitted with its owning form as part of a name/value pair. |
| `checked` | `MaybeProp<bool>` | - | The controlled checked state. |
| `default_checked` | `MaybeProp<bool>` | `false` | The checked state when initially rendered. |
| `on_checked_change` | `Option<Callback<bool>>` | - | Event handler called when the checked state changes. |
| `required` | `MaybeProp<bool>` | `false` | When `true`, indicates that the user must check the switch before the form can be submitted. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the switch. |
| `value` | `MaybeProp<String>` | `"on"` | The value given as data when submitted with a `name`. |
| `on_click` | `Option<Callback<MouseEvent>>` | - | Event handler called on click. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` |
| `[data-disabled]` | Present when disabled |

### SwitchThumb

The thumb that slides to indicate the switch state.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` |
| `[data-disabled]` | Present when disabled |

## Accessibility

Adheres to the [Switch WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/switch).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Toggles the switch. |
| `Enter` | Toggles the switch. |
