# Radio Group

A set of checkable buttons, known as radio buttons, where no more than one of the buttons can be checked at a time.

## Features

- Can be controlled or uncontrolled
- Full keyboard navigation with roving focus
- Supports horizontal and vertical orientations
- Auto-selects on focus for immediate feedback
- Supports looping navigation

## Installation

```toml
[dependencies]
leptix-radio-group = "1.0.0"
```

## Anatomy

```rust
use leptix_radio_group::*;

view! {
    <RadioGroup>
        <RadioGroupItem value="option1">
            <RadioGroupIndicator />
        </RadioGroupItem>
        <RadioGroupItem value="option2">
            <RadioGroupIndicator />
        </RadioGroupItem>
    </RadioGroup>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_radio_group::*;

#[component]
fn PlanSelector() -> impl IntoView {
    view! {
        <RadioGroup default_value="comfortable" attr:class="radio-group">
            <div style="display: flex; align-items: center; gap: 8px;">
                <RadioGroupItem value="default" attr:class="radio-item" attr:id="r1">
                    <RadioGroupIndicator attr:class="radio-indicator" />
                </RadioGroupItem>
                <label r#for="r1">"Default"</label>
            </div>
            <div style="display: flex; align-items: center; gap: 8px;">
                <RadioGroupItem value="comfortable" attr:class="radio-item" attr:id="r2">
                    <RadioGroupIndicator attr:class="radio-indicator" />
                </RadioGroupItem>
                <label r#for="r2">"Comfortable"</label>
            </div>
            <div style="display: flex; align-items: center; gap: 8px;">
                <RadioGroupItem value="compact" attr:class="radio-item" attr:id="r3">
                    <RadioGroupIndicator attr:class="radio-indicator" />
                </RadioGroupItem>
                <label r#for="r3">"Compact"</label>
            </div>
        </RadioGroup>
    }
}
```

## Styling

```css
.radio-group {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.radio-item {
    all: unset;
    width: 20px;
    height: 20px;
    border-radius: 100%;
    border: 2px solid #6b7280;
    cursor: pointer;
}

.radio-item[data-state="checked"] {
    border-color: #3b82f6;
}

.radio-item[data-disabled] {
    opacity: 0.5;
    cursor: not-allowed;
}

.radio-indicator {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    position: relative;
}

.radio-indicator::after {
    content: "";
    display: block;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background-color: #3b82f6;
}

.radio-indicator[data-state="unchecked"]::after {
    display: none;
}
```

## API Reference

### RadioGroup

Contains all the parts of a radio group.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | `MaybeProp<String>` | - | The name of the group. Submitted with its owning form as part of a name/value pair. |
| `value` | `MaybeProp<String>` | - | The controlled value of the radio item to check. |
| `default_value` | `MaybeProp<String>` | - | The value of the radio item that should be checked when initially rendered. |
| `on_value_change` | `Option<Callback<String>>` | - | Event handler called when the value changes. |
| `required` | `MaybeProp<bool>` | `false` | When `true`, indicates that the user must check a radio item before the form can be submitted. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the radio group. |
| `orientation` | `MaybeProp<String>` | - | The orientation of the component. |
| `dir` | `MaybeProp<Direction>` | - | The reading direction. |
| `loop` | `MaybeProp<bool>` | `true` | When `true`, keyboard navigation will loop from last item to first, and vice versa. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-disabled]` | Present when disabled |

### RadioGroupItem

An item in the group that can be checked. An `input` will also render when used within a `form` to ensure events propagate correctly.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | The unique value of the radio item. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the radio item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` |
| `[data-disabled]` | Present when disabled |

### RadioGroupIndicator

Renders when the radio item is in a checked state. You can style this element directly, or use it as a wrapper to put an icon into.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `force_mount` | `MaybeProp<bool>` | `false` | Used to force mounting when more control is needed. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"checked"` \| `"unchecked"` |
| `[data-disabled]` | Present when disabled |

## Accessibility

Adheres to the [Radio Group WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/radio).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `ArrowDown` | Moves focus and checks the next radio item. |
| `ArrowRight` | Moves focus and checks the next radio item. |
| `ArrowUp` | Moves focus and checks the previous radio item. |
| `ArrowLeft` | Moves focus and checks the previous radio item. |
| `Tab` | Moves focus to the checked radio item, or the first item if none is checked. |
