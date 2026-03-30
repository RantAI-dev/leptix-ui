use std::sync::{Arc, Mutex};

use leptix_core::primitive::Primitive;
use leptos::{context::Provider, ev::PointerEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScrollAreaType {
    #[default]
    Hover,
    Scroll,
    Auto,
    Always,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ScrollAreaContextValue {
    scroll_type: ScrollAreaType,
    dir: Signal<String>,
    viewport_ref: AnyNodeRef,
    scroll_top: ReadSignal<f64>,
    set_scroll_top: WriteSignal<f64>,
    scroll_left: ReadSignal<f64>,
    set_scroll_left: WriteSignal<f64>,
    viewport_height: ReadSignal<f64>,
    set_viewport_height: WriteSignal<f64>,
    viewport_width: ReadSignal<f64>,
    set_viewport_width: WriteSignal<f64>,
    content_height: ReadSignal<f64>,
    set_content_height: WriteSignal<f64>,
    content_width: ReadSignal<f64>,
    set_content_width: WriteSignal<f64>,
}

#[component]
pub fn ScrollArea(
    #[prop(into, optional)] r#type: Option<ScrollAreaType>,
    #[prop(into, optional)] dir: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let scroll_type = r#type.unwrap_or_default();
    let dir = Signal::derive(move || dir.get().unwrap_or("ltr".into()));

    let (scroll_top, set_scroll_top) = signal(0.0_f64);
    let (scroll_left, set_scroll_left) = signal(0.0_f64);
    let (viewport_height, set_viewport_height) = signal(0.0_f64);
    let (viewport_width, set_viewport_width) = signal(0.0_f64);
    let (content_height, set_content_height) = signal(0.0_f64);
    let (content_width, set_content_width) = signal(0.0_f64);

    let ctx = ScrollAreaContextValue {
        scroll_type,
        dir,
        viewport_ref: AnyNodeRef::new(),
        scroll_top,
        set_scroll_top,
        scroll_left,
        set_scroll_left,
        viewport_height,
        set_viewport_height,
        viewport_width,
        set_viewport_width,
        content_height,
        set_content_height,
        content_width,
        set_content_width,
    };

    view! {
        <Provider value=ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:dir=move || dir.get()
                attr:style="position:relative;overflow:hidden;height:100%;width:100%"
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ScrollAreaViewport(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ScrollAreaContextValue>();
    let refs = leptix_core::compose_refs::use_composed_refs(vec![node_ref, ctx.viewport_ref]);

    // Set up a ResizeObserver to track viewport and content dimensions.
    let resize_observer: Arc<Mutex<Option<SendWrapper<web_sys::ResizeObserver>>>> =
        Arc::new(Mutex::new(None));
    let cleanup_resize_observer = resize_observer.clone();

    let set_viewport_height = ctx.set_viewport_height;
    let set_viewport_width = ctx.set_viewport_width;
    let set_content_height = ctx.set_content_height;
    let set_content_width = ctx.set_content_width;
    let set_scroll_top = ctx.set_scroll_top;
    let set_scroll_left = ctx.set_scroll_left;
    let viewport_ref = ctx.viewport_ref;

    Effect::new(move |_| {
        if let Some(el) = viewport_ref
            .get()
            .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
        {
            // Read initial dimensions.
            set_viewport_height.set(el.client_height() as f64);
            set_viewport_width.set(el.client_width() as f64);
            set_content_height.set(el.scroll_height() as f64);
            set_content_width.set(el.scroll_width() as f64);
            set_scroll_top.set(el.scroll_top() as f64);
            set_scroll_left.set(el.scroll_left() as f64);

            let resize_closure: Closure<dyn Fn(Vec<web_sys::ResizeObserverEntry>)> =
                Closure::new(move |_entries: Vec<web_sys::ResizeObserverEntry>| {
                    if let Some(vp) = viewport_ref
                        .get()
                        .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                    {
                        set_viewport_height.set(vp.client_height() as f64);
                        set_viewport_width.set(vp.client_width() as f64);
                        set_content_height.set(vp.scroll_height() as f64);
                        set_content_width.set(vp.scroll_width() as f64);
                    }
                });

            if let Ok(observer) =
                web_sys::ResizeObserver::new(resize_closure.into_js_value().unchecked_ref())
            {
                // Observe the viewport element itself.
                observer.observe(el.as_ref());

                // Also observe the first child (content wrapper) if it exists.
                if let Some(first_child) = el.first_element_child() {
                    observer.observe(&first_child);
                }

                *resize_observer.lock().expect("Lock should be acquired.") =
                    Some(SendWrapper::new(observer));
            }
        }
    });

    on_cleanup(move || {
        if let Some(observer) = cleanup_resize_observer
            .lock()
            .expect("Lock should be acquired.")
            .as_ref()
        {
            observer.disconnect();
        }
    });

    view! {
        <Primitive element=html::div as_child=as_child node_ref=refs
            attr:data-leptix-scroll-area-viewport=""
            attr:style="overflow:scroll;height:100%;width:100%;scrollbar-width:none;-ms-overflow-style:none"
            attr:tabindex="0"
            on:scroll=move |_| {
                if let Some(el) = viewport_ref
                    .get()
                    .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                {
                    set_scroll_top.set(el.scroll_top() as f64);
                    set_scroll_left.set(el.scroll_left() as f64);
                    // Also update content/viewport dimensions in case they changed.
                    set_content_height.set(el.scroll_height() as f64);
                    set_content_width.set(el.scroll_width() as f64);
                    set_viewport_height.set(el.client_height() as f64);
                    set_viewport_width.set(el.client_width() as f64);
                }
            }
        >
            <div style="min-width:100%;display:table">
                {children.with_value(|c| c())}
            </div>
        </Primitive>
    }
}

/// Inject a <style> tag to hide webkit scrollbars. This is needed because
/// `::-webkit-scrollbar { display: none }` cannot be set via inline styles.
#[component]
fn ScrollAreaStyle() -> impl IntoView {
    view! {
        <style>
            {"[data-leptix-scroll-area-viewport]::-webkit-scrollbar{display:none}"}
        </style>
    }
}

#[derive(Clone, Debug)]
struct ScrollbarContextValue {
    orientation: Signal<String>,
}

#[component]
pub fn ScrollAreaScrollbar(
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let ctx = expect_context::<ScrollAreaContextValue>();
    let orientation = Signal::derive(move || orientation.get().unwrap_or("vertical".into()));

    let has_overflow = Memo::new(move |_| {
        let ori = orientation.get();
        if ori == "vertical" {
            ctx.content_height.get() > ctx.viewport_height.get()
        } else {
            ctx.content_width.get() > ctx.viewport_width.get()
        }
    });

    let should_show = Memo::new(move |_| match ctx.scroll_type {
        ScrollAreaType::Always => true,
        ScrollAreaType::Auto => has_overflow.get(),
        ScrollAreaType::Scroll => has_overflow.get(),
        ScrollAreaType::Hover => has_overflow.get(),
    });

    let scrollbar_ctx = ScrollbarContextValue { orientation };

    view! {
        <Provider value=scrollbar_ctx>
            <ScrollAreaStyle />
            <Show when=move || should_show.get()>
                <Primitive element=html::div as_child=as_child node_ref=node_ref
                    attr:role="scrollbar"
                    attr:aria-orientation=move || orientation.get()
                    attr:data-orientation=move || orientation.get()
                    attr:aria-valuenow=move || {
                        let ori = orientation.get();
                        if ori == "vertical" {
                            let max_scroll = ctx.content_height.get() - ctx.viewport_height.get();
                            if max_scroll > 0.0 {
                                ((ctx.scroll_top.get() / max_scroll) * 100.0).round().to_string()
                            } else {
                                "0".to_string()
                            }
                        } else {
                            let max_scroll = ctx.content_width.get() - ctx.viewport_width.get();
                            if max_scroll > 0.0 {
                                ((ctx.scroll_left.get() / max_scroll) * 100.0).round().to_string()
                            } else {
                                "0".to_string()
                            }
                        }
                    }
                    attr:aria-valuemin="0"
                    attr:aria-valuemax="100"
                    attr:style=move || {
                        if orientation.get() == "vertical" {
                            "position:absolute;top:0;right:0;bottom:0;width:8px;z-index:1"
                        } else {
                            "position:absolute;bottom:0;left:0;right:0;height:8px;z-index:1"
                        }
                    }
                >
                    {children.with_value(|c| c.as_ref().map(|c| c()))}
                </Primitive>
            </Show>
        </Provider>
    }
}

#[component]
pub fn ScrollAreaThumb(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let ctx = expect_context::<ScrollAreaContextValue>();
    let scrollbar_ctx = expect_context::<ScrollbarContextValue>();

    let (dragging, set_dragging) = signal(false);
    let (drag_start, set_drag_start) = signal(0.0_f64);
    let (scroll_start, set_scroll_start) = signal(0.0_f64);

    let orientation = scrollbar_ctx.orientation;

    // Compute thumb size as a percentage.
    let thumb_size_pct = Memo::new(move |_| {
        let ori = orientation.get();
        if ori == "vertical" {
            let vh = ctx.viewport_height.get();
            let ch = ctx.content_height.get();
            if ch > 0.0 {
                ((vh / ch) * 100.0).clamp(5.0, 100.0)
            } else {
                100.0
            }
        } else {
            let vw = ctx.viewport_width.get();
            let cw = ctx.content_width.get();
            if cw > 0.0 {
                ((vw / cw) * 100.0).clamp(5.0, 100.0)
            } else {
                100.0
            }
        }
    });

    // Compute thumb offset as a percentage of the scrollbar track.
    let thumb_offset_pct = Memo::new(move |_| {
        let ori = orientation.get();
        let thumb_pct = thumb_size_pct.get();
        let track_pct = 100.0 - thumb_pct;

        if ori == "vertical" {
            let max_scroll = ctx.content_height.get() - ctx.viewport_height.get();
            if max_scroll > 0.0 {
                (ctx.scroll_top.get() / max_scroll) * track_pct
            } else {
                0.0
            }
        } else {
            let max_scroll = ctx.content_width.get() - ctx.viewport_width.get();
            if max_scroll > 0.0 {
                (ctx.scroll_left.get() / max_scroll) * track_pct
            } else {
                0.0
            }
        }
    });

    // Store closures for global pointermove/pointerup during drag.
    type PointerClosure = Arc<Mutex<Option<SendWrapper<Closure<dyn Fn(web_sys::PointerEvent)>>>>>;
    let move_closure: PointerClosure = Arc::new(Mutex::new(None));
    let up_closure: PointerClosure = Arc::new(Mutex::new(None));
    let cleanup_move = move_closure.clone();
    let cleanup_up = up_closure.clone();

    // Set up global pointermove and pointerup listeners when dragging starts.
    let viewport_ref = ctx.viewport_ref;
    Effect::new(move |_| {
        if !dragging.get() {
            // Clean up listeners when dragging stops.
            let document = web_sys::window().and_then(|w| w.document());
            if let Some(doc) = &document {
                if let Some(closure) = move_closure.lock().unwrap().take() {
                    let _ = doc.remove_event_listener_with_callback(
                        "pointermove",
                        closure.as_ref().unchecked_ref(),
                    );
                }
                if let Some(closure) = up_closure.lock().unwrap().take() {
                    let _ = doc.remove_event_listener_with_callback(
                        "pointerup",
                        closure.as_ref().unchecked_ref(),
                    );
                }
            }
            return;
        }

        let document = match web_sys::window().and_then(|w| w.document()) {
            Some(d) => d,
            None => return,
        };

        let ori = orientation.get_untracked();
        let move_closure_ref = move_closure.clone();
        let up_closure_ref = up_closure.clone();

        // pointermove handler
        let move_cb: Closure<dyn Fn(web_sys::PointerEvent)> =
            Closure::new(move |event: web_sys::PointerEvent| {
                let current_pos = if ori == "vertical" {
                    event.client_y() as f64
                } else {
                    event.client_x() as f64
                };
                let delta = current_pos - drag_start.get_untracked();

                if let Some(vp) = viewport_ref
                    .get_untracked()
                    .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                {
                    if ori == "vertical" {
                        let track_height = vp.client_height() as f64;
                        let content_height = vp.scroll_height() as f64;
                        let viewport_height = vp.client_height() as f64;
                        let max_scroll = content_height - viewport_height;
                        let thumb_ratio = viewport_height / content_height;
                        let thumb_height = track_height * thumb_ratio;
                        let scroll_track = track_height - thumb_height;

                        if scroll_track > 0.0 {
                            let scroll_delta = (delta / scroll_track) * max_scroll;
                            let new_scroll = (scroll_start.get_untracked() + scroll_delta)
                                .clamp(0.0, max_scroll);
                            vp.set_scroll_top(new_scroll as i32);
                        }
                    } else {
                        let track_width = vp.client_width() as f64;
                        let content_width = vp.scroll_width() as f64;
                        let viewport_width = vp.client_width() as f64;
                        let max_scroll = content_width - viewport_width;
                        let thumb_ratio = viewport_width / content_width;
                        let thumb_width = track_width * thumb_ratio;
                        let scroll_track = track_width - thumb_width;

                        if scroll_track > 0.0 {
                            let scroll_delta = (delta / scroll_track) * max_scroll;
                            let new_scroll = (scroll_start.get_untracked() + scroll_delta)
                                .clamp(0.0, max_scroll);
                            vp.set_scroll_left(new_scroll as i32);
                        }
                    }
                }
            });

        let _ = document
            .add_event_listener_with_callback("pointermove", move_cb.as_ref().unchecked_ref());
        *move_closure_ref.lock().unwrap() = Some(SendWrapper::new(move_cb));

        // pointerup handler
        let up_move_closure = move_closure_ref.clone();
        let up_up_closure = up_closure_ref.clone();
        let up_cb: Closure<dyn Fn(web_sys::PointerEvent)> =
            Closure::new(move |_event: web_sys::PointerEvent| {
                set_dragging.set(false);

                // Eagerly remove listeners.
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    if let Some(closure) = up_move_closure.lock().unwrap().take() {
                        let _ = doc.remove_event_listener_with_callback(
                            "pointermove",
                            closure.as_ref().unchecked_ref(),
                        );
                    }
                    if let Some(closure) = up_up_closure.lock().unwrap().take() {
                        let _ = doc.remove_event_listener_with_callback(
                            "pointerup",
                            closure.as_ref().unchecked_ref(),
                        );
                    }
                }
            });

        let _ =
            document.add_event_listener_with_callback("pointerup", up_cb.as_ref().unchecked_ref());
        *up_closure_ref.lock().unwrap() = Some(SendWrapper::new(up_cb));
    });

    on_cleanup(move || {
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(closure) = cleanup_move.lock().unwrap().take() {
                let _ = doc.remove_event_listener_with_callback(
                    "pointermove",
                    closure.as_ref().unchecked_ref(),
                );
            }
            if let Some(closure) = cleanup_up.lock().unwrap().take() {
                let _ = doc.remove_event_listener_with_callback(
                    "pointerup",
                    closure.as_ref().unchecked_ref(),
                );
            }
        }
    });

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:data-leptix-scroll-area-thumb=""
            attr:style=move || {
                let ori = orientation.get();
                let size = thumb_size_pct.get();
                let offset = thumb_offset_pct.get();

                if ori == "vertical" {
                    format!(
                        "position:relative;border-radius:9999px;background:rgba(0,0,0,0.3);\
                         width:100%;height:{size:.2}%;top:{offset:.2}%;\
                         cursor:pointer;user-select:none;touch-action:none"
                    )
                } else {
                    format!(
                        "position:relative;border-radius:9999px;background:rgba(0,0,0,0.3);\
                         height:100%;width:{size:.2}%;left:{offset:.2}%;\
                         cursor:pointer;user-select:none;touch-action:none"
                    )
                }
            }
            on:pointerdown=move |event: PointerEvent| {
                event.prevent_default();

                let ori = orientation.get();
                let pos = if ori == "vertical" {
                    event.client_y() as f64
                } else {
                    event.client_x() as f64
                };

                set_drag_start.set(pos);

                // Record current scroll position at drag start.
                if let Some(vp) = viewport_ref
                    .get()
                    .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
                {
                    if ori == "vertical" {
                        set_scroll_start.set(vp.scroll_top() as f64);
                    } else {
                        set_scroll_start.set(vp.scroll_left() as f64);
                    }
                }

                // Capture pointer on the target element.
                if let Some(target) = event.current_target()
                    && let Ok(el) = target.dyn_into::<web_sys::Element>()
                {
                    let _ = el.set_pointer_capture(event.pointer_id());
                }

                set_dragging.set(true);
            }
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}

#[component]
pub fn ScrollAreaCorner(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:style="position:absolute;right:0;bottom:0"
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}
