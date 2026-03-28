# Dropdown Menu

Displays a menu to the user, triggered by a button, with support for submenus, checkbox items, and radio groups.

## Features

- Can be controlled or uncontrolled
- Supports submenus with configurable nesting
- Supports checkbox and radio items
- Full keyboard navigation
- Dismisses on escape or click outside

## Installation

```toml
[dependencies]
leptix-dropdown-menu = "1.0.0"
```

## Anatomy

```rust
use leptix_dropdown_menu::*;

view! {
    <DropdownMenu>
        <DropdownMenuTrigger />
        <DropdownMenuPortal>
            <DropdownMenuContent>
                <DropdownMenuLabel />
                <DropdownMenuItem />

                <DropdownMenuGroup>
                    <DropdownMenuItem />
                </DropdownMenuGroup>

                <DropdownMenuCheckboxItem>
                    <DropdownMenuItemIndicator />
                </DropdownMenuCheckboxItem>

                <DropdownMenuRadioGroup>
                    <DropdownMenuRadioItem>
                        <DropdownMenuItemIndicator />
                    </DropdownMenuRadioItem>
                </DropdownMenuRadioGroup>

                <DropdownMenuSub>
                    <DropdownMenuSubTrigger />
                    <DropdownMenuSubContent />
                </DropdownMenuSub>

                <DropdownMenuSeparator />
                <DropdownMenuArrow />
            </DropdownMenuContent>
        </DropdownMenuPortal>
    </DropdownMenu>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_dropdown_menu::*;

#[component]
fn OptionsMenu() -> impl IntoView {
    let (bookmarks_checked, set_bookmarks_checked) = signal(true);
    let (person, set_person) = signal("pedro".to_string());

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger attr:class="menu-trigger">
                "Options"
            </DropdownMenuTrigger>
            <DropdownMenuPortal>
                <DropdownMenuContent attr:class="menu-content">
                    <DropdownMenuLabel>"Preferences"</DropdownMenuLabel>
                    <DropdownMenuSeparator />

                    <DropdownMenuItem on_select=Callback::new(|_| {})>
                        "New Tab"
                    </DropdownMenuItem>
                    <DropdownMenuItem disabled=true>
                        "New Private Window"
                    </DropdownMenuItem>

                    <DropdownMenuSeparator />

                    <DropdownMenuCheckboxItem
                        checked=bookmarks_checked
                        on_checked_change=Callback::new(move |v: bool| set_bookmarks_checked.set(v))
                    >
                        <DropdownMenuItemIndicator>"✓"</DropdownMenuItemIndicator>
                        "Show Bookmarks"
                    </DropdownMenuCheckboxItem>

                    <DropdownMenuSeparator />

                    <DropdownMenuRadioGroup
                        value=person
                        on_value_change=Callback::new(move |v: String| set_person.set(v))
                    >
                        <DropdownMenuRadioItem value="pedro">
                            <DropdownMenuItemIndicator>"●"</DropdownMenuItemIndicator>
                            "Pedro"
                        </DropdownMenuRadioItem>
                        <DropdownMenuRadioItem value="colm">
                            <DropdownMenuItemIndicator>"●"</DropdownMenuItemIndicator>
                            "Colm"
                        </DropdownMenuRadioItem>
                    </DropdownMenuRadioGroup>
                </DropdownMenuContent>
            </DropdownMenuPortal>
        </DropdownMenu>
    }
}
```

## Styling

```css
.menu-content {
    min-width: 220px;
    background: white;
    border-radius: 6px;
    padding: 5px;
    box-shadow: 0 10px 38px -10px rgba(22, 23, 24, 0.35);
}

.menu-content[data-state="open"] {
    animation: scaleIn 100ms ease-out;
}

.menu-content[data-state="closed"] {
    animation: scaleOut 100ms ease-in;
}

[role="menuitem"][data-disabled] {
    opacity: 0.5;
    pointer-events: none;
}

[role="menuitemcheckbox"][data-state="checked"],
[role="menuitemradio"][data-state="checked"] {
    font-weight: bold;
}
```

