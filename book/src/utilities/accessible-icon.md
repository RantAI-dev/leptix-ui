# Accessible Icon

Makes icons accessible by providing a visually hidden label for screen readers while hiding the icon from the accessibility tree.

## Features

- Adds `aria-hidden="true"` to the icon child element
- Renders a visually hidden label for screen readers
- Sets `focusable="false"` on the icon to prevent SVG focus issues

## Installation

```toml
[dependencies]
leptix-accessible-icon = "1.0.0"
```

## Anatomy

```rust
use leptix_accessible_icon::*;

view! {
    <AccessibleIcon label="Description">
        // icon element (e.g., SVG)
    </AccessibleIcon>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_accessible_icon::*;

#[component]
fn IconButton() -> impl IntoView {
    view! {
        <button>
            <AccessibleIcon label="Close dialog">
                <svg width="15" height="15" viewBox="0 0 15 15" fill="none">
                    <path
                        d="M11.78 4.22a.75.75 0 00-1.06 0L7.5 7.44 4.28 4.22a.75.75 0 00-1.06 1.06L6.44 8.5 3.22 11.72a.75.75 0 101.06 1.06L7.5 9.56l3.22 3.22a.75.75 0 101.06-1.06L8.56 8.5l3.22-3.22a.75.75 0 000-1.06z"
                        fill="currentColor"
                    />
                </svg>
            </AccessibleIcon>
        </button>
    }
}
```

## Styling

The `AccessibleIcon` component does not render a visible wrapper. The visually hidden label uses absolute positioning and clipping to remain invisible while being announced by screen readers.

## API Reference

### AccessibleIcon

Wraps an icon element, hiding it from the accessibility tree and providing a screen reader label.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `label` | `Signal<String>` | *required* | The accessible label for the icon. This text is visually hidden but announced to screen reader users, similar to `alt` text for `<img>` tags. |

## Data Attributes

This component does not emit custom data attributes. It adds `aria-hidden="true"` and `focusable="false"` to its child element.

## Accessibility

The child element receives `aria-hidden="true"` to hide it from the accessibility tree and `focusable="false"` to prevent SVG focus issues in certain browsers. A `VisuallyHidden` element containing the `label` text is rendered as a sibling, ensuring screen readers announce the icon's purpose.
