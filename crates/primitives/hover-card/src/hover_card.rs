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

#[derive(Clone, Debug)]
struct HoverCardContextValue {
    open: Signal<bool>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    content_id: String,
    open_delay: Signal<i32>,
    close_delay: Signal<i32>,
    open_timer_id: RwSignal<Option<i32>>,
    close_timer_id: RwSignal<Option<i32>>,
}

impl HoverCardContextValue {
    fn clear_open_timer(&self) {
        if let Some(id) = self.open_timer_id.get_untracked() {
            clear_timeout(id);
            self.open_timer_id.set(None);
        }
    }

    fn clear_close_timer(&self) {
        if let Some(id) = self.close_timer_id.get_untracked() {
            clear_timeout(id);
            self.close_timer_id.set(None);
        }
    }
}

#[component]
pub fn HoverCard(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional, default = 700.into())] open_delay: MaybeProp<i32>,
    #[prop(into, optional, default = 300.into())] close_delay: MaybeProp<i32>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let open_delay = Signal::derive(move || open_delay.get().unwrap_or(700));
    let close_delay = Signal::derive(move || close_delay.get().unwrap_or(300));

    let (open_state, set_open) = leptix_core::use_controllable_state::use_controllable_state(
        leptix_core::use_controllable_state::UseControllableStateParams {
            prop: open,
            on_change: on_open_change.map(|cb| {
                Callback::new(move |value: Option<bool>| {
                    if let Some(value) = value {
                        cb.run(value);
                    }
                })
            }),
            default_prop: default_open,
        },
    );
    let open = Signal::derive(move || open_state.get().unwrap_or(false));
    let base_id = leptix_core::id::use_id(None).get();
    let open_timer_id: RwSignal<Option<i32>> = RwSignal::new(None);
    let close_timer_id: RwSignal<Option<i32>> = RwSignal::new(None);

    let context_value = HoverCardContextValue {
        open,
        on_open: Callback::new(move |()| set_open.run(Some(true))),
        on_close: Callback::new(move |()| set_open.run(Some(false))),
        content_id: format!("{}-hovercard", base_id),
        open_delay,
        close_delay,
        open_timer_id,
        close_timer_id,
    };

    // Clean up timers on unmount
    on_cleanup(move || {
        if let Some(id) = open_timer_id.try_get_untracked().flatten() {
            clear_timeout(id);
        }
        if let Some(id) = close_timer_id.try_get_untracked().flatten() {
            clear_timeout(id);
        }
    });

    view! {
        <Provider value=context_value>
            {children.with_value(|children| children())}
        </Provider>
    }
}

