# Form

A form component with built-in validation and accessible error messages.

## Features

- Composes native HTML form elements
- Built-in label, control, and message association via generated IDs
- Supports client-side and server-side validation
- Accessible error messages linked via `aria-describedby`

## Installation

```toml
[dependencies]
leptix-form = "1.0.0"
```

## Anatomy

```rust
use leptix_form::*;

view! {
    <Form>
        <FormField name="email">
            <FormLabel />
            <FormControl />
            <FormMessage />
            <FormValidityState />
        </FormField>
        <FormSubmit />
    </Form>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_form::*;

#[component]
fn SignupForm() -> impl IntoView {
    view! {
        <Form
            on_submit=Callback::new(|event: web_sys::Event| {
                event.prevent_default();
                // handle submission
            })
            attr:class="form-root"
        >
            <FormField name="email">
                <FormLabel>"Email"</FormLabel>
                <FormControl attr:type="email" attr:placeholder="you@example.com" />
                <FormMessage r#match="typeMismatch".into()>
                    "Please enter a valid email."
                </FormMessage>
            </FormField>

            <FormField name="password">
                <FormLabel>"Password"</FormLabel>
                <FormControl attr:type="password" attr:required=true />
                <FormMessage r#match="valueMissing".into()>
                    "Password is required."
                </FormMessage>
            </FormField>

            <FormSubmit>"Sign up"</FormSubmit>
        </Form>
    }
}
```

## Styling

```css
.form-root {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-width: 400px;
}

[role="alert"] {
    color: #dc3545;
    font-size: 13px;
    margin-top: 4px;
}
```

## API Reference

### Form

The root form element. Renders a `<form>`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `on_submit` | `Option<Callback<web_sys::Event>>` | - | Event handler called when the form is submitted. |
| `on_clear_server_errors` | `Option<Callback<()>>` | - | Event handler called to clear server-side errors. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<form>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### FormField

Groups a label, control, and messages for a single field. Generates a shared ID for association.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | `String` | *required* | The name of the field, used as the control's `name` attribute. |
| `server_invalid` | `MaybeProp<bool>` | - | When `true`, indicates a server-side validation error. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<div>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### FormLabel

A label automatically associated with the field control via `for`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<label>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### FormControl

The form input element. Renders an `<input>` with auto-generated `id`, `name`, and `aria-describedby`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<input>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### FormMessage

A validation message associated with the field control via `aria-describedby`. Renders with `role="alert"`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `r#match` | `MaybeProp<String>` | - | The type of validity state to match (e.g. `"valueMissing"`, `"typeMismatch"`). |
| `force_match` | `MaybeProp<bool>` | - | When `true`, forces the message to display regardless of validity. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<span>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### FormValidityState

Provides the current validity state of the field to its children via context.

Exposes `FormValidityStateContext` with:
- `valid: Signal<bool>` -- Whether the field is currently valid.
- `field_name: String` -- The name of the parent field.

### FormSubmit

The submit button. Renders a `<button type="submit">`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<button>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Data Attributes

| Data Attribute | Values |
|---------------|--------|
| `[role="alert"]` | Present on `FormMessage`. |

## Accessibility

Adheres to native HTML form accessibility patterns. Labels are associated with controls via `for`/`id`, and validation messages are linked to controls via `aria-describedby`.

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Tab` | Moves focus between form controls. |
| `Enter` | Submits the form when a submit button is focused. |
