# Menubar

A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands.

## Features

- Can contain submenus, checkbox items, and radio groups
- Arrow key navigation between menus and items
- Supports RTL direction
- Only one menu open at a time

## Installation

```toml
[dependencies]
leptix-menubar = "1.0.0"
```

## Anatomy

```rust
use leptix_menubar::*;

view! {
    <Menubar>
        <MenubarMenu>
            <MenubarTrigger />
            <MenubarPortal>
                <MenubarContent>
                    <MenubarLabel />
                    <MenubarItem />
                    <MenubarGroup>
                        <MenubarItem />
                    </MenubarGroup>
                    <MenubarCheckboxItem>
                        <MenubarItemIndicator />
                    </MenubarCheckboxItem>
                    <MenubarRadioGroup>
                        <MenubarRadioItem>
                            <MenubarItemIndicator />
                        </MenubarRadioItem>
                    </MenubarRadioGroup>
                    <MenubarSub>
                        <MenubarSubTrigger />
                        <MenubarSubContent />
                    </MenubarSub>
                    <MenubarSeparator />
                    <MenubarArrow />
                </MenubarContent>
            </MenubarPortal>
        </MenubarMenu>
    </Menubar>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_menubar::*;

#[component]
fn AppMenubar() -> impl IntoView {
    view! {
        <Menubar attr:class="menubar-root">
            <MenubarMenu>
                <MenubarTrigger attr:class="menubar-trigger">"File"</MenubarTrigger>
                <MenubarPortal>
                    <MenubarContent attr:class="menubar-content">
                        <MenubarItem attr:class="menubar-item">"New Tab"</MenubarItem>
                        <MenubarItem attr:class="menubar-item">"New Window"</MenubarItem>
                        <MenubarSeparator attr:class="menubar-separator" />
                        <MenubarItem attr:class="menubar-item">"Print"</MenubarItem>
                    </MenubarContent>
                </MenubarPortal>
            </MenubarMenu>

            <MenubarMenu>
                <MenubarTrigger attr:class="menubar-trigger">"Edit"</MenubarTrigger>
                <MenubarPortal>
                    <MenubarContent attr:class="menubar-content">
                        <MenubarItem attr:class="menubar-item">"Undo"</MenubarItem>
                        <MenubarItem attr:class="menubar-item">"Redo"</MenubarItem>
                        <MenubarSeparator attr:class="menubar-separator" />
                        <MenubarItem attr:class="menubar-item">"Cut"</MenubarItem>
                        <MenubarItem attr:class="menubar-item">"Copy"</MenubarItem>
                        <MenubarItem attr:class="menubar-item">"Paste"</MenubarItem>
                    </MenubarContent>
                </MenubarPortal>
            </MenubarMenu>
        </Menubar>
    }
}
```

## Styling

```css
.menubar-root {
    display: flex;
    background: white;
    padding: 3px;
    border-radius: 6px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.menubar-trigger {
    padding: 8px 12px;
    outline: none;
    font-size: 13px;
    line-height: 1;
    border-radius: 4px;
    color: #333;
    border: none;
    background: transparent;
    cursor: default;
}

.menubar-trigger[data-state="open"],
.menubar-trigger:hover {
    background-color: #f0f0f0;
}

.menubar-content {
    min-width: 220px;
    background: white;
    border-radius: 6px;
    padding: 5px;
    box-shadow: 0 10px 38px rgba(0, 0, 0, 0.35), 0 10px 20px rgba(0, 0, 0, 0.2);
}

.menubar-content[data-state="open"] {
    animation: fadeIn 150ms ease;
}

.menubar-item {
    font-size: 13px;
    line-height: 1;
    border-radius: 4px;
    padding: 5px 10px;
    cursor: default;
    outline: none;
}

.menubar-item:focus {
    background-color: #4c9aff;
    color: white;
}

.menubar-separator {
    height: 1px;
    background-color: #e5e5e5;
    margin: 5px;
}
```

## API Reference

### Menubar

The root component. Renders a `<div>` with `role="menubar"`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `dir` | `MaybeProp<Direction>` | `"ltr"` | The reading direction. |
| `r#loop` | `MaybeProp<bool>` | `true` | When `true`, keyboard navigation will loop from last to first and vice versa. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<div>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### MenubarMenu

Contains all the parts of a single menu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<String>` | auto-generated | A unique value that associates the menu with a trigger. |

### MenubarTrigger

The button that toggles the menu. Renders a `<button>` with `role="menuitem"`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### MenubarPortal

When used, portals the content part into the `body`.

### MenubarContent

The component that pops out when a menu is open. Renders with `role="menu"`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### MenubarItem

An item in the menu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `on_select` | `Option<Callback<()>>` | - | Event handler called when the user selects an item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-disabled]` | Present when disabled. |

### MenubarCheckboxItem

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

### MenubarRadioGroup

Groups multiple radio items.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `MaybeProp<String>` | - | The value of the selected item. |
| `on_value_change` | `Option<Callback<String>>` | - | Event handler called when the value changes. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### MenubarRadioItem

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

### MenubarItemIndicator

Renders when the parent checkbox or radio item is checked.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### MenubarGroup

Groups multiple items.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### MenubarLabel

Used to render a label. It won't be focusable using arrow keys.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### MenubarSeparator

Used to visually separate items in the menu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### MenubarSub

Contains all the parts of a submenu.

### MenubarSubTrigger

An item that opens a submenu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### MenubarSubContent

The component that pops out when a submenu is open.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### MenubarArrow

An optional arrow element to render alongside the content.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `width` | `MaybeProp<f64>` | `10.0` | The width of the arrow in pixels. |
| `height` | `MaybeProp<f64>` | `5.0` | The height of the arrow in pixels. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Accessibility

Adheres to the [Menu Bar WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/menubar).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | Activates the focused item or opens/closes a menu trigger. |
| `Enter` | Activates the focused item or opens/closes a menu trigger. |
| `ArrowDown` | Opens the menu when focus is on a trigger. Moves focus to the next item in an open menu. |
| `ArrowUp` | Moves focus to the previous item in an open menu. |
| `ArrowRight` | Moves focus to the next menubar trigger. Opens a submenu when focus is on a sub-trigger. |
| `ArrowLeft` | Moves focus to the previous menubar trigger. Closes a submenu when focus is in a submenu. |
| `Esc` | Closes the currently open menu and returns focus to its trigger. |
