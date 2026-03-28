# Direction Provider

Wraps your app to provide global reading direction (LTR/RTL).

```rust
use leptix_core::direction::{DirectionProvider, Direction};

view! {
    <DirectionProvider direction=Signal::derive(|| Direction::Ltr)>
        // your app
    </DirectionProvider>
}
```
