# Visually Hidden

Hides content from the screen in an accessible way. The content remains available to screen readers.

```rust
use leptix_core::visually_hidden::VisuallyHidden;

view! {
    <VisuallyHidden>"Only visible to screen readers"</VisuallyHidden>
}
```