#[component]
pub fn HoverCardTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<HoverCardContextValue>();

    // Build delayed-open closure, wrapped for Send+Sync
    let on_open = context.on_open;
    let open_closure: Arc<SendWrapper<Closure<dyn Fn()>>> =
        Arc::new(SendWrapper::new(Closure::new(move || {
            on_open.run(());
        })));

    // Build delayed-close closure, wrapped for Send+Sync
    let on_close = context.on_close;
    let close_closure: Arc<SendWrapper<Closure<dyn Fn()>>> =
        Arc::new(SendWrapper::new(Closure::new(move || {
            on_close.run(());
        })));

    let open_closure_for_enter = open_closure.clone();
    let ctx_for_enter = context.clone();
    let on_pointer_enter = move |_: leptos::ev::PointerEvent| {
        ctx_for_enter.clear_close_timer();
        ctx_for_enter.clear_open_timer();
        let delay = ctx_for_enter.open_delay.get_untracked();
        if delay <= 0 {
            ctx_for_enter.on_open.run(());
        } else {
            let id = set_timeout(&open_closure_for_enter, delay);
            ctx_for_enter.open_timer_id.set(Some(id));
        }
    };

    let close_closure_for_leave = close_closure.clone();
    let ctx_for_leave = context.clone();
    let on_pointer_leave = move |_: leptos::ev::PointerEvent| {
        ctx_for_leave.clear_open_timer();
        ctx_for_leave.clear_close_timer();
        let delay = ctx_for_leave.close_delay.get_untracked();
        if delay <= 0 {
            ctx_for_leave.on_close.run(());
        } else {
            let id = set_timeout(&close_closure_for_leave, delay);
            ctx_for_leave.close_timer_id.set(Some(id));
        }
    };

    view! {
        <Primitive
            element=html::a
            as_child=as_child
            node_ref=node_ref
            attr:data-state=move || if context.open.get() { "open" } else { "closed" }
            on:pointerenter=on_pointer_enter
            on:pointerleave=on_pointer_leave
            on:focus=move |_| context.on_open.run(())
            on:blur=move |_| context.on_close.run(())
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn HoverCardPortal(children: TypedChildrenFn<impl IntoView + 'static>) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<HoverCardContextValue>();

    view! {
        <Show when=move || context.open.get()>
            {children.with_value(|children| children())}
        </Show>
    }
}

#[component]
pub fn HoverCardContent(
    /// Which side to position on: "top" | "right" | "bottom" | "left"
    #[prop(into, optional)]
    side: MaybeProp<String>,
    /// Offset from the trigger (pixels).
    #[prop(into, optional)]
    side_offset: MaybeProp<f64>,
    /// Alignment along the side: "start" | "center" | "end"
    #[prop(into, optional)]
    align: MaybeProp<String>,
    /// Offset along the alignment axis (pixels).
    #[prop(into, optional)]
    align_offset: MaybeProp<f64>,
    /// Whether to flip/shift to avoid viewport collisions.
    #[prop(into, optional)]
    avoid_collisions: MaybeProp<bool>,
    /// Padding from viewport edge when avoiding collisions (pixels).
    #[prop(into, optional)]
    collision_padding: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    // Reserved for future floating-ui integration
    let _side = side;
    let _side_offset = side_offset;
    let _align = align;
    let _align_offset = align_offset;
    let _avoid_collisions = avoid_collisions;
    let _collision_padding = collision_padding;
    let context = expect_context::<HoverCardContextValue>();

    let present = Signal::derive(move || context.open.get());
    let presence = use_presence(present);

    let composed_refs =
        leptix_core::compose_refs::use_composed_refs(vec![node_ref, presence.node_ref]);

    // Build delayed-open/close closures for content hover, wrapped for Send+Sync
    let on_open = context.on_open;
    let open_closure: Arc<SendWrapper<Closure<dyn Fn()>>> =
        Arc::new(SendWrapper::new(Closure::new(move || {
            on_open.run(());
        })));

    let on_close = context.on_close;
    let close_closure: Arc<SendWrapper<Closure<dyn Fn()>>> =
        Arc::new(SendWrapper::new(Closure::new(move || {
            on_close.run(());
        })));

    // Store closures so they can be accessed from Fn closures inside <Show>
    let open_closure_stored = StoredValue::new(open_closure);
    let close_closure_stored = StoredValue::new(close_closure);
    let content_id = context.content_id.clone();
    let open_delay = context.open_delay;
    let close_delay = context.close_delay;
    let open_timer_id = context.open_timer_id;
    let close_timer_id = context.close_timer_id;
    let ctx_on_open = context.on_open;
    let ctx_on_close = context.on_close;

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_refs
                attr:id=content_id.clone()
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                on:pointerenter=move |_: leptos::ev::PointerEvent| {
                    // Clear close timer, start open timer
                    if let Some(id) = close_timer_id.get_untracked() {
                        clear_timeout(id);
                        close_timer_id.set(None);
                    }
                    if let Some(id) = open_timer_id.get_untracked() {
                        clear_timeout(id);
                        open_timer_id.set(None);
                    }
                    let delay = open_delay.get_untracked();
                    if delay <= 0 {
                        ctx_on_open.run(());
                    } else {
                        open_closure_stored.with_value(|cls| {
                            let id = set_timeout(cls, delay);
                            open_timer_id.set(Some(id));
                        });
                    }
                }
                on:pointerleave=move |_: leptos::ev::PointerEvent| {
                    // Clear open timer, start close timer
                    if let Some(id) = open_timer_id.get_untracked() {
                        clear_timeout(id);
                        open_timer_id.set(None);
                    }
                    if let Some(id) = close_timer_id.get_untracked() {
                        clear_timeout(id);
                        close_timer_id.set(None);
                    }
                    let delay = close_delay.get_untracked();
                    if delay <= 0 {
                        ctx_on_close.run(());
                    } else {
                        close_closure_stored.with_value(|cls| {
                            let id = set_timeout(cls, delay);
                            close_timer_id.set(Some(id));
                        });
                    }
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Show>
    }
}

#[component]
pub fn HoverCardArrow(
    #[prop(into, optional, default=10.0.into())] width: MaybeProp<f64>,
    #[prop(into, optional, default=5.0.into())] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <leptix_core::arrow::Arrow width=width height=height as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </leptix_core::arrow::Arrow>
    }
}
