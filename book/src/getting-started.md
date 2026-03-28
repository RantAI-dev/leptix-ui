# Getting Started

## Installation

Add `leptix-ui` to your `Cargo.toml`:

```toml
[dependencies]
leptix-ui = "0.1.0"
```

This includes all components by default. To reduce compile times, select only what you need:

```toml
[dependencies]
leptix-ui = { version = "0.1.0", default-features = false, features = ["dialog", "tabs", "switch"] }
```

Or use individual crates directly:

```toml
[dependencies]
leptix-dialog = "0.1.0"
leptix-tabs = "0.1.0"
```

## Quick Example

```rust
use leptos::prelude::*;
use leptix_ui::dialog::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"Open Dialog"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay />
                <DialogContent>
                    <DialogTitle>"Edit Profile"</DialogTitle>
                    <DialogDescription>"Make changes to your profile."</DialogDescription>
                    <DialogClose>"Close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}
```

## Styling

Leptix components are unstyled by default. Style them using:

- **CSS selectors**: Target `[data-state="open"]`, `[data-state="closed"]`, `[data-disabled]`, etc.
- **CSS classes**: Pass classes via the standard Leptos `class` attribute.
- **Tailwind CSS**: Works out of the box.

```css
[data-state="open"] {
    animation: fadeIn 200ms ease;
}
[data-state="closed"] {
    animation: fadeOut 200ms ease;
}
```

## Component Anatomy

Every Leptix component follows the Radix UI composition pattern:

```rust
// Root manages state
<Dialog>
    // Trigger opens the component
    <DialogTrigger>"Open"</DialogTrigger>
    // Portal renders in a different DOM location
    <DialogPortal>
        // Content is the main body
        <DialogContent>
            // Sub-components compose the UI
            <DialogTitle>"Title"</DialogTitle>
        </DialogContent>
    </DialogPortal>
</Dialog>
```

## Accessibility

All components implement WAI-ARIA patterns:
- Correct ARIA roles and attributes
- Keyboard navigation (Tab, Arrow keys, Enter, Space, Escape)
- Screen reader announcements
- Focus management for overlays
