# Changelog

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
