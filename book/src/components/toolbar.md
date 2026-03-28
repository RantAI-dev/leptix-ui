# Toolbar

A container for grouping a set of controls, such as buttons, toggle groups, or dropdown menus.

## Features

- Full arrow key navigation with optional looping
- Supports horizontal and vertical orientations
- Supports RTL direction
- Contains buttons, links, separators, and toggle groups
- Home/End keys jump to first/last item

## Installation

```toml
[dependencies]
leptix-toolbar = "1.0.0"
```

## Anatomy

```rust
use leptix_toolbar::*;

view! {
    <Toolbar>
        <ToolbarButton />
        <ToolbarSeparator />
        <ToolbarLink />
        <ToolbarToggleGroup r#type="single".into()>
            <ToolbarToggleItem value="a" />
            <ToolbarToggleItem value="b" />
        </ToolbarToggleGroup>
    </Toolbar>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_toolbar::*;

#[component]
fn EditorToolbar() -> impl IntoView {
    view! {
        <Toolbar attr:class="toolbar-root" attr:aria-label="Formatting options">
            <ToolbarToggleGroup r#type="multiple".into() attr:class="toolbar-toggle-group">
                <ToolbarToggleItem value="bold" attr:class="toolbar-toggle-item" attr:aria-label="Bold">
                    "B"
                </ToolbarToggleItem>
                <ToolbarToggleItem value="italic" attr:class="toolbar-toggle-item" attr:aria-label="Italic">
                    "I"
                </ToolbarToggleItem>
                <ToolbarToggleItem value="strikethrough" attr:class="toolbar-toggle-item" attr:aria-label="Strike through">
                    "S"
                </ToolbarToggleItem>
            </ToolbarToggleGroup>

            <ToolbarSeparator attr:class="toolbar-separator" />

            <ToolbarButton attr:class="toolbar-button">"Share"</ToolbarButton>

            <ToolbarSeparator attr:class="toolbar-separator" />

            <ToolbarLink attr:href="https://example.com" attr:class="toolbar-link">
                "Edited 2 hours ago"
            </ToolbarLink>
        </Toolbar>
    }
}
```

## Styling

```css
.toolbar-root {
    display: flex;
    padding: 10px;
    width: 100%;
    min-width: max-content;
    border-radius: 6px;
    background: white;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.toolbar-toggle-group {
    display: inline-flex;
}

.toolbar-toggle-item {
    background: transparent;
    border: none;
    color: #333;
    height: 32px;
    padding: 0 8px;
    border-radius: 4px;
    display: inline-flex;
    font-size: 13px;
    line-height: 1;
    align-items: center;
    justify-content: center;
    cursor: default;
}

.toolbar-toggle-item:hover {
    background-color: #f0f0f0;
}

.toolbar-toggle-item[data-state="on"] {
    background-color: #e0e0ff;
    color: #4c9aff;
}

.toolbar-separator {
    width: 1px;
    background-color: #e5e5e5;
    margin: 0 10px;
}

.toolbar-separator[data-orientation="vertical"] {
    width: 1px;
    height: auto;
}

.toolbar-button {
    padding: 0 10px;
    background: #4c9aff;
    color: white;
    border: none;
    border-radius: 4px;
    height: 32px;
    font-size: 13px;
    cursor: default;
}

.toolbar-link {
    color: #666;
    text-decoration: none;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 13px;
    margin-left: auto;
}
```

## API Reference

### Toolbar

The root container. Renders a `<div>` with `role="toolbar"`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `orientation` | `MaybeProp<String>` | `"horizontal"` | The orientation of the toolbar: `"horizontal"` or `"vertical"`. |
| `dir` | `MaybeProp<Direction>` | `"ltr"` | The reading direction. |
| `r#loop` | `MaybeProp<bool>` | `true` | When `true`, keyboard navigation loops from last to first and vice versa. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<div>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

### ToolbarButton

A button item within the toolbar. Renders a `<button>`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ToolbarLink

A link item within the toolbar. Renders an `<a>`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ToolbarSeparator

A separator between toolbar items. Its orientation is perpendicular to the toolbar's orientation.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"vertical"` (when toolbar is horizontal) \| `"horizontal"` (when toolbar is vertical) |

### ToolbarToggleGroup

A group of toggle items within the toolbar.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `r#type` | `MaybeProp<String>` | *required* | The selection mode: `"single"` or `"multiple"`. |
| `value` | `MaybeProp<Vec<String>>` | - | The controlled value(s). |
| `default_value` | `MaybeProp<Vec<String>>` | - | The value(s) when initially rendered. |
| `on_value_change` | `Option<Callback<Vec<String>>>` | - | Event handler called when the value changes. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents interacting with any toggle item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | Inherits the toolbar's orientation. |

### ToolbarToggleItem

An individual toggle item within a `ToolbarToggleGroup`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | *required* | A unique value that identifies this item. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with this item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"on"` \| `"off"` |
| `[data-disabled]` | Present when disabled. |

## Data Attributes

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` -- present on root, separator, and toggle group. |
| `[data-state]` | `"on"` \| `"off"` -- present on toggle items. |
| `[data-disabled]` | Present on disabled toggle items. |

## Accessibility

Adheres to the [Toolbar WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/toolbar/).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Tab` | Moves focus into and out of the toolbar. |
| `Space` | Activates the focused button or toggles the focused toggle item. |
| `Enter` | Activates the focused button or toggles the focused toggle item. |
| `ArrowRight` | Moves focus to the next item (horizontal orientation). |
| `ArrowLeft` | Moves focus to the previous item (horizontal orientation). |
| `ArrowDown` | Moves focus to the next item (vertical orientation). |
| `ArrowUp` | Moves focus to the previous item (vertical orientation). |
| `Home` | Moves focus to the first item. |
| `End` | Moves focus to the last item. |
