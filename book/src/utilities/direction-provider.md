# Direction Provider

Wraps your app to provide global reading direction (LTR/RTL). Used by components that adapt their layout based on text direction, such as Slider, Menu, and Navigation Menu.

## Installation

Part of `leptix-core` (included automatically with `leptix-ui`).

## Anatomy

```rust
use leptix_core::direction::{DirectionProvider, Direction};

view! {
    <DirectionProvider direction=Signal::derive(|| Direction::Rtl)>
        // All children inherit RTL direction
    </DirectionProvider>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_core::direction::{DirectionProvider, Direction};
use leptix_ui::slider::*;

#[component]
fn RtlApp() -> impl IntoView {
    view! {
        <DirectionProvider direction=Signal::derive(|| Direction::Rtl)>
            <Slider default_value=vec![50.0]>
                <SliderTrack>
                    <SliderRange />
                </SliderTrack>
                <SliderThumb />
            </Slider>
        </DirectionProvider>
    }
}
```

## API Reference

### DirectionProvider

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `direction` | `Signal<Direction>` | **required** | The reading direction. `Direction::Ltr` or `Direction::Rtl`. |

### Direction (enum)

| Variant | Value |
|---------|-------|
| `Direction::Ltr` | Left-to-right |
| `Direction::Rtl` | Right-to-left |

### use_direction

Hook for components to read the current direction:

```rust
use leptix_core::direction::use_direction;

// Inside a component:
let dir = use_direction(local_dir_prop);
// Returns Signal<Direction>, preferring local prop > context > Ltr default
```
