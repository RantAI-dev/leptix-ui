# Changelog

## 1.0.0

Leptix 1.0.0 — Radix UI Primitives for Leptos.

**33 packages:** 31 component crates + `leptix-core` + `leptix-ui` facade.

Components: Accordion, Alert Dialog, Aspect Ratio, Avatar, Checkbox, Collapsible, Context Menu, Dialog, Dropdown Menu, Form, Hover Card, Label, Menubar, Navigation Menu, OTP Field, Password Toggle, Popover, Progress, Radio Group, Scroll Area, Select, Separator, Slider, Switch, Tabs, Toast, Toggle, Toggle Group, Toolbar, Tooltip, Accessible Icon.

Core modules: arrow, compose_refs, direction, dismissable_layer, focus_guards, focus_scope, id, number, popper, portal, presence, primitive, use_controllable_state, use_escape_keydown, use_previous, use_size, visually_hidden.

24 unit tests. All components pass `cargo clippy -D warnings`, `cargo test`, and `wasm32-unknown-unknown` build.

Targets Leptos 0.8, Rust nightly (edition 2024).

---

## 0.4.0 — Phase 4: Menu Family + Full Parity

- Dropdown Menu with keyboard navigation, item focus, separator, label
- Context Menu (right-click triggered)
- Menubar with horizontal keyboard navigation
- Select with combobox trigger, listbox content, keyboard navigation
- Navigation Menu with hover/click sub-menus, links
- Toast with provider, viewport, auto-dismiss, title/description/action/close
- Scroll Area with viewport, scrollbar, thumb, corner
- Form with field, label, control, message, submit
- 29 component crates + leptix-core = 30 packages total

## 0.3.0 — Phase 3: Overlays & Floating

- Implemented `dismissable_layer` core module (escape key, click/focus outside detection)
- Implemented `focus_scope` core module (focus trapping, auto-focus, restore on unmount)
- New overlay components:
  - Dialog (modal with focus trap, escape dismiss, click-outside dismiss)
  - Alert Dialog (non-dismissable modal requiring explicit action)
  - Popover (non-modal floating content with dismissable layer)
  - Tooltip (hover/focus triggered popup)
  - Hover Card (rich preview content on hover)
  - Toolbar (grouped controls with roving keyboard navigation)
- 23 components total (24 counting Portal in core)

## 0.2.0 — Phase 2: Form Controls + Collapsible

- Rewrote `presence` core module for Leptos 0.8 with CSS animation awareness
- Ported Checkbox from old Leptos API (with presence, form reset handling)
- New components written from scratch:
  - Collapsible (open/close with presence-based animation support)
  - Toggle Group (single/multiple selection, builds on Toggle)
  - Radio Group (roving tabindex keyboard navigation, auto-select on focus)
  - Tabs (horizontal/vertical, automatic/manual activation, roving focus)
  - Accordion (single/multiple mode, builds on Collapsible, keyboard navigation)
  - Slider (keyboard control, track click, multi-thumb support, RTL)
- 17 components total

## 0.1.0 — Phase 1: Simple Primitives

- Ported Progress, Toggle, Switch, Avatar from old Leptos API to Leptos 0.8
- 10 components total: Label, Separator, Visually Hidden, Direction Provider, Accessible Icon, Aspect Ratio, Progress, Toggle, Switch, Avatar
- All components pass fmt, clippy, test, and wasm32 build checks

## 0.1.0-alpha.1 — Phase 0: Foundation

- Forked from [RustForWeb/radix](https://github.com/RustForWeb/radix) (MIT)
- Restructured into `leptix-core` + individual primitive crates
- Renamed all crates from `radix-leptos-*` to `leptix-*`
- Merged utility crates into `leptix-core`
- Created `leptix-ui` facade crate with feature flags
- Working primitives: Label, Separator, Accessible Icon, Aspect Ratio
- CI pipeline: fmt, clippy, test, wasm32 build
- Targets Leptos 0.8
