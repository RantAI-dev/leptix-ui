# Alert Dialog

A modal dialog that interrupts the user with important content and expects a response.

## Features

- Focus is automatically trapped within the dialog
- Can be controlled or uncontrolled
- Manages screen reader announcements with Title and Description
- Esc closes the dialog
- Clicking outside does not close the dialog (unlike Dialog)

## Installation

```toml
[dependencies]
leptix-alert-dialog = "1.0.0"
```

## Anatomy

```rust
use leptix_alert_dialog::*;

view! {
    <AlertDialog>
        <AlertDialogTrigger />
        <AlertDialogPortal>
            <AlertDialogOverlay />
            <AlertDialogContent>
                <AlertDialogTitle />
                <AlertDialogDescription />
                <AlertDialogCancel />
                <AlertDialogAction />
            </AlertDialogContent>
        </AlertDialogPortal>
    </AlertDialog>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_alert_dialog::*;

#[component]
fn DeleteConfirmation() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>"Delete account"</AlertDialogTrigger>
            <AlertDialogPortal>
                <AlertDialogOverlay attr:class="alert-overlay" />
                <AlertDialogContent attr:class="alert-content">
                    <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
                    <AlertDialogDescription>
                        "This action cannot be undone. This will permanently delete
                        your account and remove your data from our servers."
                    </AlertDialogDescription>

                    <div style="display: flex; gap: 16px; justify-content: flex-end;">
                        <AlertDialogCancel attr:class="alert-cancel">
                            "Cancel"
                        </AlertDialogCancel>
                        <AlertDialogAction attr:class="alert-action">
                            "Yes, delete account"
                        </AlertDialogAction>
                    </div>
                </AlertDialogContent>
            </AlertDialogPortal>
        </AlertDialog>
    }
}
```

## Styling

```css
.alert-overlay {
    background-color: rgba(0, 0, 0, 0.5);
    position: fixed;
    inset: 0;
    animation: overlayShow 150ms cubic-bezier(0.16, 1, 0.3, 1);
}

.alert-content {
    background: white;
    border-radius: 6px;
    padding: 25px;
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 90vw;
    max-width: 500px;
}

.alert-content[data-state="closed"] {
    animation: contentHide 100ms ease-in;
}

.alert-cancel {
    background-color: #e5e7eb;
}

.alert-action {
    background-color: #ef4444;
    color: white;
}
```

## API Reference

### AlertDialog

Contains all the parts of an alert dialog.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `MaybeProp<bool>` | - | The controlled open state. |
| `default_open` | `MaybeProp<bool>` | `false` | The open state when initially rendered. |
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |

### AlertDialogTrigger

A button that opens the dialog. Re-exported from `DialogTrigger`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### AlertDialogPortal

When used, portals your overlay and content parts into the `body`.

### AlertDialogOverlay

A layer that covers the inert portion of the view when the dialog is open. Re-exported from `DialogOverlay`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### AlertDialogContent

Contains content to be rendered when the dialog is open. Unlike `DialogContent`, clicking outside will not close the dialog.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `on_open_auto_focus` | `Option<Callback<Event>>` | - | Event handler called when focus moves into the component after opening. |
| `on_close_auto_focus` | `Option<Callback<Event>>` | - | Event handler called when focus moves to the trigger after closing. |
| `on_escape_key_down` | `Option<Callback<KeyboardEvent>>` | - | Event handler called when the escape key is down. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### AlertDialogCancel

A button that closes the dialog. This button should be distinguished visually from `AlertDialogAction` buttons.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### AlertDialogAction

A button that closes the dialog. These buttons should be distinguished visually from the `AlertDialogCancel` button.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### AlertDialogTitle

An accessible title announced when the dialog is opened. Re-exported from `DialogTitle`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### AlertDialogDescription

An optional accessible description announced when the dialog is opened. Re-exported from `DialogDescription`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Accessibility

Adheres to the [Alert Dialog WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/alertdialog).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Opens/closes the dialog when focus is on the trigger. |
| `Enter` | Opens/closes the dialog when focus is on the trigger. |
| `Tab` | Moves focus to the next focusable element within the dialog. |
| `Shift + Tab` | Moves focus to the previous focusable element within the dialog. |
| `Esc` | Closes the dialog and moves focus to the trigger. |
