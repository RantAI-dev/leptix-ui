# Context Menu

Displays a menu located at the pointer, triggered by a right-click or a long-press.

## Features

- Triggered by right-click (contextmenu event)
- Supports submenus, checkbox items, and radio groups
- Arrow key navigation between items
- Dismisses on outside click or Escape

## Installation

```toml
[dependencies]
leptix-context-menu = "1.0.0"
```

## Anatomy

```rust
use leptix_context_menu::*;

view! {
    <ContextMenu>
        <ContextMenuTrigger />
        <ContextMenuPortal>
            <ContextMenuContent>
                <ContextMenuLabel />
                <ContextMenuItem />
                <ContextMenuGroup>
                    <ContextMenuItem />
                </ContextMenuGroup>
                <ContextMenuCheckboxItem>
                    <ContextMenuItemIndicator />
                </ContextMenuCheckboxItem>
                <ContextMenuRadioGroup>
                    <ContextMenuRadioItem>
                        <ContextMenuItemIndicator />
                    </ContextMenuRadioItem>
                </ContextMenuRadioGroup>
                <ContextMenuSub>
                    <ContextMenuSubTrigger />
                    <ContextMenuSubContent />
                </ContextMenuSub>
                <ContextMenuSeparator />
                <ContextMenuArrow />
            </ContextMenuContent>
        </ContextMenuPortal>
    </ContextMenu>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_context_menu::*;

#[component]
fn Demo() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger attr:class="context-trigger">
                "Right click here"
            </ContextMenuTrigger>
            <ContextMenuPortal>
                <ContextMenuContent attr:class="context-content">
                    <ContextMenuItem attr:class="context-item">"Back"</ContextMenuItem>
                    <ContextMenuItem attr:class="context-item">"Forward"</ContextMenuItem>
                    <ContextMenuItem attr:class="context-item">"Reload"</ContextMenuItem>
                    <ContextMenuSeparator attr:class="context-separator" />
                    <ContextMenuItem attr:class="context-item">"View Source"</ContextMenuItem>
                    <ContextMenuItem attr:class="context-item">"Inspect"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenuPortal>
        </ContextMenu>
    }
}
```

## Styling

```css
.context-trigger {
    display: block;
    border: 2px dashed #ccc;
    border-radius: 4px;
    padding: 45px 0;
    text-align: center;
    color: #888;
}

.context-content {
    min-width: 220px;
    background: white;
    border-radius: 6px;
    padding: 5px;
    box-shadow: 0 10px 38px rgba(0, 0, 0, 0.35), 0 10px 20px rgba(0, 0, 0, 0.2);
}

.context-content[data-state="open"] {
    animation: fadeIn 150ms ease;
}

.context-content[data-state="closed"] {
    animation: fadeOut 100ms ease;
}

.context-item {
    font-size: 13px;
    line-height: 1;
    border-radius: 3px;
    padding: 5px 10px;
    cursor: default;
    outline: none;
}

.context-item:focus {
    background-color: #4c9aff;
    color: white;
}

.context-item[data-disabled] {
    color: #aaa;
    pointer-events: none;
}

.context-separator {
    height: 1px;
    background-color: #e5e5e5;
    margin: 5px;
}
```

## API Reference

### ContextMenu

Contains all the parts of a context menu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |

### ContextMenuTrigger

The area that opens the context menu on right-click.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, the trigger is disabled and context menu will not open. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<span>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when disabled. |

### ContextMenuPortal

When used, portals the content part into the `body`.

### ContextMenuContent

The component that pops out when the context menu is open.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### ContextMenuItem

An item in the context menu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `on_select` | `Option<Callback<()>>` | - | Event handler called when the user selects an item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-disabled]` | Present when disabled. |

### ContextMenuCheckboxItem

An item that can be controlled and rendered like a checkbox.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `MaybeProp<bool>` | - | The controlled checked state. |
| `on_checked_change` | `Option<Callback<bool>>` | - | Event handler called when the checked state changes. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` |
| `[data-disabled]` | Present when disabled. |

### ContextMenuRadioGroup

Groups multiple radio items.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `MaybeProp<String>` | - | The value of the selected item. |
| `on_value_change` | `Option<Callback<String>>` | - | Event handler called when the value changes. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ContextMenuRadioItem

An item that can be controlled and rendered like a radio.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | The unique value of the radio item. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` |
| `[data-disabled]` | Present when disabled. |

### ContextMenuItemIndicator

Renders when the parent `ContextMenuCheckboxItem` or `ContextMenuRadioItem` is checked.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ContextMenuGroup

Groups multiple items.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ContextMenuLabel

Used to render a label. It won't be focusable using arrow keys.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ContextMenuSeparator

Used to visually separate items in the context menu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ContextMenuSub

Contains all the parts of a submenu.

### ContextMenuSubTrigger

An item that opens a submenu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when disabled. |

### ContextMenuSubContent

The component that pops out when a submenu is open.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### ContextMenuArrow

An optional arrow element to render alongside the content.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `width` | `MaybeProp<f64>` | `10.0` | The width of the arrow in pixels. |
| `height` | `MaybeProp<f64>` | `5.0` | The height of the arrow in pixels. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Accessibility

Adheres to the [Menu WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/menu).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Activates the focused item. |
| `Enter` | Activates the focused item. |
| `ArrowDown` | Moves focus to the next item. |
| `ArrowUp` | Moves focus to the previous item. |
| `ArrowRight` | When focus is on a sub-trigger, opens the submenu. |
| `ArrowLeft` | When focus is in a submenu, closes the submenu. |
| `Esc` | Closes the context menu. |
