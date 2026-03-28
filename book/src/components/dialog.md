# Dialog

A window overlaid on the primary window, rendering content in a layer above the page.

## Features

- Focus is automatically trapped within the dialog
- Can be controlled or uncontrolled
- Manages screen reader announcements with Title and Description
- Esc closes the dialog
- Click outside closes the dialog

## Installation

```toml
[dependencies]
leptix-dialog = "1.0.0"
```

## Anatomy

```rust
use leptix_dialog::*;

view! {
    <Dialog>
        <DialogTrigger />
        <DialogPortal>
            <DialogOverlay />
            <DialogContent>
                <DialogTitle />
                <DialogDescription />
                <DialogClose />
            </DialogContent>
        </DialogPortal>
    </Dialog>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_dialog::*;

#[component]
fn ProfileDialog() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"Edit Profile"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class="dialog-overlay" />
                <DialogContent attr:class="dialog-content">
                    <DialogTitle>"Edit profile"</DialogTitle>
                    <DialogDescription>
                        "Make changes to your profile here."
                    </DialogDescription>

                    <fieldset>
                        <label r#for="name">"Name"</label>
                        <input id="name" value="John Doe" />
                    </fieldset>

                    <DialogClose attr:class="dialog-close">"Save changes"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}
```

## Styling

```css
.dialog-overlay {
    background-color: rgba(0, 0, 0, 0.5);
    position: fixed;
    inset: 0;
    animation: overlayShow 150ms cubic-bezier(0.16, 1, 0.3, 1);
}

.dialog-content {
    background: white;
    border-radius: 6px;
    padding: 25px;
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 90vw;
    max-width: 450px;
    animation: contentShow 150ms cubic-bezier(0.16, 1, 0.3, 1);
}

.dialog-content[data-state="closed"] {
    animation: contentHide 100ms ease-in;
}
```

## API Reference

### Dialog

Contains all the parts of a dialog.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `MaybeProp<bool>` | - | The controlled open state. |
| `default_open` | `MaybeProp<bool>` | `false` | The open state when initially rendered. Use when you do not need to control the open state. |
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |
| `modal` | `Option<bool>` | `true` | When `true`, interaction with outside elements is disabled and only dialog content is visible to screen readers. |

### DialogTrigger

The button that opens the dialog.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<button>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### DialogPortal

When used, portals your overlay and content parts into the `body`.

### DialogOverlay

A layer that covers the inert portion of the view when the dialog is open.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### DialogContent

Contains content to be rendered in the open dialog.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `on_open_auto_focus` | `Option<Callback<Event>>` | - | Event handler called when focus moves into the component after opening. Can be prevented. |
| `on_close_auto_focus` | `Option<Callback<Event>>` | - | Event handler called when focus moves to the trigger after closing. Can be prevented. |
| `on_escape_key_down` | `Option<Callback<KeyboardEvent>>` | - | Event handler called when the escape key is down. Can be prevented. |
| `on_pointer_down_outside` | `Option<Callback<PointerEvent>>` | - | Event handler called when a pointer event happens outside of the component. Can be prevented. |
| `on_interact_outside` | `Option<Callback<Event>>` | - | Event handler called when an interaction happens outside the component. Can be prevented. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### DialogClose

The button that closes the dialog.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### DialogTitle

An accessible title announced when the dialog is opened.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### DialogDescription

An optional accessible description announced when the dialog is opened.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Accessibility

Adheres to the [Dialog WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Opens/closes the dialog when focus is on the trigger. |
| `Enter` | Opens/closes the dialog when focus is on the trigger. |
| `Tab` | Moves focus to the next focusable element within the dialog. |
| `Shift + Tab` | Moves focus to the previous focusable element within the dialog. |
| `Esc` | Closes the dialog and moves focus to the trigger. |
