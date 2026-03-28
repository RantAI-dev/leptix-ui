# Navigation Menu

A collection of links for navigating websites, with support for expandable sub-menus and an animated indicator.

## Features

- Can be controlled or uncontrolled
- Supports horizontal and vertical orientations
- Supports RTL direction
- Opens on hover or click with animated content transitions
- Optional active indicator

## Installation

```toml
[dependencies]
leptix-navigation-menu = "1.0.0"
```

## Anatomy

```rust
use leptix_navigation_menu::*;

view! {
    <NavigationMenu>
        <NavigationMenuList>
            <NavigationMenuItem>
                <NavigationMenuTrigger />
                <NavigationMenuContent>
                    <NavigationMenuLink />
                </NavigationMenuContent>
            </NavigationMenuItem>

            <NavigationMenuItem>
                <NavigationMenuLink />
            </NavigationMenuItem>

            <NavigationMenuIndicator />
        </NavigationMenuList>

        <NavigationMenuViewport />
    </NavigationMenu>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_navigation_menu::*;

#[component]
fn SiteNav() -> impl IntoView {
    view! {
        <NavigationMenu attr:class="nav-root">
            <NavigationMenuList attr:class="nav-list">
                <NavigationMenuItem>
                    <NavigationMenuTrigger attr:class="nav-trigger">
                        "Learn"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class="nav-content">
                        <ul style="display: grid; gap: 10px; padding: 20px; width: 400px;">
                            <li>
                                <NavigationMenuLink attr:href="/docs" attr:class="nav-link">
                                    "Documentation"
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:href="/tutorials" attr:class="nav-link">
                                    "Tutorials"
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem>
                    <NavigationMenuLink attr:href="/about" attr:class="nav-link" active=true>
                        "About"
                    </NavigationMenuLink>
                </NavigationMenuItem>

                <NavigationMenuIndicator attr:class="nav-indicator" />
            </NavigationMenuList>

            <NavigationMenuViewport attr:class="nav-viewport" />
        </NavigationMenu>
    }
}
```

## Styling

```css
.nav-root {
    position: relative;
    display: flex;
    justify-content: center;
}

.nav-list {
    display: flex;
    list-style: none;
    padding: 4px;
    margin: 0;
    border-radius: 6px;
    background: white;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.nav-trigger {
    padding: 8px 12px;
    outline: none;
    border: none;
    background: transparent;
    border-radius: 4px;
    font-size: 15px;
    cursor: default;
}

.nav-trigger[data-state="open"] {
    background-color: #f0f0f0;
}

.nav-content {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
}

.nav-content[data-state="open"] {
    animation: fadeIn 200ms ease;
}

.nav-content[data-state="closed"] {
    animation: fadeOut 150ms ease;
}

.nav-link {
    text-decoration: none;
    color: #333;
    display: block;
    padding: 12px;
    border-radius: 6px;
}

.nav-link:hover {
    background-color: #f5f5f5;
}

.nav-link[data-active] {
    color: #4c9aff;
}

.nav-indicator {
    display: flex;
    align-items: flex-end;
    justify-content: center;
    height: 10px;
    top: 100%;
    overflow: hidden;
    transition: width, transform 250ms ease;
}

.nav-indicator[data-state="visible"] {
    animation: fadeIn 200ms ease;
}

.nav-indicator[data-state="hidden"] {
    animation: fadeOut 200ms ease;
}

.nav-viewport {
    position: relative;
    margin-top: 10px;
    width: 100%;
    background: white;
    border-radius: 6px;
    overflow: hidden;
    box-shadow: 0 10px 38px rgba(0, 0, 0, 0.35), 0 10px 20px rgba(0, 0, 0, 0.2);
    height: var(--leptix-navigation-menu-viewport-height);
    transition: width, height 300ms ease;
}

.nav-viewport[data-state="open"] {
    animation: scaleIn 200ms ease;
}

.nav-viewport[data-state="closed"] {
    animation: scaleOut 200ms ease;
}
```

## API Reference

### NavigationMenu

The root component. Renders a `<nav>` element.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `MaybeProp<String>` | - | The controlled value of the active menu item. |
| `default_value` | `MaybeProp<String>` | - | The value when initially rendered. Use when you do not need to control the state. |
| `on_value_change` | `Option<Callback<String>>` | - | Event handler called when the value changes. |
| `orientation` | `MaybeProp<String>` | `"horizontal"` | The orientation of the menu. |
| `dir` | `MaybeProp<Direction>` | `"ltr"` | The reading direction. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<nav>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

### NavigationMenuList

Contains the top-level menu items. Renders a `<ul>`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

### NavigationMenuItem

A top-level menu item. Renders a `<li>`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Option<String>` | auto-generated | A unique value that identifies this item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### NavigationMenuTrigger

The button that toggles the content for a menu item.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### NavigationMenuContent

Contains the content associated with each trigger.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### NavigationMenuLink

A navigational link. Renders an `<a>`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `active` | `MaybeProp<bool>` | `false` | Whether the link represents the currently active page. |
| `on_select` | `Option<Callback<()>>` | - | Event handler called when the link is selected. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-active]` | Present when `active` is `true`. |

### NavigationMenuViewport

An optional viewport element that contains the active content outside of the list.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

### NavigationMenuSub

A submenu navigation component. Shares the same context structure as the root.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `MaybeProp<String>` | - | The controlled value of the active sub-item. |
| `default_value` | `MaybeProp<String>` | - | The value when initially rendered. |
| `on_value_change` | `Option<Callback<String>>` | - | Event handler called when the value changes. |
| `orientation` | `MaybeProp<String>` | `"horizontal"` | The orientation of the sub-menu. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

### NavigationMenuIndicator

An optional indicator element that highlights the active trigger.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"visible"` \| `"hidden"` |
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

## Accessibility

Adheres to the [Navigation WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/disclosure/examples/disclosure-navigation).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | When focus is on a trigger, toggles the content. |
| `Enter` | When focus is on a trigger, toggles the content. When focus is on a link, navigates. |
| `Tab` | Moves focus to the next focusable element. |
| `ArrowDown` | When orientation is vertical, moves focus to the next item. |
| `ArrowUp` | When orientation is vertical, moves focus to the previous item. |
| `ArrowRight` | When orientation is horizontal, moves focus to the next item. |
| `ArrowLeft` | When orientation is horizontal, moves focus to the previous item. |
