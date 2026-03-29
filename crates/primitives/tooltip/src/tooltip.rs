use std::sync::Arc;

use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptix_core::visually_hidden::VisuallyHidden;
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
struct TooltipContextValue {
    open: Signal<bool>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    trigger_ref: AnyNodeRef,
    content_id: String,
    delay_duration: Signal<i32>,
    open_timer_id: RwSignal<Option<i32>>,
}

impl TooltipContextValue {
    /// Clear any pending open timer.
    fn clear_open_timer(&self) {
        if let Some(id) = self.open_timer_id.get_untracked() {
            clear_timeout(id);
            self.open_timer_id.set(None);
        }
    }
}

/// Global tooltip provider — manages delay behavior.
#[component]
pub fn TooltipProvider(
    #[prop(into, optional, default = 700.into())] delay_duration: MaybeProp<i32>,
    #[prop(into, optional, default = 300.into())] skip_delay_duration: MaybeProp<i32>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let _delay = Signal::derive(move || delay_duration.get().unwrap_or(700));
    let _skip_delay = Signal::derive(move || skip_delay_duration.get().unwrap_or(300));

    // In a full implementation, the provider would manage shared timer state.
    // For now, each tooltip manages its own delay.
    children.with_value(|children| children())
}

#[component]
pub fn Tooltip(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional, default = 700.into())] delay_duration: MaybeProp<i32>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let delay_duration = Signal::derive(move || delay_duration.get().unwrap_or(700));

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

    let context_value = TooltipContextValue {
        open,
        on_open: Callback::new(move |()| set_open.run(Some(true))),
        on_close: Callback::new(move |()| set_open.run(Some(false))),
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{}-tooltip", base_id),
        delay_duration,
        open_timer_id,
    };

    // Clean up timer on unmount
    on_cleanup({
        let timer_id = context_value.open_timer_id;
        move || {
            if let Some(id) = timer_id.try_get_untracked().flatten() {
                clear_timeout(id);
            }
        }
    });

    view! {
        <Provider value=context_value>
            {children.with_value(|children| children())}
        </Provider>
    }
}

#[component]
pub fn TooltipTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TooltipContextValue>();
    let composed_refs =
        leptix_core::compose_refs::use_composed_refs(vec![node_ref, context.trigger_ref]);

    // Build the delayed-open closure once, wrapped for Send+Sync
    let on_open = context.on_open;
    let open_closure: Arc<SendWrapper<Closure<dyn Fn()>>> =
        Arc::new(SendWrapper::new(Closure::new(move || {
            on_open.run(());
        })));

    let open_closure_for_enter = open_closure.clone();
    let ctx_for_enter = context.clone();
    let on_pointer_enter = move |_: leptos::ev::PointerEvent| {
        ctx_for_enter.clear_open_timer();
        let delay = ctx_for_enter.delay_duration.get_untracked();
        if delay <= 0 {
            ctx_for_enter.on_open.run(());
        } else {
            let id = set_timeout(&open_closure_for_enter, delay);
            ctx_for_enter.open_timer_id.set(Some(id));
        }
    };

    let ctx_for_leave = context.clone();
    let on_pointer_leave = move |_: leptos::ev::PointerEvent| {
        ctx_for_leave.clear_open_timer();
        ctx_for_leave.on_close.run(());
    };

    let content_id = context.content_id.clone();

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=composed_refs
            attr:aria-describedby=move || context.open.get().then(|| content_id.clone())
            attr:data-state=move || if context.open.get() { "delayed-open" } else { "closed" }
            on:pointerenter=on_pointer_enter
            on:pointerleave=on_pointer_leave
            on:focus=move |_| context.on_open.run(())
            on:blur=move |_| {
                context.clear_open_timer();
                context.on_close.run(());
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn TooltipPortal(children: TypedChildrenFn<impl IntoView + 'static>) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TooltipContextValue>();

    view! {
        <Show when=move || context.open.get()>
            {children.with_value(|children| children())}
        </Show>
    }
}

#[component]
pub fn TooltipContent(
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
    let context = expect_context::<TooltipContextValue>();

    // Reserved for future floating-ui integration
    let _side = Signal::derive(move || side.get().unwrap_or("top".into()));
    let _side_offset = side_offset;
    let _align = align;
    let _align_offset = align_offset;
    let _avoid_collisions = avoid_collisions;
    let _collision_padding = collision_padding;

    let present = Signal::derive(move || context.open.get());
    let presence = use_presence(present);

    let composed_refs =
        leptix_core::compose_refs::use_composed_refs(vec![node_ref, presence.node_ref]);

    let content_id = context.content_id.clone();
    let ctx_open = context.open;
    let ctx_on_open = context.on_open;
    let ctx_on_close = context.on_close;
    let ctx_open_timer_id = context.open_timer_id;

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_refs
                attr:id=content_id.clone()
                attr:role="tooltip"
                attr:data-state=move || if ctx_open.get() { "delayed-open" } else { "closed" }
                attr:style="pointer-events:auto"
                on:pointerenter=move |_| ctx_on_open.run(())
                on:pointerleave=move |_| {
                    if let Some(id) = ctx_open_timer_id.get_untracked() {
                        clear_timeout(id);
                        ctx_open_timer_id.set(None);
                    }
                    ctx_on_close.run(());
                }
            >
                {children.with_value(|children| children())}
                // Screen reader announcement
                <VisuallyHidden>
                    {children.with_value(|children| children())}
                </VisuallyHidden>
            </Primitive>
        </Show>
    }
}

#[component]
pub fn TooltipArrow(
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
