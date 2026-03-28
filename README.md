# Leptix

Radix-style accessible UI primitives for [Leptos](https://leptos.dev/).

Based on [RustForWeb/radix](https://github.com/RustForWeb/radix) (MIT), restructured and maintained as an independent project.

## Status

**0.1.0** — Phase 1 (Simple Primitives)

10 components:

| Component | Crate | Key Pattern |
|-----------|-------|-------------|
| Label | `leptix-label` | Crate structure, `as_child` |
| Separator | `leptix-separator` | Orientation, ARIA roles |
| Visually Hidden | `leptix-core` | Screen-reader-only content |
| Direction Provider | `leptix-core` | RTL/LTR context |
| Accessible Icon | `leptix-accessible-icon` | `aria-label`/`aria-hidden` |
| Aspect Ratio | `leptix-aspect-ratio` | CSS layout primitive |
| Progress | `leptix-progress` | ARIA progressbar |
| Toggle | `leptix-toggle` | `use_controllable_state`, `data-state` |
| Switch | `leptix-switch` | Form control, `role="switch"` |
| Avatar | `leptix-avatar` | Async image loading, fallback |

## Usage

Add `leptix-ui` to your `Cargo.toml`:

```toml
[dependencies]
leptix-ui = "0.1.0"
```

Or use individual crates:

```toml
[dependencies]
leptix-label = "0.1.0"
leptix-switch = "0.1.0"
```

## Crate Structure

```
crates/
  core/           — leptix-core: shared utilities and hooks
  leptix-ui/      — facade crate with feature flags
  primitives/     — one crate per component
    label/
    separator/
    accessible-icon/
    aspect-ratio/
    progress/
    toggle/
    switch/
    avatar/
```

## License

MIT
