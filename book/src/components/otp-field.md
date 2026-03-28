# OTP Field

A one-time password input with individual character segments, auto-advance, and validation.

## Features

- Configurable number of input segments via `max_length`
- Supports numeric, alphanumeric, or free-text validation
- Auto-advances focus to the next input on entry
- Backspace navigates to the previous input
- Fires `on_complete` when all segments are filled

## Installation

```toml
[dependencies]
leptix-otp-field = "1.0.0"
```

## Anatomy

```rust
use leptix_otp_field::*;

view! {
    <OneTimePasswordField>
        <OneTimePasswordFieldInput index=0 />
        <OneTimePasswordFieldInput index=1 />
        <OneTimePasswordFieldInput index=2 />
        <OneTimePasswordFieldInput index=3 />
        <OneTimePasswordFieldInput index=4 />
        <OneTimePasswordFieldInput index=5 />
        <OneTimePasswordFieldHiddenInput />
    </OneTimePasswordField>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_otp_field::*;

#[component]
fn VerificationCode() -> impl IntoView {
    view! {
        <OneTimePasswordField
            max_length=6
            on_complete=Callback::new(|code: String| {
                leptos::logging::log!("Code entered: {}", code);
            })
            attr:class="otp-root"
        >
            <OneTimePasswordFieldInput index=0 />
            <OneTimePasswordFieldInput index=1 />
            <OneTimePasswordFieldInput index=2 />
            <span style="padding: 0 4px;">"-"</span>
            <OneTimePasswordFieldInput index=3 />
            <OneTimePasswordFieldInput index=4 />
            <OneTimePasswordFieldInput index=5 />
            <OneTimePasswordFieldHiddenInput name="otp-code".into() />
        </OneTimePasswordField>
    }
}
```

## Styling

```css
.otp-root {
    display: flex;
    gap: 4px;
    align-items: center;
}

[data-leptix-otp-input] {
    width: 2.5em;
    height: 2.5em;
    text-align: center;
    font-size: 18px;
    border: 2px solid #ccc;
    border-radius: 6px;
    outline: none;
}

[data-leptix-otp-input]:focus {
    border-color: #4c9aff;
    box-shadow: 0 0 0 2px rgba(76, 154, 255, 0.3);
}

[data-leptix-otp-input][disabled] {
    opacity: 0.5;
    cursor: not-allowed;
}
```

## API Reference

### OneTimePasswordField

The root container for the OTP input segments.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `max_length` | `MaybeProp<usize>` | `6` | Number of input segments. |
| `default_value` | `MaybeProp<String>` | - | The initial value to populate segments with. |
| `on_value_change` | `Option<Callback<String>>` | - | Event handler called when the combined value changes. |
| `on_complete` | `Option<Callback<String>>` | - | Event handler called when all segments are filled. |
| `validation_type` | `Option<InputValidationType>` | `Numeric` | The type of input validation: `Numeric`, `Alphanumeric`, or `Text`. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, all inputs are disabled. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<div>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-disabled]` | Present when disabled. |

### OneTimePasswordFieldInput

An individual input segment. Renders an `<input>` with `maxlength="1"`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `index` | `Option<usize>` | `0` | The zero-based index of this input segment. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-leptix-otp-input]` | Always present. |

### OneTimePasswordFieldHiddenInput

A hidden `<input>` that holds the combined OTP value for form submission.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | `MaybeProp<String>` | - | The form field name for submission. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Data Attributes

| Data Attribute | Values |
|---------------|--------|
| `[data-disabled]` | Present on the root when disabled. |
| `[data-leptix-otp-input]` | Present on each input segment. |

## Accessibility

Each input segment is labelled with `aria-label="Digit N"` and uses `inputmode="numeric"` and `autocomplete="one-time-code"` for mobile keyboard hints.

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `0-9` / `a-z` | Enters a character and auto-advances to the next segment (depending on `validation_type`). |
| `Backspace` | Clears the current segment. If already empty, moves focus to the previous segment. |
| `Tab` | Moves focus to the next focusable element. |
| `Shift + Tab` | Moves focus to the previous focusable element. |
