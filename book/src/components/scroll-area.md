# Scroll Area

Augments native scroll functionality for custom, cross-browser styled scrollbars.

## Features

- Custom scrollbar styling with no layout shift
- Supports vertical and horizontal scrollbars
- Scrollbar visibility modes: hover, scroll, auto, always
- Accessible via keyboard scrolling

## Installation

```toml
[dependencies]
leptix-scroll-area = "1.0.0"
```

## Anatomy

```rust
use leptix_scroll_area::*;

view! {
    <ScrollArea>
        <ScrollAreaViewport>
            // scrollable content
        </ScrollAreaViewport>
        <ScrollAreaScrollbar orientation="vertical".into()>
            <ScrollAreaThumb />
        </ScrollAreaScrollbar>
        <ScrollAreaScrollbar orientation="horizontal".into()>
            <ScrollAreaThumb />
        </ScrollAreaScrollbar>
        <ScrollAreaCorner />
    </ScrollArea>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_scroll_area::*;

#[component]
fn TagList() -> impl IntoView {
    let tags = vec![
        "Rust", "Leptos", "WebAssembly", "Reactive", "SSR",
        "Hydration", "Signals", "Components", "Primitives",
        "Accessibility", "TypeScript", "CSS", "HTML", "ARIA",
        "WAI", "Radix", "Leptix", "Frontend", "Backend", "Fullstack",
    ];

    view! {
        <ScrollArea attr:class="scroll-root" attr:style="height: 200px; width: 200px;">
            <ScrollAreaViewport attr:class="scroll-viewport">
                <div style="padding: 15px 20px;">
                    <p style="font-weight: 500; margin-bottom: 10px;">"Tags"</p>
                    {tags.into_iter().map(|tag| view! {
                        <div style="margin-top: 10px; padding: 4px 8px; border-radius: 3px; background: #f0f0f0; font-size: 13px;">
                            {tag}
                        </div>
                    }).collect_view()}
                </div>
            </ScrollAreaViewport>
            <ScrollAreaScrollbar attr:class="scroll-scrollbar" orientation="vertical".into()>
                <ScrollAreaThumb attr:class="scroll-thumb" />
            </ScrollAreaScrollbar>
            <ScrollAreaCorner />
        </ScrollArea>
    }
}
```

## Styling

```css
.scroll-root {
    border-radius: 4px;
    overflow: hidden;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    background: white;
}

.scroll-viewport {
    width: 100%;
    height: 100%;
    border-radius: inherit;
}

.scroll-scrollbar {
    display: flex;
    user-select: none;
    touch-action: none;
    padding: 2px;
    background: rgba(0, 0, 0, 0.05);
    transition: background 160ms ease-out;
}

.scroll-scrollbar:hover {
    background: rgba(0, 0, 0, 0.1);
}

.scroll-scrollbar[data-orientation="vertical"] {
    width: 10px;
}

.scroll-scrollbar[data-orientation="horizontal"] {
    flex-direction: column;
    height: 10px;
}

.scroll-thumb {
    flex: 1;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 10px;
    position: relative;
}

.scroll-thumb::before {
    content: "";
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 100%;
    height: 100%;
    min-width: 44px;
    min-height: 44px;
}
```

## API Reference

### ScrollArea

The root container. Applies `overflow: hidden` and relative positioning.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `r#type` | `Option<ScrollAreaType>` | `Hover` | Scrollbar visibility mode. One of `Hover`, `Scroll`, `Auto`, `Always`. |
| `dir` | `MaybeProp<String>` | `"ltr"` | The reading direction. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element instead of rendering a default `<div>`. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### ScrollAreaViewport

The scrollable viewport. Contains the actual scrollable content.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-leptix-scroll-area-viewport]` | Always present. |

### ScrollAreaScrollbar

The scrollbar track. Position depends on orientation.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `orientation` | `MaybeProp<String>` | `"vertical"` | The orientation of the scrollbar: `"vertical"` or `"horizontal"`. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"vertical"` \| `"horizontal"` |

### ScrollAreaThumb

The draggable scrollbar thumb.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

| Data Attribute | Values |
|---------------|--------|
| `[data-leptix-scroll-area-thumb]` | Always present. |

### ScrollAreaCorner

The corner element rendered where vertical and horizontal scrollbars meet.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Data Attributes

| Data Attribute | Values |
|---------------|--------|
| `[data-orientation]` | `"vertical"` \| `"horizontal"` -- present on scrollbar. |
| `[data-leptix-scroll-area-viewport]` | Present on viewport. |
| `[data-leptix-scroll-area-thumb]` | Present on thumb. |

## Accessibility

The scrollbar uses `role="scrollbar"` with the appropriate `aria-orientation`. The viewport is focusable via `tabindex="0"` to allow keyboard scrolling.

### Keyboard Interactions

| Key | Description |
|-----|-------------|
| `ArrowDown` | Scrolls down when the viewport is focused. |
| `ArrowUp` | Scrolls up when the viewport is focused. |
| `ArrowRight` | Scrolls right when the viewport is focused. |
| `ArrowLeft` | Scrolls left when the viewport is focused. |
| `Page Down` | Scrolls down by one page. |
| `Page Up` | Scrolls up by one page. |
| `Home` | Scrolls to the top. |
| `End` | Scrolls to the bottom. |
