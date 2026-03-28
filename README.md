# Leptix

Radix-style accessible UI primitives for [Leptos](https://leptos.dev/).

Based on [RustForWeb/radix](https://github.com/RustForWeb/radix) (MIT) and [Radix UI Primitives](https://www.radix-ui.com/primitives).

## Installation

```toml
[dependencies]
leptix-ui = "1.0.0"
```

Or pick individual crates:

```toml
[dependencies]
leptix-dialog = "1.0.0"
leptix-tabs = "1.0.0"
```

## Components (31)

| Component | Crate | Key Feature |
|-----------|-------|-------------|
| Accordion | `leptix-accordion` | Single/multiple expand, keyboard nav |
| Alert Dialog | `leptix-alert-dialog` | Non-dismissable modal |
| Aspect Ratio | `leptix-aspect-ratio` | CSS ratio container |
| Avatar | `leptix-avatar` | Image loading + fallback |
| Checkbox | `leptix-checkbox` | Indeterminate state, form integration |
| Collapsible | `leptix-collapsible` | Expand/collapse with animation |
| Context Menu | `leptix-context-menu` | Right-click menu |
| Dialog | `leptix-dialog` | Modal with focus trap |
| Dropdown Menu | `leptix-dropdown-menu` | Menu with submenus, checkbox/radio items |
| Form | `leptix-form` | Validation, accessible errors |
| Hover Card | `leptix-hover-card` | Rich hover preview |
| Label | `leptix-label` | Accessible form label |
| Menubar | `leptix-menubar` | Horizontal menu bar |
| Navigation Menu | `leptix-navigation-menu` | Top-level nav with sub-menus |
| OTP Field | `leptix-otp-field` | Multi-segment OTP input |
| Password Toggle | `leptix-password-toggle` | Show/hide password |
| Popover | `leptix-popover` | Floating content panel |
| Progress | `leptix-progress` | Determinate/indeterminate progress |
| Radio Group | `leptix-radio-group` | Single selection, roving focus |
| Scroll Area | `leptix-scroll-area` | Custom scrollbars |
| Select | `leptix-select` | Dropdown select with groups |
| Separator | `leptix-separator` | Visual/semantic separator |
| Slider | `leptix-slider` | Range input, multi-thumb |
| Switch | `leptix-switch` | Toggle switch, form control |
| Tabs | `leptix-tabs` | Tab panels, keyboard nav |
| Toast | `leptix-toast` | Notification queue |
| Toggle | `leptix-toggle` | Two-state button |
| Toggle Group | `leptix-toggle-group` | Single/multiple selection |
| Toolbar | `leptix-toolbar` | Grouped controls |
| Tooltip | `leptix-tooltip` | Hover/focus popup |
| Accessible Icon | `leptix-accessible-icon`* | Screen reader labels |

*\*Accessible Icon is exported from leptix-core.*

Plus `leptix-core` (17 shared modules) and `leptix-ui` (facade crate).

## Targets

- Leptos 0.8
- Rust nightly (edition 2024)
- `wasm32-unknown-unknown`

## License

MIT
