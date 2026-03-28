# Toast

A succinct message that is displayed temporarily, providing feedback on an action.

## Features

- Can be controlled or uncontrolled
- Configurable auto-dismiss duration
- Supports title, description, action, and close button parts
- Managed via a provider for multiple toasts
- Presence-aware with open/close animations

## Installation

```toml
[dependencies]
leptix-toast = "1.0.0"
```

## Anatomy

```rust
use leptix_toast::*;

view! {
    <ToastProvider>
        // app content
        <Toast>
            <ToastTitle />
            <ToastDescription />
            <ToastAction alt_text="..." />
            <ToastClose />
        </Toast>
        <ToastViewport />
    </ToastProvider>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_toast::*;

#[component]
fn ToastDemo() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <ToastProvider>
            <button on:click=move |_| set_open.set(true)>"Add to calendar"</button>

            <Toast
                open=open
                on_open_change=Callback::new(move |value: bool| set_open.set(value))
                attr:class="toast-root"
            >
                <ToastTitle attr:class="toast-title">"Scheduled: Catch up"</ToastTitle>
                <ToastDescription attr:class="toast-description">
                    "Friday, February 10, 2025 at 5:57 PM"
                </ToastDescription>
                <ToastAction alt_text="Go to schedule to undo" attr:class="toast-action">
                    "Undo"
                </ToastAction>
                <ToastClose attr:class="toast-close">"x"</ToastClose>
            </Toast>

            <ToastViewport attr:class="toast-viewport" />
        </ToastProvider>
    }
}
```

## Styling

```css
.toast-viewport {
    position: fixed;
    bottom: 0;
    right: 0;
    display: flex;
    flex-direction: column;
    padding: 25px;
    gap: 10px;
    width: 390px;
    max-width: 100vw;
    margin: 0;
    list-style: none;
    z-index: 2147483647;
    outline: none;
}

.toast-root {
    background: white;
    border-radius: 6px;
    box-shadow: 0 10px 38px rgba(0, 0, 0, 0.35), 0 10px 20px rgba(0, 0, 0, 0.2);
    padding: 15px;
    display: grid;
    grid-template-areas: "title action" "description action";
    grid-template-columns: auto max-content;
    column-gap: 15px;
    align-items: center;
}

.toast-root[data-state="open"] {
    animation: slideIn 150ms cubic-bezier(0.16, 1, 0.3, 1);
}

.toast-root[data-state="closed"] {
    animation: hide 100ms ease-in;
}

.toast-title {
    grid-area: title;
    font-weight: 500;
    color: #111;
    font-size: 15px;
}

.toast-description {
    grid-area: description;
    margin: 0;
    color: #666;
    font-size: 13px;
    line-height: 1.3;
}

.toast-action {
    grid-area: action;
}

.toast-close {
    position: absolute;
    top: 4px;
    right: 4px;
}
```

## API Reference

### ToastProvider

The provider that manages toast state. Wrap your application with this component.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `duration` | `MaybeProp<i32>` | `5000` | The default auto-dismiss duration in milliseconds for all toasts. |
| `swipe_direction` | `MaybeProp<String>` | `"right"` | The swipe direction to dismiss: `"right"`, `"left"`, `"up"`, or `"down"`. |

### Toast

An individual toast notification. Renders a `<li>` with `role="status"`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `MaybeProp<bool>` | - | The controlled open state. |
| `default_open` | `MaybeProp<bool>` | `true` | The open state when initially rendered. |
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |
| `duration` | `MaybeProp<i32>` | `5000` | Override the provider's auto-dismiss duration for this toast. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### ToastTitle

An optional title for the toast.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ToastDescription

The toast message content.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ToastAction

An action button that provides an alternative way to dismiss. Requires `alt_text` for accessibility when the action is destructive or important.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `alt_text` | `String` | *required* | An accessible description of the action for screen readers. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ToastClose

A button that closes the toast.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ToastViewport

The viewport area where toasts are rendered. Renders an `<ol>` with `role="region"`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `label` | `MaybeProp<String>` | `"Notifications"` | An accessible label for the toast region. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Data Attributes

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` -- present on each toast. |

## Accessibility

Adheres to the [Alert WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/alert/).

Toasts use `role="status"` and `aria-live="off"` to allow screen reader users to navigate to them without being interrupted. The viewport uses `role="region"` with a configurable `aria-label`. The action button has an `aria-label` from `alt_text`.

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Tab` | Moves focus to the toast and between its focusable elements. |
| `Space` | Activates the focused action or close button. |
| `Enter` | Activates the focused action or close button. |
| `Esc` | Closes the focused toast (when supported). |
