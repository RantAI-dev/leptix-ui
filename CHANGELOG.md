# Changelog

## 0.1.6 — Full Radix Parity + API Gaps

Resolves all remaining partial fixes and structural API limitations.

- **#14** True safe triangle: convex hull geometry using barycentric point-in-triangle test between pointer exit point and sub-trigger corners. Document-level pointermove tracking with 400ms safety timeout.
- **#34** Toast pause-all/dismiss-all: ToastViewport pauses all toasts on focus/pointer-enter and resumes on blur/pointer-leave via provider registry.
- **L1** Slider `thumb_alignment` prop: "center" (default, thumb centered over value) or "contain" (thumb stays within track bounds).
- **L2** DropdownMenuSubContent `as_child` prop now wired through to PopperContent.

---

## 0.1.5 — Final Audit Fixes (36/36)

All 36 Radix compatibility issues resolved.

- **#17** `collision_padding_object` prop added to Popover, Tooltip, HoverCard for per-side padding via `Padding::PerSide(...)`.
- **#23** ContextMenu uses `page_x`/`page_y` with `position:absolute` for correct positioning regardless of scroll offset.
- **#31** Select `position="item-aligned"` mode: content positions itself over the trigger using fixed positioning aligned to the trigger rect.

---

## 0.1.4 — Remaining Compatibility Fixes

- **#17** Popover/Tooltip/HoverCard: expose `collision_boundary`, `sticky`, `hide_when_detached`, `update_position_strategy` props through to PopperContent.
- **#14** DropdownMenu safe triangle: pointer-aware grace period using trigger rect geometry instead of flat timer.
- **#26** HoverCard: `on_open_auto_focus`/`on_close_auto_focus` callbacks now invoked on open/close transitions.
- **#34** Toast: `Toast` components self-register with `ToastProvider` on mount and deregister on cleanup.

---

## 0.1.3 — Radix Compatibility Audit

33 Radix UI compatibility issues fixed for shadcn/ui (rustcn) parity.

### Core Infrastructure
- **Portal**: All portals (Dialog, DropdownMenu, ContextMenu, Menubar, Popover, Tooltip, HoverCard, Select) now teleport to `document.body` via `mount_to`. Portal wrapper uses `display:contents` to avoid stacking context issues. Context re-provided across `mount_to` boundary.
- **DismissableLayer**: Global layer stack — only topmost layer responds to Escape/pointer-outside. Right-clicks no longer dismiss.
- **Presence**: Collapsible/Accordion content unmounts immediately when no CSS animation exists (fallback for `prefers-reduced-motion`).
- **FocusScope**: Uses `getComputedStyle` instead of inline style checks. Skips elements inside `[inert]` subtrees.

### Component Fixes
- **Menubar**: Native item components replace broken re-exports that caused runtime panics.
- **RadioGroup**: Hidden `<input type="radio">` for native form submission.
- **ContextMenu**: 9 new sub-components (CheckboxItem, RadioGroup, RadioItem, ItemIndicator, Sub, SubTrigger, SubContent, Arrow).
- **DropdownMenu**: Sub-menus use Popper for floating positioning. Grace timer on pointerleave (safe triangle).
- **Accordion**: Removed double `<h3>` nesting; `AccordionHeader` is now required wrapper for `AccordionTrigger`. Content has `aria-labelledby`.
- **Tooltip**: Removed duplicate `VisuallyHidden` render. Added `instant-open` data-state.
- **Dialog**: Body scroll lock compensates for scrollbar width. Cleanup on Portal unmount.
- **Select**: `SelectItemIndicator` auto-hides when item not selected. Scroll buttons functional with pointer-hold scrolling. Spurious `on_open_change` on initial render fixed.
- **Slider**: Auto-assigns thumb index from render order.
- **ScrollArea**: `Hover` type shows scrollbar only on pointer hover. `Scroll` type shows only while scrolling.
- **NavigationMenu**: Full keyboard navigation (Arrow keys, Enter/Space, Escape).
- **Checkbox/Switch**: `form` prop for out-of-form submission.
- **Toast**: `hotkey` prop on Viewport. `data-swipe` absent when no swipe active.
- **AlertDialog**: `force_mount` prop passthrough.
- **HoverCard**: `on_open_auto_focus`/`on_close_auto_focus` props.
- **CheckboxIndicator**: Presence `node_ref` composed for animation tracking.

### All Menus (DropdownMenu, ContextMenu, Menubar)
- Typeahead character search with 1s buffer reset.
- Home/End/PageUp/PageDown focus navigation.
- Tab key prevention.

---

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
