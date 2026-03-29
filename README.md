# Leptix

**Radix-quality accessible UI primitives for [Leptos](https://leptos.dev/)**

[![Crates.io](https://img.shields.io/crates/v/leptix-ui.svg)](https://crates.io/crates/leptix-ui)
[![docs.rs](https://img.shields.io/docsrs/leptix-ui)](https://docs.rs/leptix-ui)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Leptos](https://img.shields.io/badge/leptos-0.8-orange.svg)](https://leptos.dev/)

Leptix is a complete set of 33 unstyled, accessible UI primitives for Leptos, built to match the Radix UI API surface. Ship accessible components without fighting your design system.

## Features

- **1:1 Radix UI API parity** -- component names, props, and behaviors match Radix Primitives
- **33 accessible, unstyled primitives** -- from simple Label to complex Select and Navigation Menu
- **Floating UI positioning** -- Popover, Tooltip, HoverCard, Select, and DropdownMenu use collision-aware placement
- **Full keyboard navigation** -- arrow keys, Home/End, typeahead search in menus and selects
- **Focus management** -- focus trapping in dialogs, roving tabindex in groups, focus return on close
- **Portal rendering** -- overlays render into `document.body` to escape stacking contexts
- **WAI-ARIA compliant** -- correct roles, states, and properties out of the box
- **Controlled and uncontrolled modes** -- use signals for full control or let components manage their own state
- **RTL support** -- directional keyboard navigation adapts to writing direction
- **CSS animation support** -- `data-state` attributes (`open`/`closed`) for entry/exit animations
- **SSR-safe** -- all `web_sys` calls are guarded behind browser checks
- **Tree-shakeable** -- use the `leptix-ui` umbrella crate with feature flags, or depend on individual crates

## Quick Start

```rust
use leptix_ui::dialog::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"Open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay />
                <DialogContent>
                    <DialogTitle>"Edit Profile"</DialogTitle>
                    <DialogDescription>"Make changes to your profile."</DialogDescription>
                    <DialogClose>"Save"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}
```

## Installation

### Umbrella crate (all 33 primitives)

```toml
[dependencies]
leptix-ui = "0.1"
```

All primitives are enabled by default. To slim down compile times, disable defaults and pick what you need:

```toml
[dependencies]
leptix-ui = { version = "0.1", default-features = false, features = ["dialog", "tabs", "tooltip"] }
```

### Individual crates

```toml
[dependencies]
leptix-dialog = "0.1"
leptix-tabs = "0.1"
leptix-tooltip = "0.1"
```

## Components (33)

| Component | Crate | Key Feature |
|-----------|-------|-------------|
| Accordion | `leptix-accordion` | Single/multiple expand, keyboard nav |
| Alert Dialog | `leptix-alert-dialog` | Non-dismissable modal with required action |
| Aspect Ratio | `leptix-aspect-ratio` | CSS ratio container |
| Avatar | `leptix-avatar` | Image loading states + fallback |
| Checkbox | `leptix-checkbox` | Indeterminate state, form integration |
| Collapsible | `leptix-collapsible` | Expand/collapse with CSS animation |
| Context Menu | `leptix-context-menu` | Right-click menu with submenus |
| Dialog | `leptix-dialog` | Modal with focus trap and portal |
| Dropdown Menu | `leptix-dropdown-menu` | Menu with submenus, checkbox/radio items |
| Form | `leptix-form` | Validation and accessible error messages |
| Hover Card | `leptix-hover-card` | Rich hover preview with Floating UI |
| Label | `leptix-label` | Accessible form label |
| Menubar | `leptix-menubar` | Horizontal menu bar with keyboard nav |
| Navigation Menu | `leptix-navigation-menu` | Top-level nav with sub-menus and viewport |
| OTP Field | `leptix-otp-field` | Multi-segment one-time password input |
| Password Toggle | `leptix-password-toggle` | Show/hide password visibility |
| Popover | `leptix-popover` | Floating content panel with Floating UI |
| Progress | `leptix-progress` | Determinate/indeterminate progress bar |
| Radio Group | `leptix-radio-group` | Single selection with roving focus |
| Scroll Area | `leptix-scroll-area` | Custom scrollbars with native scrolling |
| Select | `leptix-select` | Dropdown select with groups and typeahead |
| Separator | `leptix-separator` | Visual and semantic separator |
| Slider | `leptix-slider` | Range input with multi-thumb support |
| Switch | `leptix-switch` | Toggle switch, form-friendly |
| Tabs | `leptix-tabs` | Tab panels with keyboard nav |
| Toast | `leptix-toast` | Notification queue with auto-dismiss |
| Toggle | `leptix-toggle` | Two-state button |
| Toggle Group | `leptix-toggle-group` | Single/multiple selection toggle set |
| Toolbar | `leptix-toolbar` | Grouped controls with roving focus |
| Tooltip | `leptix-tooltip` | Hover/focus popup with Floating UI |
| Visually Hidden | `leptix-visually-hidden` | Screen reader only content |
| Accessible Icon | `leptix-accessible-icon` | Icon with screen reader label |

Plus **`leptix-core`** (shared infrastructure: Popper, Portal, Presence, FocusScope, DismissableLayer, and more).

## Requirements

- **Leptos** 0.8+
- **Rust nightly** (edition 2024)
- **Target:** `wasm32-unknown-unknown`

## Architecture

```
leptix-ui (umbrella, re-exports all primitives)
  |
  +-- leptix-dialog, leptix-tabs, leptix-tooltip, ... (33 primitive crates)
        |
        +-- leptix-core (Popper, Portal, Presence, FocusScope, DismissableLayer, ...)
```

Each primitive crate is independently publishable. `leptix-core` provides the shared building blocks: floating positioning, focus management, dismiss-on-outside-click, portal rendering, and CSS animation presence detection.

## Links

- [GitHub](https://github.com/RantAI-dev/leptix-ui)
- [API Documentation (docs.rs)](https://docs.rs/leptix-ui)

## Credits

Built on the shoulders of:

- [Radix UI Primitives](https://www.radix-ui.com/primitives) -- the reference API and behavior specification
- [RustForWeb/radix](https://github.com/RustForWeb/radix) -- the original Rust/Leptos port (MIT)
- [Floating UI](https://floating-ui.com/) -- collision-aware positioning via [`floating-ui-leptos`](https://crates.io/crates/floating-ui-leptos)

## License

MIT
