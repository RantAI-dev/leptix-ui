# Aspect Ratio

Displays content within a desired ratio.

## Features

- Accepts any custom ratio
- Uses CSS padding-bottom trick for consistent sizing
- Child content stretches to fill the ratio container

## Installation

```toml
[dependencies]
leptix-aspect-ratio = "1.0.0"
```

## Anatomy

```rust
use leptix_aspect_ratio::*;

view! {
    <AspectRatio>
        // content
    </AspectRatio>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_aspect_ratio::*;

#[component]
fn ImageWithRatio() -> impl IntoView {
    view! {
        <div style="width: 300px">
            <AspectRatio ratio=16.0 / 9.0>
                <img
                    src="https://images.unsplash.com/photo-1535025183041-0991a977e25b"
                    alt="Landscape"
                    style="object-fit: cover; width: 100%; height: 100%;"
                />
            </AspectRatio>
        </div>
    }
}
```

## Styling

```css
[data-leptix-aspect-ratio-wrapper] {
    position: relative;
    width: 100%;
}

[data-leptix-aspect-ratio-wrapper] > * {
    position: absolute;
    inset: 0;
}
```

## API Reference

### AspectRatio

Contains the content you want to constrain to a given ratio.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `ratio` | `Signal<f64>` | `1.0` | The desired ratio. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<div>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Data Attributes

| Data Attribute | Values |
|---------------|--------|
| `[data-leptix-aspect-ratio-wrapper]` | Present on the outer wrapper `<div>`. |

## Accessibility

This component is purely presentational and does not require specific WAI-ARIA attributes. The aspect ratio wrapper does not affect the accessibility tree.
