# Separator

Visually or semantically separates content.

## Features

- Supports horizontal and vertical orientations
- Can be decorative (no semantic meaning) or semantic (`role="separator"`)
- Applies correct `aria-orientation` based on context

## Installation

```toml
[dependencies]
leptix-separator = "1.0.0"
```

## Anatomy

```rust
use leptix_separator::*;

view! {
    <Separator />
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_separator::*;

#[component]
fn SeparatorDemo() -> impl IntoView {
    view! {
        <div style="max-width: 300px; margin: 0 15px;">
            <div>
                <p style="font-weight: 500;">"Leptix Primitives"</p>
                <p style="margin-top: 4px; color: #666;">"An accessible component library."</p>
            </div>
            <Separator attr:class="separator" />
            <div style="display: flex; height: 20px; align-items: center; gap: 15px;">
                <span>"Blog"</span>
                <Separator orientation=Orientation::Vertical attr:class="separator" />
                <span>"Docs"</span>
                <Separator orientation=Orientation::Vertical attr:class="separator" />
                <span>"Source"</span>
            </div>
        </div>
    }
}
```

## Styling

```css
.separator {
    background-color: #e5e5e5;
}

.separator[data-orientation="horizontal"] {
    height: 1px;
    width: 100%;
    margin: 15px 0;
}

.separator[data-orientation="vertical"] {
    height: 100%;
    width: 1px;
    margin: 0 15px;
}
```

## API Reference

### Separator

The separator element.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `orientation` | `MaybeProp<Orientation>` | `Horizontal` | The orientation of the separator. `Orientation::Horizontal` or `Orientation::Vertical`. |
| `decorative` | `MaybeProp<bool>` | `false` | When `true`, the separator is purely visual and carries no semantic meaning (`role="none"`). When `false`, renders with `role="separator"`. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<div>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Data Attributes

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |

## Accessibility

Adheres to the [Separator WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/separator/).

When `decorative` is `false`, the component renders with `role="separator"` and applies `aria-orientation="vertical"` when vertical. Horizontal separators do not need an explicit `aria-orientation` per the spec.

When `decorative` is `true`, the component renders with `role="none"` so it is removed from the accessibility tree.

### Keyboard Interactions

This component is not interactive and does not have keyboard interactions.
