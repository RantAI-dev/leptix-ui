# Changelog

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
