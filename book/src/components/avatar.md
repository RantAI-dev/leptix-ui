# Avatar

An image element with a fallback for representing the user.

## Features

- Automatic and manual control of image loading state
- Fallback rendering when the image fails to load
- Optional delay on fallback to avoid content flashing
- Supports custom loading status change callbacks

## Installation

```toml
[dependencies]
leptix-avatar = "1.0.0"
```

## Anatomy

```rust
use leptix_avatar::*;

view! {
    <Avatar>
        <AvatarImage />
        <AvatarFallback />
    </Avatar>
}
```

## Example

```rust
use leptos::prelude::*;
use leptix_avatar::*;

#[component]
fn UserAvatar() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 16px;">
            // Avatar with image
            <Avatar attr:class="avatar-root">
                <AvatarImage
                    src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?w=128&h=128&fit=crop"
                    attr:alt="Jane Doe"
                />
                <AvatarFallback attr:class="avatar-fallback" delay_ms=600>
                    "JD"
                </AvatarFallback>
            </Avatar>

            // Avatar with fallback only
            <Avatar attr:class="avatar-root">
                <AvatarFallback attr:class="avatar-fallback">
                    "AB"
                </AvatarFallback>
            </Avatar>
        </div>
    }
}
```

## Styling

```css
.avatar-root {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    vertical-align: middle;
    overflow: hidden;
    user-select: none;
    width: 45px;
    height: 45px;
    border-radius: 100%;
}

.avatar-root img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: inherit;
}

.avatar-fallback {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #e5e7eb;
    color: #374151;
    font-size: 15px;
    font-weight: 500;
    line-height: 1;
}
```

## API Reference

### Avatar

Contains all the parts of an avatar.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### AvatarImage

The image to render. By default it will only render when it has loaded.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `src` | `MaybeProp<String>` | - | The image source URL. |
| `on_loading_status_change` | `Option<Callback<ImageLoadingStatus>>` | - | A callback providing information about the loading status of the image. Receives `Idle`, `Loading`, `Loaded`, or `Error`. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

### AvatarFallback

An element that renders when the image has not loaded. This means whilst it is loading, or if there was an error. You can use an optional delay to prevent content flashing.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `delay_ms` | `MaybeProp<i32>` | - | Useful for delaying rendering so that it only appears for those with slower connections. In milliseconds. |
| `as_child` | `MaybeProp<bool>` | `false` | Merge props onto the child element. |
| `node_ref` | `AnyNodeRef` | - | A ref to the underlying DOM element. |

## Accessibility

Adheres to the [Image WAI-ARIA design pattern](https://www.w3.org/WAI/tutorials/images/decorative/).

### Keyboard Interactions

Not applicable -- avatars are not interactive elements.
