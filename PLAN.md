# Leptix — Project Plan

**Accessible UI primitives for Leptos. Rust port of Radix UI.**

Repo: `github.com/RantAI-dev/leptix-ui`
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
[ ] Fork RustForWeb/radix → RantAI-dev/leptix-ui
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

**Ship:** `0.4.0` — All component crates exist, basic API surfaces in place.

---

### Phase 5: Menu Completeness

**Goal:** Full sub-component parity for the menu family — the largest gap from Radix 1:1.

*Reference: `ref/radix-ui-primitives/packages/react/menu/`, `dropdown-menu/`, `context-menu/`, `menubar/`*

The Radix menu family shares a common `Menu` base with 16 sub-components. Dropdown Menu, Context Menu, and Menubar all re-export the same item variants.

| Work Item | Sub-components to add |
|-----------|----------------------|
| **Shared menu base** | Extract common logic from dropdown-menu. Implement: `MenuGroup`, `MenuCheckboxItem`, `MenuRadioGroup`, `MenuRadioItem`, `MenuItemIndicator`, `MenuSub`, `MenuSubTrigger`, `MenuSubContent`, `MenuArrow`. |
| **Dropdown Menu** | Add: `DropdownMenuGroup`, `DropdownMenuCheckboxItem`, `DropdownMenuRadioGroup`, `DropdownMenuRadioItem`, `DropdownMenuItemIndicator`, `DropdownMenuSub`, `DropdownMenuSubTrigger`, `DropdownMenuSubContent`, `DropdownMenuArrow`. Wire typeahead search. |
| **Context Menu** | Add: `ContextMenuGroup`, `ContextMenuCheckboxItem`, `ContextMenuRadioGroup`, `ContextMenuRadioItem`, `ContextMenuItemIndicator`, `ContextMenuSub`, `ContextMenuSubTrigger`, `ContextMenuSubContent`, `ContextMenuArrow`. Position-at-cursor. |
| **Menubar** | Add: `MenubarPortal`, `MenubarGroup`, `MenubarCheckboxItem`, `MenubarRadioGroup`, `MenubarRadioItem`, `MenubarItemIndicator`, `MenubarArrow`, `MenubarSub`, `MenubarSubTrigger`, `MenubarSubContent`. Cross-menu arrow-key navigation. |

**Key behaviors to implement:**
- Submenu open/close with delay (pointer grace area)
- Typeahead character search across menu items
- `checked`/`onCheckedChange` for CheckboxItem
- `value`/`onValueChange` for RadioGroup/RadioItem
- ItemIndicator renders only when parent is checked

**Ship:** `0.5.0`

---

### Phase 6: Select Completeness

**Goal:** Full Radix Select parity — the second largest gap.

*Reference: `ref/radix-ui-primitives/packages/react/select/`*

Radix Select exports 16 sub-components. Leptix has 7.

| Work Item | Details |
|-----------|---------|
| `SelectIcon` | Decorative icon next to trigger value |
| `SelectViewport` | Scrollable container inside content, handles scroll position |
| `SelectScrollUpButton` / `SelectScrollDownButton` | Auto-scroll indicators when items overflow viewport |
| `SelectItemText` | Text portion of item (separate from indicator, used for typeahead matching) |
| `SelectItemIndicator` | Checkmark or custom indicator for the selected item |
| `SelectGroup` / `SelectLabel` | Grouped items with accessible group label (`role="group"` + `aria-labelledby`) |
| `SelectArrow` | Popper arrow pointing at trigger |
| Floating UI positioning | Wire `floating-ui-leptos` into `SelectContent` — add `side`, `side_offset`, `align`, `align_offset`, `position` props |
| Typeahead | Keyboard character search to jump to matching `SelectItemText` values |
| Native select fallback | Hidden `<select>` for form submission (like Switch/Checkbox BubbleInput pattern) |

**Ship:** `0.6.0`

---

### Phase 7: Behavioral Completeness + Missing Sub-components

**Goal:** Wire real behaviors that are currently stubbed. Add all missing sub-components across well-implemented primitives.

*Reference: read each component's `src/` directory in `ref/radix-ui-primitives/packages/react/`*

**Missing sub-components to add:**

| Component | Missing | From Radix |
|-----------|---------|------------|
| Accordion | `AccordionHeader` | Semantic wrapper with `data-orientation`, `data-state` |
| Popover | `PopoverAnchor`, `PopoverArrow` | Anchor for custom trigger positioning; Arrow via floating-ui |
| Tooltip | `TooltipArrow` | Arrow via floating-ui |
| Hover Card | `HoverCardArrow` | Arrow via floating-ui |
| Toolbar | `ToolbarToggleGroup`, `ToolbarToggleItem` | Inline toggle group within toolbar |
| Navigation Menu | `NavigationMenuSub`, `NavigationMenuIndicator` | Nested sub-navigation; animated active indicator |
| Form | `FormValidityState` | Render-prop exposing native validation state |

**Behavioral work:**

