# Progress

Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.

## Features

- Supports indeterminate state for unknown progress
- Accessible value label generation
- Custom value label formatting
- Data attributes for complete, loading, and indeterminate states

## Installation

```toml
[dependencies]
leptix-progress = "1.0.0"
```

## Anatomy

```rust
use leptix_progress::*;

view! {
    <Progress>
        <ProgressIndicator />
    </Progress>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_progress::*;

#[component]
fn DownloadProgress() -> impl IntoView {
    let (progress, set_progress) = signal(66.0);

    view! {
        <Progress value=progress attr:class="progress-root">
            <ProgressIndicator
                attr:class="progress-indicator"
                attr:style=move || {
                    format!("transform: translateX(-{}%)", 100.0 - progress.get())
                }
            />
        </Progress>
        <button on:click=move |_| set_progress.set(100.0)>
            "Complete"
        </button>
    }
}

#[component]
fn IndeterminateProgress() -> impl IntoView {
    view! {
        <Progress attr:class="progress-root">
            <ProgressIndicator attr:class="progress-indicator progress-indeterminate" />
        </Progress>
    }
}
```

## Styling

```css
.progress-root {
    position: relative;
    overflow: hidden;
    background: #e5e7eb;
    border-radius: 9999px;
    width: 300px;
    height: 16px;
}

.progress-indicator {
    background-color: #3b82f6;
    width: 100%;
    height: 100%;
    transition: transform 660ms cubic-bezier(0.65, 0, 0.35, 1);
    border-radius: 9999px;
}

.progress-root[data-state="complete"] .progress-indicator {
    background-color: #22c55e;
}

.progress-root[data-state="indeterminate"] .progress-indicator {
    width: 50%;
    animation: indeterminate 1.5s ease-in-out infinite;
}

@keyframes indeterminate {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(300%); }
}
```

## API Reference

### Progress

Contains all of the progress parts.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `MaybeProp<f64>` | - | The progress value. Set to `None` for indeterminate state. |
| `max` | `MaybeProp<f64>` | `100.0` | The maximum progress value. |
| `get_value_label` | `Option<Box<dyn Fn(f64, f64) -> String + Send + Sync>>` | - | A function to get the accessible label text representing the current value. If not provided, the value label will be read as a percentage. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"complete"` \| `"loading"` \| `"indeterminate"` |
| `[data-value]` | The current value |
| `[data-max]` | The max value |

### ProgressIndicator

Used to show the progress visually. It also makes progress accessible to assistive technologies.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-state]` | `"complete"` \| `"loading"` \| `"indeterminate"` |
| `[data-value]` | The current value |
| `[data-max]` | The max value |

## Accessibility

Adheres to the [Progressbar WAI-ARIA design pattern](https://www.w3.org/WAI/ARIA/apg/patterns/meter).

### Keyboard Interactions

Not applicable -- progress indicators are not interactive.
