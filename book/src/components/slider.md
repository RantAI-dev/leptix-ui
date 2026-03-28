# Slider

An input where the user selects a value from within a given range.

## Features

- Can be controlled or uncontrolled
- Supports multiple thumbs for range selection
- Supports horizontal and vertical orientations
- Supports minimum, maximum, and step values
- Full keyboard navigation including Page Up/Down and Home/End

## Installation

```toml
[dependencies]
leptix-slider = "1.0.0"
```

## Anatomy

```rust
use leptix_slider::*;

view! {
    <Slider>
        <SliderTrack>
            <SliderRange />
        </SliderTrack>
        <SliderThumb />
    </Slider>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_slider::*;

#[component]
fn VolumeSlider() -> impl IntoView {
    view! {
        <Slider
            default_value=vec![50.0]
            max=100.0
            step=1.0
            attr:class="slider-root"
        >
            <SliderTrack attr:class="slider-track">
                <SliderRange attr:class="slider-range" />
            </SliderTrack>
            <SliderThumb attr:class="slider-thumb" />
        </Slider>
    }
}

#[component]
fn PriceRangeSlider() -> impl IntoView {
    view! {
        <Slider
            default_value=vec![25.0, 75.0]
            min=0.0
            max=100.0
            step=1.0
            attr:class="slider-root"
        >
            <SliderTrack attr:class="slider-track">
                <SliderRange attr:class="slider-range" />
            </SliderTrack>
            <SliderThumb index=0 attr:class="slider-thumb" />
            <SliderThumb index=1 attr:class="slider-thumb" />
        </Slider>
    }
}
```

## Styling

```css
.slider-root {
    position: relative;
    display: flex;
    align-items: center;
    width: 200px;
    height: 20px;
}

.slider-track {
    background-color: #e5e7eb;
    position: relative;
    flex-grow: 1;
    border-radius: 9999px;
    height: 3px;
}

.slider-range {
    background-color: #3b82f6;
    border-radius: 9999px;
    height: 100%;
}

.slider-thumb {
    display: block;
    width: 20px;
    height: 20px;
    background: white;
    border: 2px solid #3b82f6;
    border-radius: 50%;
    cursor: pointer;
}

.slider-thumb:focus {
    outline: none;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.5);
}

.slider-root[data-disabled] {
    opacity: 0.5;
    pointer-events: none;
}
```

## API Reference

### Slider

Contains all the parts of a slider.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `MaybeProp<Vec<f64>>` | - | The controlled value. Use with `on_value_change`. |
| `default_value` | `MaybeProp<Vec<f64>>` | `[0]` | The value when initially rendered. |
| `on_value_change` | `Option<Callback<Vec<f64>>>` | - | Event handler called when the value changes during interaction. |
| `on_value_commit` | `Option<Callback<Vec<f64>>>` | - | Event handler called when the value changes at the end of an interaction (e.g. key up, pointer up). |
| `min` | `MaybeProp<f64>` | `0.0` | The minimum value. |
| `max` | `MaybeProp<f64>` | `100.0` | The maximum value. |
| `step` | `MaybeProp<f64>` | `1.0` | The stepping interval. |
| `disabled` | `MaybeProp<bool>` | `false` | When `true`, prevents the user from interacting with the slider. |
| `orientation` | `MaybeProp<String>` | `"horizontal"` | The orientation of the slider. |
| `dir` | `MaybeProp<Direction>` | - | The reading direction. |
| `inverted` | `MaybeProp<bool>` | `false` | Whether the slider is visually inverted. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |
| `[data-disabled]` | Present when disabled |

### SliderTrack

The track that contains the slider range.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |
| `[data-disabled]` | Present when disabled |

### SliderRange

The filled part of the track that indicates the selected range.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |
| `[data-disabled]` | Present when disabled |

### SliderThumb

A draggable thumb. You can render multiple thumbs for range selection.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `index` | `Option<usize>` | `0` | The index of the value this thumb controls (for multi-thumb sliders). |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"horizontal"` \| `"vertical"` |
| `[data-disabled]` | Present when disabled |

## Accessibility

Adheres to the [Slider WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/slider).

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `ArrowRight` | Increments the value by the step amount. |
| `ArrowLeft` | Decrements the value by the step amount. |
| `ArrowUp` | Increments the value by the step amount. |
| `ArrowDown` | Decrements the value by the step amount. |
| `PageUp` | Increments the value by a larger step (10x step). |
| `PageDown` | Decrements the value by a larger step (10x step). |
| `Home` | Sets the value to the minimum. |
| `End` | Sets the value to the maximum. |
