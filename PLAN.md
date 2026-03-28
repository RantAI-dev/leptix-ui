# Leptix — Project Plan

**Accessible UI primitives for Leptos. Rust port of Radix UI.**

Repo: `github.com/rantai-dev/leptix`
License: MIT
By: RantAI

---

## 1. What Is Leptix

Leptix is a pure-Rust port of [Radix UI Primitives](https://www.radix-ui.com/primitives) for the [Leptos](https://leptos.dev) web framework. Unstyled, accessible, composable components that serve as the foundation for design systems and styled component libraries.

### Scope

| Category | Components | Count |
|----------|-----------|-------|
| **Components** | Accordion, Alert Dialog, Aspect Ratio, Avatar, Checkbox, Collapsible, Context Menu, Dialog, Dropdown Menu, Form, Hover Card, Label, Menubar, Navigation Menu, Popover, Progress, Radio Group, Scroll Area, Select, Separator, Slider, Switch, Tabs, Toast, Toggle, Toggle Group, Toolbar, Tooltip | 28 |
| **Utilities** | Accessible Icon, Direction Provider, Portal, Slot, Visually Hidden | 5 |
| **Preview** | One-Time Password Field, Password Toggle Field | 2 |
| **Total** | | **35** |

### Out of Scope

- Radix Themes (design tokens, color system) → `rantai-ui`
- Radix Icons → separate crate or use `leptos-icons`
- Radix Colors → separate crate
- Styled/Tailwind components → `rantai-ui`
- CLI tooling → `rantai-ui`

---

## 2. Starting Point: Fork RustForWeb/radix

[RustForWeb/radix](https://github.com/RustForWeb/radix) was **archived Feb 2, 2026** (MIT license, 614 commits). We fork and reshape it:

1. **Strip Yew and Dioxus code** — Leptix is Leptos-only.
2. **Audit existing work** — Each primitive is in a different state of completion. Audit what compiles, what runs, what matches the Radix API.
3. **Modernize to Leptos 0.7+** — Update signal APIs, view macro syntax, context patterns.
4. **Rename crates** — `radix-leptos-*` → `leptix-*`
5. **Set up CI/CD** — GitHub Actions, crates.io publishing, mdBook docs.

### Post-fork checklist

```
[ ] Fork RustForWeb/radix → rantai-dev/leptix
[ ] Delete packages/primitives/yew/, packages/primitives/dioxus/
[ ] Delete Yew/Dioxus stories and book examples
[ ] Rename all crate names: radix-leptos-* → leptix-*
[ ] Update Cargo.toml workspace members
[ ] Verify cargo build --workspace compiles
[ ] Verify cargo test --workspace passes
[ ] Set up GitHub Actions CI (fmt, clippy, test, wasm)
[ ] Create CONTRIBUTING.md
[ ] Create CHANGELOG.md
[ ] Update README with leptix branding
[ ] Update book/ with leptix branding
[ ] Reserve crate names on crates.io (publish 0.1.0-alpha)
[ ] Submit to awesome-leptos
```

---

## 3. Repo Structure

```
leptix/
├── Cargo.toml                  # Workspace root
│
├── crates/
│   ├── leptix/                 # Facade crate (re-exports everything)
│   │   └── Cargo.toml          # features = ["dialog", "popover", ...]
│   │
│   ├── core/                   # leptix-core
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── primitive.rs    # Base Primitive component
│   │       ├── compose_refs.rs # Ref merging
│   │       ├── context.rs      # Context utilities
│   │       ├── id.rs           # Deterministic ID generation (SSR-safe)
│   │       ├── direction.rs    # RTL/LTR direction provider
│   │       ├── presence.rs     # Mount/unmount animation management
│   │       ├── collection.rs   # Child item collection pattern
│   │       ├── controllable.rs # Controlled/uncontrolled state
│   │       ├── dismissable.rs  # Click-outside + escape layer
│   │       ├── focus_scope.rs  # Focus trapping
│   │       ├── focus_guards.rs # Focus guard sentinels
│   │       ├── roving_focus.rs # Arrow key navigation in groups
│   │       ├── popper.rs       # Floating UI integration layer
│   │       ├── portal.rs       # Portal wrapper
│   │       ├── slot.rs         # AsChild / Slot pattern
│   │       └── visually_hidden.rs
│   │
│   ├── primitives/
│   │   ├── accordion/          # leptix-accordion
│   │   ├── alert-dialog/       # leptix-alert-dialog
│   │   ├── aspect-ratio/       # leptix-aspect-ratio
│   │   ├── avatar/             # leptix-avatar
│   │   ├── checkbox/           # leptix-checkbox
│   │   ├── collapsible/        # leptix-collapsible
│   │   ├── context-menu/       # leptix-context-menu
│   │   ├── dialog/             # leptix-dialog
│   │   ├── dropdown-menu/      # leptix-dropdown-menu
│   │   ├── hover-card/         # leptix-hover-card
│   │   ├── label/              # leptix-label
│   │   ├── menubar/            # leptix-menubar
│   │   ├── navigation-menu/    # leptix-navigation-menu
│   │   ├── popover/            # leptix-popover
│   │   ├── progress/           # leptix-progress
│   │   ├── radio-group/        # leptix-radio-group
│   │   ├── scroll-area/        # leptix-scroll-area
│   │   ├── select/             # leptix-select
│   │   ├── separator/          # leptix-separator
│   │   ├── slider/             # leptix-slider
│   │   ├── switch/             # leptix-switch
│   │   ├── tabs/               # leptix-tabs
│   │   ├── toast/              # leptix-toast
│   │   ├── toggle/             # leptix-toggle
│   │   ├── toggle-group/       # leptix-toggle-group
│   │   ├── toolbar/            # leptix-toolbar
│   │   └── tooltip/            # leptix-tooltip
│   │
│   └── menu/                   # leptix-menu (shared by dropdown, context, menubar)
│
├── examples/
│   ├── showcase/               # Interactive demo site (Leptos SSR app)
│   └── basic/                  # Minimal usage examples
│
├── book/                       # mdBook documentation
│   └── src/
│       ├── SUMMARY.md
│       ├── overview/
│       │   ├── introduction.md
│       │   ├── getting-started.md
│       │   └── accessibility.md
│       ├── guides/
│       │   ├── styling.md
│       │   ├── animation.md
│       │   └── ssr.md
│       └── primitives/
│           ├── accordion.md
│           ├── dialog.md
│           └── ...
│
└── .github/
    └── workflows/
        ├── ci.yml
        ├── docs.yml
        └── release.yml
```

### Consumer usage

```toml
# Option A: facade crate with feature flags (recommended)
[dependencies]
leptix = { version = "0.1", features = ["dialog", "popover", "tabs"] }

# Option B: individual crates
[dependencies]
leptix-dialog = "0.1"
leptix-popover = "0.1"
leptix-tabs = "0.1"
```

```rust
use leptix::dialog::*;
// or
use leptix_dialog::*;
```

---

## 4. Key Dependencies

| Crate | Purpose | Status |
|-------|---------|--------|
| `leptos` 0.7+ | Framework | Stable |
| `floating-ui-leptos` | Positioning (tooltips, popovers, menus) | Active, MIT, [RustForWeb/floating-ui](https://github.com/RustForWeb/floating-ui) |
| `leptos-use` | Composable utilities (click-outside, element-size, etc.) | Active |
| `web-sys` | DOM APIs | Stable |
| `wasm-bindgen` | JS interop | Stable |
| `send_wrapper` | WASM Send/Sync workarounds | Stable |

`floating-ui-leptos` provides `use_floating()` with full middleware support (Offset, Flip, Shift, Arrow, Size, AutoPlacement, Hide, Inline). This covers positioning for Popover, Tooltip, Dropdown Menu, Select, Hover Card, Context Menu, Menubar, and Navigation Menu.

---

## 5. Phased Roadmap

### Phase 0: Fork & Foundation

**Goal:** Clean fork, compiles, CI green.

- Fork, strip Yew/Dioxus, rename crates
- Fix compilation for Leptos 0.7+
- Set up CI (fmt, clippy, test, wasm-pack)
- Set up mdBook + GitHub Pages deploy
- Reserve crate names on crates.io
- CONTRIBUTING.md, README, LICENSE

**Ship:** `0.1.0-alpha.1`

---

### Phase 1: Simple Primitives

**Goal:** First production-ready components. Prove out all core patterns.

| Component | Complexity | Core Pattern It Validates |
|-----------|-----------|---------------------------|
| Label | Low | Basic primitive, `for` attribute |
| Separator | Low | Orientation, decorative vs semantic |
| Visually Hidden | Low | Screen-reader-only content |
| Aspect Ratio | Low | CSS-based layout primitive |
| Progress | Low | ARIA progressbar, determinate/indeterminate |
| Toggle | Low | Controlled/uncontrolled state, `data-state` |
| Avatar | Medium | Image loading states, fallback rendering |
| Switch | Medium | Form control, ARIA switch role, keyboard |
| Accessible Icon | Low | aria-label/aria-hidden |
| Direction Provider | Low | RTL/LTR context |

**Ship:** `0.1.0` — 10 components.

**Validates:** Slot/AsChild pattern, controlled/uncontrolled state, `data-state` attributes, SSR correctness, release pipeline.

---

### Phase 2: Form Controls + Collapsible

**Goal:** Form primitives and animation-aware components.

| Component | Complexity | Why Now |
|-----------|-----------|---------|
| Checkbox | Medium | Indeterminate state, form integration |
| Radio Group | Medium | Roving tabindex pattern (reusable) |
| Collapsible | Medium | Presence/animation pattern (reusable) |
| Tabs | Medium | Roving tabindex + panels + keyboard nav |
| Accordion | Medium | Builds on Collapsible, single/multiple mode |
| Toggle Group | Medium | Builds on Toggle, single/multiple selection |
| Slider | High | Multiple thumbs, step, RTL, touch |

**Ship:** `0.2.0` — 17 components total.

**Validates:** Roving focus groups, presence/animation, collection pattern, complex keyboard interactions.

---

### Phase 3: Overlays & Floating

**Goal:** Portals, positioning, focus traps, dismiss layers.

| Component | Complexity | Dependencies |
|-----------|-----------|--------------|
| Portal | Low | Leptos built-in `<Portal>` wrapper |
| Dialog | High | Portal, focus trap, dismiss layer, overlay |
| Alert Dialog | High | Builds on Dialog, blocks interaction |
| Popover | High | floating-ui-leptos, portal, dismiss, focus |
| Tooltip | High | floating-ui-leptos, hover intent, delay |
| Hover Card | Medium | floating-ui-leptos, hover intent |
| Toolbar | Medium | Roving tabindex |

**Ship:** `0.3.0` — 24 components total.

This is the make-or-break phase. Dialog and Popover are the most complex and most demanded primitives.

---

### Phase 4: Menu Family + Full Parity

**Goal:** Complete Radix parity.

| Component | Complexity | Notes |
|-----------|-----------|-------|
| Menu (shared internal) | Very High | Base for Dropdown, Context, Menubar |
| Dropdown Menu | Very High | Submenus, checkable items, typeahead |
| Context Menu | Very High | Right-click trigger, builds on Menu |
| Menubar | Very High | Multiple menu triggers, keyboard nav |
| Select | Very High | Typeahead, scroll, virtual list, portal |
| Navigation Menu | Very High | Viewport-based, indicator, complex layout |
| Toast | High | Viewport management, swipe, auto-dismiss |
| Scroll Area | High | Custom scrollbar, cross-browser |
| Form | Medium | Client-side validation, accessible errors |
| OTP Field | Medium | Multi-input, paste handling |
| Password Toggle | Low | Show/hide password |

**Ship:** `1.0.0` — Full Radix parity (35 components).

---

## 6. Core Patterns

These are shared across many components. Getting them right early makes later phases dramatically easier.

### 6.1 Controlled / Uncontrolled State

```rust
#[component]
pub fn Toggle(
    #[prop(optional, into)]
    pressed: Option<MaybeSignal<bool>>,
    #[prop(optional)]
    default_pressed: Option<bool>,
    #[prop(optional, into)]
    on_pressed_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let (is_pressed, set_pressed) = use_controllable_state(
        pressed,
        default_pressed.unwrap_or(false),
        on_pressed_change,
    );
    // ...
}
```

Build `use_controllable_state` in `leptix-core` first.

### 6.2 Data-State Attributes

```rust
attr:data-state=move || if open.get() { "open" } else { "closed" }
```

Consumers style with `[data-state="open"]` selectors.

### 6.3 Slot / AsChild

```rust
view! { <DialogTrigger>"Open"</DialogTrigger> }

view! {
    <DialogTrigger as_child=true>
        <a href="#">"Open as link"</a>
    </DialogTrigger>
}
```

### 6.4 Dismiss Layer Stack

Manages stacked overlays — which layer captures Escape, which captures click-outside, correct stacking order.

### 6.5 Focus Scope

Focus trapping for modals — trap within boundary, return focus on close, sentinel elements.

### 6.6 Roving Focus Group

Arrow key navigation in groups — configurable orientation, loop wrapping, Home/End support. Used by Tabs, Radio Group, Menu, Toolbar.

---

## 7. Per-Component Porting Checklist

```
STUDY
  [ ] Read Radix React source for this primitive
  [ ] Read the Radix docs (API, accessibility, keyboard interactions)
  [ ] Read the relevant WAI-ARIA design pattern
  [ ] Check what RustForWeb/radix had already
  [ ] List all sub-components (e.g., Dialog.Root, Dialog.Trigger, Dialog.Content)

IMPLEMENT
  [ ] Port or write each sub-component
  [ ] All ARIA attributes correct
  [ ] All keyboard interactions working
  [ ] Controlled + uncontrolled modes
  [ ] data-state attributes
  [ ] data-disabled attribute where applicable
  [ ] SSR renders valid HTML without JS
  [ ] AsChild/Slot support on relevant parts

TEST
  [ ] Unit tests: state logic, props, defaults
  [ ] WASM tests: DOM rendering, events, ARIA
  [ ] Keyboard: Tab, Shift+Tab, Arrow, Enter, Space, Escape
  [ ] Screen reader: manual test (VoiceOver or NVDA)
  [ ] SSR: cargo test --features ssr

DOCUMENT
  [ ] mdBook page: overview, anatomy, API reference, examples
  [ ] Showcase entry: interactive demo

SHIP
  [ ] Changelog entry
  [ ] Version bump
  [ ] Publish to crates.io
```

---

## 8. CI/CD

```yaml
name: CI
on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
          components: clippy, rustfmt
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo test --workspace
      - run: cargo build --workspace --target wasm32-unknown-unknown
      - run: cargo test --workspace --features ssr

  wasm-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - run: cargo install wasm-pack
      - run: wasm-pack test --headless --firefox crates/core

  docs:
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - run: cargo install mdbook
      - run: mdbook build book/
      - uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./book/book
```

Release via `cargo-release` for independent per-crate version bumps.

---

## 9. Community

**Launch (Phase 0):** Post on r/rust, Leptos Discord. Reach out to DanielleHuisman (RustForWeb author). Submit to awesome-leptos.

**First release (0.1.0):** Blog post on building accessible UI primitives in Rust. Share on r/rust, Hacker News, Rust Indonesia communities.

**Ongoing:** "Good first issue" labels per unported primitive. Monthly progress updates. All PRs require the porting checklist from Section 7.

---

## 10. Success Criteria for 1.0

Every component must:
- [ ] Match the Radix UI Primitives API surface
- [ ] Pass WCAG 2.1 AA accessibility audit
- [ ] Work with Leptos SSR + hydration
- [ ] Have mdBook documentation with examples
- [ ] Have unit + integration tests
- [ ] Be published on crates.io
- [ ] Be demonstrated in the showcase app

The project must:
- [ ] Be listed in awesome-leptos
- [ ] Have at least 1 production user (RantAI counts)

---

*Leptix Project Plan v1.0 — March 2026 — RantAI*