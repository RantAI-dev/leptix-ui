# Accordion

A vertically stacked set of interactive headings that each reveal a section of content.

## Features

- Can expand one or multiple items at once
- Can be controlled or uncontrolled
- Supports horizontal and vertical orientations
- Full keyboard navigation with arrow keys, Home, and End
- Collapsible mode for single-type accordions

## Installation

```toml
[dependencies]
leptix-accordion = "1.0.0"
```

## Anatomy

```rust
use leptix_accordion::*;

view! {
    <Accordion r#type=AccordionType::Single>
        <AccordionItem value="item-1">
            <AccordionHeader>
                <AccordionTrigger />
            </AccordionHeader>
            <AccordionContent />
        </AccordionItem>
    </Accordion>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_accordion::*;

#[component]
fn FaqAccordion() -> impl IntoView {
    view! {
        <Accordion
            r#type=AccordionType::Single
            collapsible=true
            default_value=vec!["item-1".to_string()]
            attr:class="accordion-root"
        >
            <AccordionItem value="item-1">
                <AccordionHeader>
                    <AccordionTrigger attr:class="accordion-trigger">
                        "Is it accessible?"
                    </AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class="accordion-content">
                    "Yes. It adheres to the WAI-ARIA design pattern."
                </AccordionContent>
            </AccordionItem>

            <AccordionItem value="item-2">
                <AccordionHeader>
                    <AccordionTrigger attr:class="accordion-trigger">
                        "Is it unstyled?"
                    </AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class="accordion-content">
                    "Yes. It ships with zero styles so you can customize it."
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
```

## Styling

```css
.accordion-trigger {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 12px 16px;
    background: none;
    border: none;
    font-size: 15px;
    cursor: pointer;
}

.accordion-content {
    padding: 0 16px 12px;
    overflow: hidden;
}

.accordion-content[data-state="closed"] {
    display: none;
}

.accordion-content[data-state="open"] {
    animation: slideDown 200ms ease-out;
}

@keyframes slideDown {
    from { height: 0; }
    to { height: var(--leptix-collapsible-content-height); }
}
```

## API Reference

### Accordion

Contains all the parts of an accordion.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `type` | `AccordionType` | - | Whether a single or multiple items can be opened at the same time. |
| `value` | `MaybeProp<Vec<String>>` | - | The controlled value of the item(s) to expand. |
| `default_value` | `MaybeProp<Vec<String>>` | - | The value of the item(s) to expand when initially rendered. Use when you do not need to control the state. |
| `on_value_change` | `Option<Callback<Vec<String>>>` | - | Event handler called when the expanded state changes. |
| `collapsible` | `Option<bool>` | `false` | When type is `Single`, allows closing content when clicking trigger for an open item. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the accordion. |
| `orientation` | `MaybeProp<String>` | `"vertical"` | The orientation of the accordion. |
| `dir` | `MaybeProp<Direction>` | - | The reading direction. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"vertical"` \| `"horizontal"` |
| `[data-disabled]` | Present when disabled |

### AccordionItem

Contains all the parts of a collapsible section.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | A unique value for the item. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the item. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-disabled]` | Present when disabled |

### AccordionHeader

Wraps an `AccordionTrigger`. Use the `as_child` prop to update it to the appropriate heading level for your page.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-orientation]` | `"vertical"` \| `"horizontal"` |
| `[data-disabled]` | Present when disabled |

### AccordionTrigger

Toggles the collapsed state of its associated item.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"open"` \| `"closed"` |
| `[data-orientation]` | `"vertical"` \| `"horizontal"` |

### AccordionContent

Contains the collapsible content for an item.

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

Adheres to the [Accordion WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/accordion).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Space` | When focus is on a trigger, toggles the associated content. |
| `Enter` | When focus is on a trigger, toggles the associated content. |
| `Tab` | Moves focus to the next focusable element. |
| `Shift + Tab` | Moves focus to the previous focusable element. |
| `ArrowDown` | Moves focus to the next trigger (vertical orientation). |
| `ArrowUp` | Moves focus to the previous trigger (vertical orientation). |
| `ArrowRight` | Moves focus to the next trigger (horizontal orientation). |
| `ArrowLeft` | Moves focus to the previous trigger (horizontal orientation). |
| `Home` | Moves focus to the first trigger. |
| `End` | Moves focus to the last trigger. |
