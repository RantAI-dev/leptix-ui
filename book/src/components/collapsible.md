# Collapsible

An interactive component which expands and collapses a panel.

## Features

- Can be controlled or uncontrolled
- Supports force-mounting content for animation control
- Accessible open/closed state for assistive technology

## Installation

```toml
[dependencies]
leptix-collapsible = "1.0.0"
```

## Anatomy

```rust
use leptix_collapsible::*;

view! {
    <Collapsible>
        <CollapsibleTrigger />
        <CollapsibleContent />
    </Collapsible>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_collapsible::*;

#[component]
fn RepoList() -> impl IntoView {
    view! {
        <Collapsible attr:class="collapsible-root">
            <div style="display: flex; align-items: center; justify-content: space-between;">
                <span>"@user starred 3 repositories"</span>
                <CollapsibleTrigger attr:class="collapsible-trigger">
                    "Toggle"
                </CollapsibleTrigger>
            </div>

            <div attr:class="repo-item">"repo/one"</div>

            <CollapsibleContent attr:class="collapsible-content">
                <div attr:class="repo-item">"repo/two"</div>
                <div attr:class="repo-item">"repo/three"</div>
            </CollapsibleContent>
        </Collapsible>
    }
}
```

## Styling

```css
.collapsible-root {
    width: 300px;
}

.collapsible-trigger {
    all: unset;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
}

.collapsible-content[data-state="open"] {
    animation: slideDown 200ms ease-out;
}

.collapsible-content[data-state="closed"] {
    animation: slideUp 200ms ease-out;
}

@keyframes slideDown {
    from { height: 0; opacity: 0; }
    to { height: var(--leptix-collapsible-content-height); opacity: 1; }
}

@keyframes slideUp {
    from { height: var(--leptix-collapsible-content-height); opacity: 1; }
    to { height: 0; opacity: 0; }
}
```

## API Reference

### Collapsible

Contains all the parts of a collapsible.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `MaybeProp<bool>` | - | The controlled open state. |
| `default_open` | `MaybeProp<bool>` | `false` | The open state when initially rendered. |
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the collapsible. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when disabled |

### CollapsibleTrigger

The button that toggles the collapsible.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when disabled |

### CollapsibleContent

The component that contains the collapsible content.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `force_mount` | `MaybeProp<bool>` | `false` | Used to force mounting when more control is needed. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when disabled |

## Accessibility

Adheres to the [Disclosure WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/disclosure).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Opens/closes the collapsible when focus is on the trigger. |
| `Enter` | Opens/closes the collapsible when focus is on the trigger. |
