use std::sync::Arc;

use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

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
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ToastEntry {
    id: String,
    open: RwSignal<bool>,
}

#[component]
pub fn ToastProvider(
    #[prop(into, optional, default = 5000.into())] duration: MaybeProp<i32>,
    #[prop(into, optional)] swipe_direction: MaybeProp<String>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let _duration = Signal::derive(move || duration.get().unwrap_or(5000));
    let _swipe = Signal::derive(move || swipe_direction.get().unwrap_or("right".into()));
    let toasts: RwSignal<Vec<ToastEntry>> = RwSignal::new(vec![]);

    let ctx = ToastProviderContextValue {
        toasts,
        add_toast: Callback::new(move |entry: ToastEntry| {
            toasts.update(|t| t.push(entry));
        }),
        remove_toast: Callback::new(move |id: String| {
            toasts.update(|t| t.retain(|e| e.id != id));
        }),
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
    let duration = Signal::derive(move || duration.get().unwrap_or(5000));
    let open_state = RwSignal::new(open.get().or(default_open.get()).unwrap_or(true));

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

    // Build the auto-close closure once, wrapped for Send+Sync
    let close_closure: Arc<SendWrapper<Closure<dyn Fn()>>> =
        Arc::new(SendWrapper::new(Closure::new(move || {
            on_close_cb.run(());
        })));

    // Start auto-close timer when open becomes true
    let close_closure_for_effect = close_closure.clone();
    Effect::new(move |_| {
        let is_open = open_state.get();
        if is_open {
            // Clear any existing timer
            if let Some(id) = timer_id.get_untracked() {
                clear_timeout(id);
            }
            let dur = duration.get_untracked();
            if dur > 0 {
                let id = set_timeout(&close_closure_for_effect, dur);
                timer_id.set(Some(id));
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

    view! {
        <Provider value=toast_ctx>
            <Show when=move || presence.is_present.get()>
                <Primitive element=html::li as_child=as_child node_ref=node_ref
                    attr:role="status"
                    attr:aria-live="off"
                    attr:aria-atomic="true"
                    attr:data-state=move || if open_state.get() { "open" } else { "closed" }
                    attr:tabindex="0"
                    on:pointerenter=move |_: leptos::ev::PointerEvent| {
                        // Pause: clear auto-close timer
                        if let Some(id) = timer_id.get_untracked() {
                            clear_timeout(id);
                            timer_id.set(None);
                        }
                    }
                    on:pointerleave=move |_: leptos::ev::PointerEvent| {
                        // Resume: restart auto-close timer
                        let dur = duration.get_untracked();
                        if dur > 0 && open_state.get_untracked() {
                            close_closure_stored.with_value(|cls| {
                                let id = set_timeout(cls, dur);
                                timer_id.set(Some(id));
                            });
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
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let label = Signal::derive(move || label.get().unwrap_or("Notifications".into()));

    view! {
        <Primitive element=html::ol as_child=as_child node_ref=node_ref
            attr:role="region"
            attr:aria-label=move || label.get()
            attr:tabindex="-1"
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}