## API Reference

### DropdownMenu

Contains all the parts of a dropdown menu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `MaybeProp<bool>` | - | The controlled open state. |
| `default_open` | `MaybeProp<bool>` | `false` | The open state when initially rendered. |
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |
| `dir` | `MaybeProp<Direction>` | - | The reading direction. |

### DropdownMenuTrigger

The button that toggles the dropdown menu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### DropdownMenuPortal

When used, portals the content part into the `body`.

### DropdownMenuContent

The component that pops out when the dropdown menu is open.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `on_escape_key_down` | `Option<Callback<KeyboardEvent>>` | - | Event handler called when the escape key is down. Can be prevented. |
| `on_pointer_down_outside` | `Option<Callback<PointerEvent>>` | - | Event handler called when a pointer event happens outside. Can be prevented. |
| `loop` | `MaybeProp<bool>` | `false` | When `true`, keyboard navigation will loop from last to first item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### DropdownMenuGroup

Used to group multiple menu items.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### DropdownMenuLabel

Used to render a label. It will not be given focus when navigating with the keyboard.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### DropdownMenuItem

The component that contains the dropdown menu items.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `on_select` | `Option<Callback<()>>` | - | Event handler called when the user selects an item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-disabled]` | Present when disabled |

### DropdownMenuCheckboxItem

An item that can be controlled and rendered like a checkbox.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `MaybeProp<bool>` | `false` | The controlled checked state. |
| `on_checked_change` | `Option<Callback<bool>>` | - | Event handler called when the checked state changes. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `on_select` | `Option<Callback<()>>` | - | Event handler called when the user selects the item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` |
| `[data-disabled]` | Present when disabled |

### DropdownMenuRadioGroup

Used to group multiple radio items.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `MaybeProp<String>` | - | The value of the selected item. |
| `on_value_change` | `Option<Callback<String>>` | - | Event handler called when the value changes. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### DropdownMenuRadioItem

An item that can be controlled and rendered like a radio button. Must be used within a `DropdownMenuRadioGroup`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | The unique value of the item. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `on_select` | `Option<Callback<()>>` | - | Event handler called when the user selects the item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` |
| `[data-disabled]` | Present when disabled |

### DropdownMenuItemIndicator

Renders when the parent `DropdownMenuCheckboxItem` or `DropdownMenuRadioItem` is checked.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `force_mount` | `MaybeProp<bool>` | `false` | Used to force mounting when more control is needed. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` |

### DropdownMenuSeparator

Used to visually separate items in the dropdown menu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### DropdownMenuArrow

An optional arrow element to render alongside the dropdown menu content.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### DropdownMenuSub

Contains all the parts of a submenu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `MaybeProp<bool>` | - | The controlled open state. |
| `default_open` | `MaybeProp<bool>` | `false` | The open state when initially rendered. |
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |

### DropdownMenuSubTrigger

An item that opens a submenu.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when disabled |

### DropdownMenuSubContent

The component that pops out when a submenu is open.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `force_mount` | `MaybeProp<bool>` | `false` | Used to force mounting when more control is needed. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

## Accessibility

Adheres to the [Menu WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/menu).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | When focus is on the trigger, opens the menu. When focus is on an item, activates the focused item. |
| `Enter` | When focus is on the trigger, opens the menu. When focus is on an item, activates the focused item. |
| `ArrowDown` | When focus is on the trigger, opens the menu. Moves focus to the next item. |
| `ArrowUp` | Moves focus to the previous item. |
| `ArrowRight` | When focus is on a sub-trigger, opens the submenu. |
| `ArrowLeft` | When focus is inside a submenu, closes the submenu. |
| `Esc` | Closes the dropdown menu. |