| Area | Components Affected | Work |
|------|-------------------|------|
| **Floating UI integration** | Popover, Tooltip, Hover Card, Select, all Menus | Wire `floating-ui-leptos` `use_floating()` with full middleware (Offset, Flip, Shift, Arrow, Hide). Add `side`, `side_offset`, `align`, `align_offset`, `avoid_collisions`, `collision_boundary`, `collision_padding`, `sticky`, `hide_when_detached` props to all content components. |
| **Toast auto-dismiss** | Toast | Wire timeout timer via `set_timeout`. Pause on hover/focus. Resume on leave/blur. Swipe-to-dismiss gesture. Proper viewport queue management. |
| **Scroll Area mechanics** | Scroll Area | Wire thumb size calculation from content/viewport ratio. Pointer-drag scrolling. Auto-hide scrollbar based on type (hover/scroll/auto/always). Track click to jump. |
| **Dismiss layer stacking** | Dialog, Popover, Menu family | Implement stacked dismiss layer manager — global stack tracks active layers, Escape/click-outside targets innermost layer only. |
| **Focus scope improvements** | Dialog, Alert Dialog, Popover | Focus on mount should prefer `[data-autofocus]` element, then first tabbable. Focus scope should handle `pointer-events: none` on body when modal. |
| **Hover intent** | Tooltip, Hover Card | Open/close delay timers. Pointer grace area (triangle between trigger and content). Skip delay when quickly moving between tooltips. |
| **Roving focus improvements** | Radio Group, Tabs, Toolbar, Menu | Use core roving-focus infrastructure instead of inline DOM queries. Support `orientation` prop, `dir` prop, `loop` prop consistently. |

**Ship:** `0.7.0`

---

### Phase 8: New Components (OTP + Password Toggle)

**Goal:** Implement the two Radix components that don't exist yet in Leptix.

*Reference: `ref/radix-ui-primitives/packages/react/one-time-password-field/`, `password-toggle-field/`*

| Component | Radix sub-components | Details |
|-----------|---------------------|---------|
| **OneTimePasswordField** | `OneTimePasswordField` (Root), `OneTimePasswordFieldInput`, `OneTimePasswordFieldHiddenInput` | Multi-segment OTP input. Single hidden `<input>` for form submission. Per-character slots rendered visually. Paste handling distributes across slots. Auto-advance cursor on keypress. Backspace navigates backward. `InputValidationType` enum (numeric, alphanumeric, etc.). |
| **PasswordToggleField** | `PasswordToggleField` (Root), `PasswordToggleFieldInput`, `PasswordToggleFieldToggle`, `PasswordToggleFieldIcon`, `PasswordToggleFieldSlot` | Password input with reveal toggle. Slot for custom input rendering. Icon slot shows current visibility state. Toggle button switches `type="password"` ↔ `type="text"`. |

**Ship:** `0.8.0`

---

### Phase 9: Prop Audit + Testing + Documentation

**Goal:** Every component matches the Radix API surface exactly. Full test coverage. Complete documentation.

**Prop-by-prop audit:**

For every component, diff our props against Radix's TypeScript prop types (in `ref/radix-ui-primitives/packages/react/*/src/*.tsx`). Add any missing props. This includes:

| Pattern | Missing everywhere | Action |
|---------|--------------------|--------|
| `forceMount` prop | Overlay content components | Add to all presence-gated content (CollapsibleContent, AccordionContent, DialogContent, etc.) — most already have this |
| `onOpenAutoFocus` / `onCloseAutoFocus` | Popover, Menu content | Add focus lifecycle callbacks |
| `onEscapeKeyDown` / `onPointerDownOutside` / `onFocusOutside` / `onInteractOutside` | All dismissable content | Wire all 4 dismiss callbacks consistently |
| `asChild` | Verify on every sub-component | Audit each component; some may be missing it |
| `dir` (direction) | Menu family, Select, Slider | Wire RTL/LTR from direction context |
| `loop` | Roving focus groups | Wire to keyboard navigation |

**Testing:**

| Work Item | Details |
|-----------|---------|
| **Unit tests** | State logic, prop defaults, data-state computation for every component. `#[cfg(test)]` modules. |
| **WASM integration tests** | `wasm-bindgen-test` for DOM rendering, event handling, ARIA attributes. Priority: Dialog, Select, Dropdown Menu, Tabs, Accordion, Radio Group. |
| **SSR tests** | `cargo test --features ssr` — verify every component renders valid HTML without `window`/`document` access. |
| **Accessibility audit** | Manual screen reader testing (VoiceOver + NVDA) for all overlay and form components. |
| **Keyboard interaction audit** | Verify every component matches the Radix keyboard interaction table exactly. |

**Documentation:**

| Work Item | Details |
|-----------|---------|
| **mdBook page per component** | Overview, anatomy diagram, full API reference (all props + all sub-components), usage examples, keyboard table. |
| **Showcase app** | Leptos SSR app with interactive demos for every component. Deploy to GitHub Pages. |
| **Migration guide** | From `radix-leptos-*` or `cloud-shuttle/radix-leptos`. |

**Ship:** `1.0.0-rc.1`

---

### Phase 10: 1.0 Release

**Goal:** Stable release with full Radix 1:1 parity.

| Work Item | Details |
|-----------|---------|
| Final API review | Lock public API. Review all `pub` exports. Verify no accidental internal types are public. |
| Version bump | All crates to `1.0.0`. |
| crates.io publish | Publish all 31+ crates. Verify `leptix-*` names are available. |
| Documentation site | Deploy mdBook to `leptix.dev` or GitHub Pages. |
| Announcement | Blog post, r/rust, Leptos Discord, Hacker News. |
| awesome-leptos PR | Submit listing. |
| Radix version tracking | Document which Radix UI Primitives version we match (tag/commit). |

**Ship:** `1.0.0` — Full Radix 1:1 parity.

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