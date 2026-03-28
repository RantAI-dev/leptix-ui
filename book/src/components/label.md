# Label

Renders an accessible label associated with controls. Prevents text selection on double-click.

## Features

- Renders a native `<label>` element
- Prevents text selection when double-clicking
- Supports custom mouse-down handlers
- Works with any focusable form control

## Installation

```toml
[dependencies]
leptix-label = "1.0.0"
```

## Anatomy

```rust
use leptix_label::*;

view! {
    <Label />
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_label::*;

#[component]
fn LabelDemo() -> impl IntoView {
    view! {
        <div style="display: flex; flex-wrap: wrap; gap: 15px; align-items: center;">
            <Label attr:r#for="name" attr:class="label">"Name"</Label>
            <input id="name" type="text" attr:class="input" />
        </div>
    }
}
```

## Styling

```css
.label {
    font-size: 15px;
    font-weight: 500;
    line-height: 35px;
    color: #111;
    user-select: none;
}
```

## API Reference

### Label

A text label associated with a form control.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `on_mouse_down` | `MaybeCallback<MouseEvent>` | - | Event handler called when the mouse button is pressed on the label. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<label>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Data Attributes

This component does not emit custom data attributes.

## Accessibility

This component renders a native `<label>` element. Use the `for` attribute (via `attr:r#for`) to associate it with a form control by `id`.

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| *N/A* | The label itself is not interactive. Clicking it moves focus to the associated control. |
