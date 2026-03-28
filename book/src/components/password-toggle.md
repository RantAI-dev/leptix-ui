# Password Toggle

A password input field with a built-in visibility toggle button.

## Features

- Toggles input type between `"password"` and `"text"`
- Accessible toggle button with dynamic `aria-label`
- Supports an icon slot that reflects visibility state
- Can be uncontrolled with a default visibility

## Installation

```toml
[dependencies]
leptix-password-toggle = "1.0.0"
```

## Anatomy

```rust
use leptix_password_toggle::*;

view! {
    <PasswordToggleField>
        <PasswordToggleFieldInput />
        <PasswordToggleFieldToggle>
            <PasswordToggleFieldIcon />
        </PasswordToggleFieldToggle>
        <PasswordToggleFieldSlot />
    </PasswordToggleField>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_password_toggle::*;

#[component]
fn PasswordInput() -> impl IntoView {
    view! {
        <PasswordToggleField attr:class="password-root">
            <PasswordToggleFieldInput attr:placeholder="Enter password" attr:class="password-input" />
            <PasswordToggleFieldToggle attr:class="password-toggle">
                <PasswordToggleFieldIcon>
                    // Icon changes based on data-state
                </PasswordToggleFieldIcon>
            </PasswordToggleFieldToggle>
        </PasswordToggleField>
    }
}
```

## Styling

```css
.password-root {
    display: flex;
    align-items: center;
    border: 2px solid #ccc;
    border-radius: 6px;
    padding: 0 8px;
}

.password-root[data-state="visible"] {
    border-color: #4c9aff;
}

.password-input {
    flex: 1;
    border: none;
    outline: none;
    padding: 8px 0;
    font-size: 15px;
}

.password-toggle {
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px;
    color: #666;
}

.password-toggle[data-state="visible"] {
    color: #4c9aff;
}
```

## API Reference

### PasswordToggleField

The root container that provides visibility state to children.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `default_visible` | `MaybeProp<bool>` | `false` | The initial visibility state. When `true`, the password is shown as plain text. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<div>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"visible"` \| `"hidden"` |

### PasswordToggleFieldInput

The password input element. Renders an `<input>` whose `type` toggles between `"password"` and `"text"`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"visible"` \| `"hidden"` |

### PasswordToggleFieldToggle

The button that toggles password visibility.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"visible"` \| `"hidden"` |

### PasswordToggleFieldIcon

A decorative icon container that reflects the visibility state.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"visible"` \| `"hidden"` |

### PasswordToggleFieldSlot

A generic slot for additional content (e.g., strength meter).

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Data Attributes

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"visible"` \| `"hidden"` -- present on root, input, toggle, and icon. |

## Accessibility

The toggle button uses `aria-label` that dynamically changes between `"Show password"` and `"Hide password"`, and `aria-pressed` reflects the current visibility state.

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Toggles password visibility when focus is on the toggle button. |
| `Enter` | Toggles password visibility when focus is on the toggle button. |
| `Tab` | Moves focus between the input and the toggle button. |
