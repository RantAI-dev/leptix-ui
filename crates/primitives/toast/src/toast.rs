use std::sync::Arc;

use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

const SWIPE_THRESHOLD_PX: f64 = 100.0;

fn window() -> web_sys::Window {
    web_sys::window().expect("should have a Window")
}

fn set_timeout(callback: &Closure<dyn Fn()>, ms: i32) -> i32 {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            ms,
        )
        .expect("set_timeout should succeed")
}

fn clear_timeout(id: i32) {
    window().clear_timeout_with_handle(id);
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ToastProviderContextValue {
    toasts: RwSignal<Vec<ToastEntry>>,
    add_toast: Callback<ToastEntry>,
    remove_toast: Callback<String>,
    swipe_direction: Signal<String>,
    default_duration: Signal<i32>,
}

#[derive(Clone, Debug)]
struct ToastEntry {
    id: String,
    open: RwSignal<bool>,
    /// When true, the auto-close timer is paused.
    paused: RwSignal<bool>,
}

#[component]
pub fn ToastProvider(
    #[prop(into, optional, default = 5000.into())] duration: MaybeProp<i32>,
    #[prop(into, optional)] swipe_direction: MaybeProp<String>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let default_duration = Signal::derive(move || duration.get().unwrap_or(5000));
    let swipe_dir = Signal::derive(move || swipe_direction.get().unwrap_or("right".into()));
    let toasts: RwSignal<Vec<ToastEntry>> = RwSignal::new(vec![]);

    let ctx = ToastProviderContextValue {
        toasts,
        add_toast: Callback::new(move |entry: ToastEntry| {
            toasts.update(|t| t.push(entry));
        }),
        remove_toast: Callback::new(move |id: String| {
            toasts.update(|t| t.retain(|e| e.id != id));
        }),
        swipe_direction: swipe_dir,
        default_duration,
    };

    view! { <Provider value=ctx>{children.with_value(|c| c())}</Provider> }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ToastContextValue {
    id: String,
    open: RwSignal<bool>,
    on_close: Callback<()>,
    duration: Signal<i32>,
    timer_id: RwSignal<Option<i32>>,
}

impl ToastContextValue {
    fn clear_timer(&self) {
        if let Some(id) = self.timer_id.get_untracked() {
            clear_timeout(id);
            self.timer_id.set(None);
        }
    }
}

#[component]
pub fn Toast(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] duration: MaybeProp<i32>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let provider_ctx = use_context::<ToastProviderContextValue>();
    let duration = Signal::derive(move || {
        duration.get().unwrap_or_else(|| {
            provider_ctx
                .as_ref()
                .map(|ctx| ctx.default_duration.get())
                .unwrap_or(5000)
        })
    });
    let open_state = RwSignal::new(open.get().or(default_open.get()).unwrap_or(true));

    // Get swipe direction from provider context (default "right")
    let provider_ctx = use_context::<ToastProviderContextValue>();
    let swipe_direction = Signal::derive(move || {
        provider_ctx
            .as_ref()
            .map(|ctx| ctx.swipe_direction.get())
            .unwrap_or_else(|| "right".to_string())
    });

    Effect::new(move |_| {
        if let Some(o) = open.get() {
            open_state.set(o);
        }
    });

    let id = leptix_core::id::use_id(None).get();
    let timer_id: RwSignal<Option<i32>> = RwSignal::new(None);

    let on_close_cb = Callback::new(move |()| {
        open_state.set(false);
        if let Some(cb) = on_open_change {
            cb.run(false);
        }
    });

    let toast_ctx = ToastContextValue {
        id: id.clone(),
        open: open_state,
        on_close: on_close_cb,
        duration,
        timer_id,
    };

    // Self-register with the provider so "pause all" / "dismiss all" is possible.
    let provider_for_reg = use_context::<ToastProviderContextValue>();
    let paused = RwSignal::new(false);
    if let Some(ref pctx) = provider_for_reg {
        pctx.add_toast.run(ToastEntry {
            id: id.clone(),
            open: open_state,
            paused,
        });
    }
    {
        let id_for_cleanup = id.clone();
        on_cleanup(move || {
            if let Some(ref pctx) = provider_for_reg {
                pctx.remove_toast.run(id_for_cleanup.clone());
            }
        });
    }

    // Build the auto-close closure once, wrapped for Send+Sync
    let close_closure: Arc<SendWrapper<Closure<dyn Fn()>>> =
        Arc::new(SendWrapper::new(Closure::new(move || {
            on_close_cb.run(());
        })));

    // Start auto-close timer when open becomes true; pause/resume via `paused` signal.
    let close_closure_for_effect = close_closure.clone();
    Effect::new(move |_| {
        let is_open = open_state.get();
        let is_paused = paused.get();
        if is_open && !is_paused {
            // Clear any existing timer
            if let Some(id) = timer_id.get_untracked() {
                clear_timeout(id);
            }
            let dur = duration.get_untracked();
            if dur > 0 {
                let id = set_timeout(&close_closure_for_effect, dur);
                timer_id.set(Some(id));
            }
        } else if is_paused {
            // Paused — clear timer but keep toast open
            if let Some(id) = timer_id.get_untracked() {
                clear_timeout(id);
                timer_id.set(None);
            }
        } else {
            // Toast closed, clear timer
            if let Some(id) = timer_id.get_untracked() {
                clear_timeout(id);
                timer_id.set(None);
            }
        }
    });

    // Clean up on unmount
    on_cleanup(move || {
        if let Some(id) = timer_id.try_get_untracked().flatten() {
            clear_timeout(id);
        }
    });

    let present = Signal::derive(move || open_state.get());
    let presence = use_presence(present);

    // Store the close closure in a StoredValue so it can be accessed from Fn closures
    let close_closure_stored = StoredValue::new(close_closure);

    // --- Swipe state ---
    let swipe_start_x = RwSignal::new(0.0f64);
    let swipe_start_y = RwSignal::new(0.0f64);
    let swipe_delta_x = RwSignal::new(0.0f64);
    let swipe_delta_y = RwSignal::new(0.0f64);
    let is_swiping = RwSignal::new(false);
    let swipe_data_attr: RwSignal<Option<&'static str>> = RwSignal::new(None);

    view! {
        <Provider value=toast_ctx>
            <Show when=move || presence.is_present.get()>
                <Primitive element=html::li as_child=as_child node_ref=node_ref
                    attr:role="status"
                    attr:aria-live="off"
                    attr:aria-atomic="true"
                    attr:data-state=move || if open_state.get() { "open" } else { "closed" }
                    attr:data-swipe=move || swipe_data_attr.get()
                    attr:tabindex="0"
                    style:transform=move || {
                        if !is_swiping.get() {
                            return String::new();
                        }
                        let dx = swipe_delta_x.get();
                        let dy = swipe_delta_y.get();
                        let dir = swipe_direction.get();
                        match dir.as_str() {
                            "right" => format!("translateX({}px)", dx.max(0.0)),
                            "left" => format!("translateX({}px)", dx.min(0.0)),
                            "up" => format!("translateY({}px)", dy.min(0.0)),
                            "down" => format!("translateY({}px)", dy.max(0.0)),
                            _ => String::new(),
                        }
                    }
                    style:user-select=move || {
                        if is_swiping.get() { "none" } else { "" }
                    }
                    style:touch-action=move || {
                        match swipe_direction.get().as_str() {
                            "right" | "left" => "pan-y",
                            "up" | "down" => "pan-x",
                            _ => "auto",
                        }
                    }
                    on:pointerdown=move |event: leptos::ev::PointerEvent| {
                        swipe_start_x.set(event.client_x() as f64);
                        swipe_start_y.set(event.client_y() as f64);
                        swipe_delta_x.set(0.0);
                        swipe_delta_y.set(0.0);
                        is_swiping.set(true);
                        swipe_data_attr.set(Some("start"));
                        // Capture pointer for reliable tracking
                        if let Some(target) = event.target()
                            && let Ok(el) = target.dyn_into::<web_sys::Element>()
                        {
                            let _ = el.set_pointer_capture(event.pointer_id());
                        }
                        // Pause auto-close timer during swipe
                        if let Some(id) = timer_id.get_untracked() {
                            clear_timeout(id);
                            timer_id.set(None);
                        }
                    }
                    on:pointermove=move |event: leptos::ev::PointerEvent| {
                        if !is_swiping.get_untracked() {
                            return;
                        }
                        let dx = event.client_x() as f64 - swipe_start_x.get_untracked();
                        let dy = event.client_y() as f64 - swipe_start_y.get_untracked();
                        swipe_delta_x.set(dx);
                        swipe_delta_y.set(dy);
                        // Only set "move" if the swipe is in the configured direction
                        let dir = swipe_direction.get_untracked();
                        let is_valid = match dir.as_str() {
                            "right" => dx > 0.0,
                            "left" => dx < 0.0,
                            "up" => dy < 0.0,
                            "down" => dy > 0.0,
                            _ => false,
                        };
                        if is_valid {
                            swipe_data_attr.set(Some("move"));
                        }
                    }
                    on:pointerup=move |event: leptos::ev::PointerEvent| {
                        if !is_swiping.get_untracked() {
                            return;
                        }
                        // Release pointer capture
                        if let Some(target) = event.target()
                            && let Ok(el) = target.dyn_into::<web_sys::Element>()
                        {
                            let _ = el.release_pointer_capture(event.pointer_id());
                        }
                        let dx = swipe_delta_x.get_untracked();
                        let dy = swipe_delta_y.get_untracked();
                        let dir = swipe_direction.get_untracked();
                        let exceeded = match dir.as_str() {
                            "right" => dx > SWIPE_THRESHOLD_PX,
                            "left" => dx < -SWIPE_THRESHOLD_PX,
                            "up" => dy < -SWIPE_THRESHOLD_PX,
                            "down" => dy > SWIPE_THRESHOLD_PX,
                            _ => false,
                        };
                        if exceeded {
                            // Swipe completed — dismiss
                            swipe_data_attr.set(Some("end"));
                            on_close_cb.run(());
                        } else {
                            // Snap back
                            swipe_data_attr.set(Some("cancel"));
                        }
                        // Reset swipe state
                        is_swiping.set(false);
                        swipe_delta_x.set(0.0);
                        swipe_delta_y.set(0.0);
                        // Resume auto-close timer if still open
                        let dur = duration.get_untracked();
                        if dur > 0 && open_state.get_untracked() {
                            close_closure_stored.with_value(|cls| {
                                let id = set_timeout(cls, dur);
                                timer_id.set(Some(id));
                            });
                        }
                    }
                    on:pointerenter=move |_: leptos::ev::PointerEvent| {
                        // Pause: clear auto-close timer (only if not swiping, since swipe already pauses)
                        if !is_swiping.get_untracked()
                            && let Some(id) = timer_id.get_untracked()
                        {
                            clear_timeout(id);
                            timer_id.set(None);
                        }
                    }
                    on:pointerleave=move |_: leptos::ev::PointerEvent| {
                        // Resume: restart auto-close timer (only if not swiping)
                        if !is_swiping.get_untracked() {
                            let dur = duration.get_untracked();
                            if dur > 0 && open_state.get_untracked() {
                                close_closure_stored.with_value(|cls| {
                                    let id = set_timeout(cls, dur);
                                    timer_id.set(Some(id));
                                });
                            }
                        }
                    }
                >
                    {children.with_value(|c| c())}
                </Primitive>
            </Show>
        </Provider>
    }
}

#[component]
pub fn ToastTitle(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! { <Primitive element=html::div as_child=as_child node_ref=node_ref>{children.with_value(|c| c())}</Primitive> }
}

#[component]
pub fn ToastDescription(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! { <Primitive element=html::div as_child=as_child node_ref=node_ref>{children.with_value(|c| c())}</Primitive> }
}

#[component]
pub fn ToastAction(
    #[prop(into)] alt_text: String,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            attr:aria-label=alt_text
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ToastClose(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ToastContextValue>();

    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            on:click=move |_| {
                ctx.clear_timer();
                ctx.on_close.run(());
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ToastViewport(
    #[prop(into, optional)] label: MaybeProp<String>,
    /// Keyboard shortcut(s) to focus the viewport. Defaults to `["F8"]`.
    #[prop(into, optional)]
    hotkey: Option<Vec<String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let label = Signal::derive(move || label.get().unwrap_or("Notifications".into()));
    let hotkeys = hotkey.unwrap_or_else(|| vec!["F8".into()]);
    let viewport_ref = AnyNodeRef::new();
    let refs = leptix_core::compose_refs::use_composed_refs(vec![node_ref, viewport_ref]);

    // Register global hotkey listener to focus viewport
    let hotkeys = StoredValue::new(hotkeys);
    Effect::new(move |_| {
        // Attach document-level keydown for hotkey
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let handler =
                web_sys::wasm_bindgen::closure::Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
                    move |event: web_sys::KeyboardEvent| {
                        let keys = hotkeys.get_value();
                        if keys.iter().any(|k| k.eq_ignore_ascii_case(&event.key()))
                            && let Some(el) = viewport_ref.get()
                        {
                            use web_sys::wasm_bindgen::JsCast;
                            if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
                                let _ = html_el.focus();
                            }
                        }
                    },
                );
            let _ = document
                .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
            // Leak the closure to keep it alive for the document listener lifetime.
            // This is acceptable for a global singleton viewport.
            handler.forget();
        }
    });

    // Access provider registry for pause-all / resume-all on viewport focus.
    // Extract the signal (Copy) so closures can share it without move conflicts.
    let toasts_signal = use_context::<ToastProviderContextValue>().map(|ctx| ctx.toasts);

    let set_all_paused = move |paused: bool| {
        if let Some(toasts) = toasts_signal {
            toasts.with_untracked(|list| {
                for toast in list {
                    toast.paused.set(paused);
                }
            });
        }
    };

    view! {
        <Primitive element=html::ol as_child=as_child node_ref=refs
            attr:role="region"
            attr:aria-label=move || label.get()
            attr:tabindex="-1"
            on:focusin=move |_| set_all_paused(true)
            on:focusout=move |_| set_all_paused(false)
            on:pointerenter=move |_| set_all_paused(true)
            on:pointerleave=move |_| set_all_paused(false)
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}
