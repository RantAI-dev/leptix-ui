# Select

Displays a list of options for the user to pick from, triggered by a button.

## Features

- Can be controlled or uncontrolled
- Supports grouped items with labels
- Full keyboard navigation
- Supports custom placeholder text
- Focus is trapped and managed within the content

## Installation

```toml
[dependencies]
leptix-select = "1.0.0"
```

## Anatomy

```rust
use leptix_select::*;

view! {
    <Select>
        <SelectTrigger>
            <SelectValue />
            <SelectIcon />
        </SelectTrigger>
        <SelectPortal>
            <SelectContent>
                <SelectScrollUpButton />
                <SelectViewport>
                    <SelectItem>
                        <SelectItemText />
                        <SelectItemIndicator />
                    </SelectItem>

                    <SelectGroup>
                        <SelectLabel />
                        <SelectItem>
                            <SelectItemText />
                            <SelectItemIndicator />
                        </SelectItem>
                    </SelectGroup>

                    <SelectSeparator />
                </SelectViewport>
                <SelectScrollDownButton />
                <SelectArrow />
            </SelectContent>
        </SelectPortal>
    </Select>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_select::*;

#[component]
fn FruitSelect() -> impl IntoView {
    view! {
        <Select default_value="apple">
            <SelectTrigger attr:class="select-trigger">
                <SelectValue placeholder="Select a fruit..." />
                <SelectIcon />
            </SelectTrigger>
            <SelectPortal>
                <SelectContent attr:class="select-content">
                    <SelectScrollUpButton />
                    <SelectViewport attr:class="select-viewport">
                        <SelectGroup>
                            <SelectLabel>"Fruits"</SelectLabel>
                            <SelectItem value="apple">
                                <SelectItemText>"Apple"</SelectItemText>
                                <SelectItemIndicator>"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem value="banana">
                                <SelectItemText>"Banana"</SelectItemText>
                                <SelectItemIndicator>"✓"</SelectItemIndicator>
                            </SelectItem>
                            <SelectItem value="orange">
                                <SelectItemText>"Orange"</SelectItemText>
                                <SelectItemIndicator>"✓"</SelectItemIndicator>
                            </SelectItem>
                        </SelectGroup>
                    </SelectViewport>
                    <SelectScrollDownButton />
                </SelectContent>
            </SelectPortal>
        </Select>
    }
}
```

## Styling

```css
.select-trigger {
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: 5px;
    padding: 8px 12px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    background: white;
    font-size: 14px;
    cursor: pointer;
}

.select-trigger[data-state="open"] {
    border-color: #3b82f6;
}

.select-trigger[data-placeholder] {
    color: #9ca3af;
}

.select-content {
    background: white;
    border-radius: 6px;
    padding: 5px;
    box-shadow: 0 10px 38px -10px rgba(22, 23, 24, 0.35);
}

.select-content[data-state="open"] {
    animation: scaleIn 100ms ease-out;
}

[role="option"][data-state="checked"] {
    background-color: #eff6ff;
    font-weight: 500;
}

[role="option"][data-disabled] {
    opacity: 0.5;
    pointer-events: none;
}
```

## API Reference

### Select

Contains all the parts of a select.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `MaybeProp<String>` | - | The controlled value. |
| `default_value` | `MaybeProp<String>` | - | The value when initially rendered. |
| `on_value_change` | `Option<Callback<String>>` | - | Event handler called when the value changes. |
| `open` | `MaybeProp<bool>` | - | The controlled open state. |
| `on_open_change` | `Option<Callback<bool>>` | - | Event handler called when the open state changes. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the select. |
| `name` | `MaybeProp<String>` | - | The name of the select for form submission. |

### SelectTrigger

The button that toggles the select.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when disabled |
| `[data-placeholder]` | Present when no value is selected |

### SelectValue

The part that reflects the selected value. By default the selected item's text will be rendered. If you want more control, you can control the select and pass your own children.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `placeholder` | `MaybeProp<String>` | - | The content that will be rendered when no value is selected. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### SelectIcon

A small icon rendered alongside the value as a visual cue for the select.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### SelectPortal

When used, portals the content part into the `body`.

### SelectContent

The component that pops out when the select is open.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |

### SelectViewport

The scrolling viewport that contains all of the items.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### SelectItem

The component that contains the select items.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | The value given as data when submitted with a name. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` |
| `[data-disabled]` | Present when disabled |

### SelectItemText

The textual part of the item. Should only contain the text you want to display for the item.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### SelectItemIndicator

Renders when the item is selected. You can style this element directly, or use it as a wrapper to put an icon into.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### SelectGroup

Used to group multiple items. Use in conjunction with `SelectLabel`.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### SelectLabel

Used to render the label of a group.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### SelectSeparator

Used to visually separate items in the select.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### SelectScrollUpButton

An optional button used to scroll up within the viewport.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### SelectScrollDownButton

An optional button used to scroll down within the viewport.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### SelectArrow

An optional arrow element to render alongside the select content.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Accessibility

Adheres to the [Listbox WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/listbox).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | When focus is on the trigger, opens the select. When focus is on an item, selects the focused item. |
| `Enter` | When focus is on the trigger, opens the select. When focus is on an item, selects the focused item. |
| `ArrowDown` | When focus is on the trigger, opens the select. Moves focus to the next item. |
| `ArrowUp` | When focus is on the trigger, opens the select. Moves focus to the previous item. |
| `Esc` | Closes the select and moves focus to the trigger. |
