# leptix-core

Shared infrastructure for the [Leptix](https://github.com/rantai-dev/leptix) component library.

[![Crates.io](https://img.shields.io/crates/v/leptix-core.svg)](https://crates.io/crates/leptix-core)
[![docs.rs](https://img.shields.io/docsrs/leptix-core)](https://docs.rs/leptix-core)

## What's inside

`leptix-core` provides the building blocks that all Leptix primitives share:

- **Popper** -- Floating UI integration for collision-aware positioning
- **Portal** -- render children into `document.body`
- **Presence** -- mount/unmount with CSS animation support via `data-state`
- **FocusScope** -- trap focus within a container (used by Dialog, AlertDialog)
- **DismissableLayer** -- close on outside click, Escape key, or pointer-down-outside
- **RovingFocusGroup** -- arrow-key navigation between focusable items
- **FocusGuards** -- hidden sentinel elements that catch escaping focus
- **Slot** -- merge props onto a child element (Radix `asChild` pattern)
- **Accessible Icon** -- wrap an icon with a screen reader label
- **Direction** -- RTL/LTR context provider
- and more utility hooks and helpers

## Usage

Most users should depend on [`leptix-ui`](https://crates.io/crates/leptix-ui) (the umbrella crate) or individual primitive crates like `leptix-dialog`. Those crates re-export everything you need.

Depend on `leptix-core` directly only if you are building your own primitive on top of the Leptix infrastructure.

```toml
[dependencies]
leptix-core = "0.1"
```

## Full documentation

See the [Leptix README](https://github.com/rantai-dev/leptix) for the complete component list, installation guide, and examples.

## License

MIT
