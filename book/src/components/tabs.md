# Tabs

A set of layered sections of content, known as tab panels, that are displayed one at a time.

## Features

- Can be controlled or uncontrolled
- Supports horizontal and vertical orientations
- Supports automatic or manual activation modes
- Full keyboard navigation with arrow keys, Home, and End
- Roving focus with optional looping

## Installation

```toml
[dependencies]
leptix-tabs = "1.0.0"
```

## Anatomy

```rust
use leptix_tabs::*;

view! {
    <Tabs>
        <TabsList>
            <TabsTrigger value="tab1" />
            <TabsTrigger value="tab2" />
        </TabsList>
        <TabsContent value="tab1" />
        <TabsContent value="tab2" />
    </Tabs>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_tabs::*;

#[component]
fn AccountTabs() -> impl IntoView {
    view! {
        <Tabs default_value="account" attr:class="tabs-root">
            <TabsList attr:class="tabs-list">
                <TabsTrigger value="account" attr:class="tabs-trigger">
                    "Account"
                </TabsTrigger>
                <TabsTrigger value="password" attr:class="tabs-trigger">
                    "Password"
                </TabsTrigger>
            </TabsList>

            <TabsContent value="account" attr:class="tabs-content">
                <p>"Make changes to your account here."</p>
                <input placeholder="Name" />
            </TabsContent>

            <TabsContent value="password" attr:class="tabs-content">
                <p>"Change your password here."</p>
                <input type="password" placeholder="New password" />
            </TabsContent>
        </Tabs>
    }
}
```

## Styling

```css
.tabs-list {
    display: flex;
    border-bottom: 1px solid #e5e7eb;
}

.tabs-trigger {
    all: unset;
    padding: 10px 16px;
    cursor: pointer;
    color: #6b7280;
}

.tabs-trigger[data-state="active"] {
    color: #111827;
    box-shadow: inset 0 -2px 0 0 currentColor;
}

.tabs-trigger[data-disabled] {
    opacity: 0.5;
    cursor: not-allowed;
}

.tabs-content {
    padding: 16px 0;
}

.tabs-content[data-state="inactive"] {
    display: none;
}
```

## API Reference

### Tabs

Contains all the parts of a tabs component.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `MaybeProp<String>` | - | The controlled value of the tab to activate. |
| `default_value` | `MaybeProp<String>` | - | The value of the tab to activate when initially rendered. |
| `on_value_change` | `Option<Callback<String>>` | - | Event handler called when the value changes. |
| `orientation` | `MaybeProp<String>` | `"horizontal"` | The orientation of the tabs. |
| `dir` | `MaybeProp<Direction>` | - | The reading direction. |
| `activation_mode` | `Option<ActivationMode>` | `ActivationMode::Automatic` | When `Automatic`, tabs are activated when receiving focus. When `Manual`, tabs are activated on click. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

### TabsList

Contains the triggers that are aligned along the edge of the active content.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `loop` | `MaybeProp<bool>` | `true` | When `true`, keyboard navigation will loop from last tab to first, and vice versa. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

### TabsTrigger

The button that activates its associated content.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | A unique value that associates the trigger with a content. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the tab. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"active"` \| `"inactive"` |
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |
| `[data-disabled]` | Present when disabled |

### TabsContent

Contains the content associated with each trigger.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `String` | - | A unique value that associates the content with a trigger. |
| `force_mount` | `MaybeProp<bool>` | `false` | Used to force mounting when more control is needed. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"active"` \| `"inactive"` |
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

## Accessibility

Adheres to the [Tabs WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/tabs).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `Tab` | When focus moves onto the tabs, focuses the active trigger. When focused on a trigger, moves focus to the active content. |
| `ArrowDown` | Moves focus to the next trigger (vertical orientation). |
| `ArrowUp` | Moves focus to the previous trigger (vertical orientation). |
| `ArrowRight` | Moves focus to the next trigger (horizontal orientation). |
| `ArrowLeft` | Moves focus to the previous trigger (horizontal orientation). |
| `Home` | Moves focus to the first trigger. |
| `End` | Moves focus to the last trigger. |
| `Space` | Activates the focused trigger (manual activation mode). |
| `Enter` | Activates the focused trigger (manual activation mode). |
