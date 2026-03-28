# Checkbox

A control that allows the user to toggle between checked, unchecked, and indeterminate states.

## Features

- Supports indeterminate state
- Can be controlled or uncontrolled
- Full keyboard navigation
- Automatically integrates with native HTML forms

## Installation

```toml
[dependencies]
leptix-checkbox = "1.0.0"
```

## Anatomy

```rust
use leptix_checkbox::*;

view! {
    <Checkbox>
        <CheckboxIndicator />
    </Checkbox>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_checkbox::*;

#[component]
fn AcceptTerms() -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center; gap: 8px;">
            <Checkbox
                attr:class="checkbox-root"
                attr:id="terms"
                default_checked=CheckedState::False
            >
                <CheckboxIndicator attr:class="checkbox-indicator">
                    "✓"
                </CheckboxIndicator>
            </Checkbox>
            <label r#for="terms">"Accept terms and conditions"</label>
        </div>
    }
}
```

## Styling

```css
.checkbox-root {
    all: unset;
    width: 20px;
    height: 20px;
    border-radius: 4px;
    border: 2px solid #6b7280;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
}

.checkbox-root[data-state="checked"] {
    background-color: #3b82f6;
    border-color: #3b82f6;
}

.checkbox-root[data-state="indeterminate"] {
    background-color: #93c5fd;
    border-color: #93c5fd;
}

.checkbox-indicator {
    color: white;
    font-size: 12px;
}
```

## API Reference

### Checkbox

The root checkbox control.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | `MaybeProp<String>` | - | The name of the checkbox. Submitted with its owning form as part of a name/value pair. |
| `checked` | `MaybeProp<CheckedState>` | - | The controlled checked state. Can be `True`, `False`, or `Indeterminate`. |
| `default_checked` | `MaybeProp<CheckedState>` | `CheckedState::False` | The checked state when initially rendered. |
| `on_checked_change` | `Option<Callback<CheckedState>>` | - | Event handler called when the checked state changes. |
| `required` | `MaybeProp<bool>` | `false` | When `true`, indicates that the user must check the checkbox before the form can be submitted. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the checkbox. |
| `value` | `MaybeProp<String>` | `"on"` | The value given as data when submitted with a `name`. |
| `on_keydown` | `Option<Callback<KeyboardEvent>>` | - | Event handler called on key down. |
| `on_click` | `Option<Callback<MouseEvent>>` | - | Event handler called on click. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` \| `"indeterminate"` |
| `[data-disabled]` | Present when disabled |

### CheckboxIndicator

Renders when the checkbox is in a checked or indeterminate state. You can style this element directly, or use it as a wrapper to put an icon into.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `force_mount` | `MaybeProp<bool>` | `false` | Used to force mounting when more control is needed. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` \| `"indeterminate"` |
| `[data-disabled]` | Present when disabled |

## Accessibility

Adheres to the [Checkbox WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/checkbox).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Toggles the checkbox state. |
