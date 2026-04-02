use std::sync::Arc;

use floating_ui_leptos::Padding;
use leptix_core::popper::{
    Popper, PopperAnchor, PopperArrow, PopperContent, parse_align, parse_side,
};
use leptix_core::portal::Portal;
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
struct TooltipContextValue {
    open: Signal<bool>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    trigger_ref: AnyNodeRef,
    content_id: String,
    delay_duration: Signal<i32>,
    open_timer_id: RwSignal<Option<i32>>,
    was_instant_open: RwSignal<bool>,
}

impl TooltipContextValue {
    #[allow(dead_code)]
    fn clear_open_timer(&self) {
        if let Some(id) = self.open_timer_id.get_untracked() {
            clear_timeout(id);
            self.open_timer_id.set(None);
        }
    }
}

#[derive(Clone, Debug)]
struct TooltipProviderContextValue {
    delay_duration: Signal<i32>,
    skip_delay_duration: Signal<i32>,
    /// Timestamp (ms) of the last tooltip close, used to skip delays for rapid hover.
    last_close_time: RwSignal<f64>,
}

/// Global tooltip provider — manages delay behavior.
#[component]
pub fn TooltipProvider(
    #[prop(into, optional, default = 700.into())] delay_duration: MaybeProp<i32>,
    #[prop(into, optional, default = 300.into())] skip_delay_duration: MaybeProp<i32>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let provider_ctx = TooltipProviderContextValue {
        delay_duration: Signal::derive(move || delay_duration.get().unwrap_or(700)),
        skip_delay_duration: Signal::derive(move || skip_delay_duration.get().unwrap_or(300)),
        last_close_time: RwSignal::new(0.0),
    };

    view! {
        <Provider value=provider_ctx>
            {children.with_value(|children| children())}
        </Provider>
    }
}

#[component]
pub fn Tooltip(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] delay_duration: MaybeProp<i32>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let provider_ctx = use_context::<TooltipProviderContextValue>();
    let provider_for_base_delay = provider_ctx.clone();
    let delay_duration = Signal::derive(move || {
        delay_duration.get().unwrap_or_else(|| {
            provider_for_base_delay
                .as_ref()
                .map(|ctx| ctx.delay_duration.get())
                .unwrap_or(700)
        })
    });

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

    // Compute effective delay: skip if within provider's skip_delay_duration of last close
    let provider_for_delay = provider_ctx.clone();
    let effective_delay = Signal::derive(move || {
        if let Some(ref pctx) = provider_for_delay {
            let skip_dur = pctx.skip_delay_duration.get() as f64;
            let last_close = pctx.last_close_time.get();
            let now = window().performance().map(|p| p.now()).unwrap_or(0.0);
            if last_close > 0.0 && (now - last_close) < skip_dur {
                return 0;
            }
        }
        delay_duration.get()
    });

    let provider_for_close = provider_ctx.clone();
    let on_close_cb = Callback::new(move |()| {
        set_open.run(Some(false));
        if let Some(ref pctx) = provider_for_close {
            let now = window().performance().map(|p| p.now()).unwrap_or(0.0);
            pctx.last_close_time.set(now);
        }
    });

    let was_instant_open = RwSignal::new(false);

    let context_value = TooltipContextValue {
        open,
        on_open: Callback::new(move |()| {
            was_instant_open.set(effective_delay.get_untracked() == 0);
            set_open.run(Some(true));
        }),
        on_close: on_close_cb,
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{}-tooltip", base_id),
        delay_duration: effective_delay,
        open_timer_id,
        was_instant_open,
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

    let context_value = StoredValue::new(context_value);

    view! {
        <Popper>
            <Provider value=context_value.get_value()>
                {children.with_value(|children| children())}
            </Provider>
        </Popper>
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

    let ctx_open = context.open;
    let ctx_on_open = context.on_open;
    let ctx_on_close = context.on_close;
    let ctx_delay = context.delay_duration;
    let ctx_timer = context.open_timer_id;
    let content_id = StoredValue::new(context.content_id.clone());
    let open_closure = StoredValue::new(open_closure);

    view! {
        <PopperAnchor>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_refs
                attr:aria-describedby=move || ctx_open.get().then(|| content_id.get_value())
                attr:data-state=move || if ctx_open.get() {
                    if context.was_instant_open.get() { "instant-open" } else { "delayed-open" }
                } else { "closed" }
                on:pointerenter=move |_: leptos::ev::PointerEvent| {
                    if let Some(id) = ctx_timer.try_get_untracked().flatten() {
                        clear_timeout(id);
                        ctx_timer.set(None);
                    }
                    let delay = ctx_delay.get_untracked();
                    if delay <= 0 {
                        ctx_on_open.run(());
                    } else {
                        let oc = open_closure.get_value();
                        let id = set_timeout(&oc, delay);
                        ctx_timer.set(Some(id));
                    }
                }
                on:pointerleave=move |_: leptos::ev::PointerEvent| {
                    if let Some(id) = ctx_timer.try_get_untracked().flatten() {
                        clear_timeout(id);
                        ctx_timer.set(None);
                    }
                    ctx_on_close.run(());
                }
                on:focus=move |_| ctx_on_open.run(())
                on:blur=move |_| {
                    if let Some(id) = ctx_timer.try_get_untracked().flatten() {
                        clear_timeout(id);
                        ctx_timer.set(None);
                    }
                    ctx_on_close.run(());
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </PopperAnchor>
    }
}

#[component]
pub fn TooltipPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TooltipContextValue>();

    view! {
        <Show when=move || context.open.get()>
            <Portal container=container>
                {children.with_value(|children| children())}
            </Portal>
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
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TooltipContextValue>();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    let popper_side = Signal::derive(move || parse_side(&side.get().unwrap_or("top".into())));
    let popper_align = Signal::derive(move || parse_align(&align.get().unwrap_or("center".into())));
    let popper_side_offset = Signal::derive(move || side_offset.get().unwrap_or(0.0));
    let popper_align_offset = Signal::derive(move || align_offset.get().unwrap_or(0.0));
    let popper_avoid_collisions = Signal::derive(move || avoid_collisions.get().unwrap_or(true));
    let popper_collision_padding =
        Signal::derive(move || Padding::All(collision_padding.get().unwrap_or(0.0)));

    let present = Signal::derive(move || context.open.get());
    let presence = use_presence(present);

    let composed_refs =
        leptix_core::compose_refs::use_composed_refs(vec![node_ref, presence.node_ref]);

    let content_id = StoredValue::new(context.content_id.clone());
    let ctx_open = context.open;
    let ctx_on_open = context.on_open;
    let ctx_on_close = context.on_close;
    let ctx_open_timer_id = context.open_timer_id;

    view! {
        <Show when=move || force_mount.get() || presence.is_present.get()>
            <PopperContent
                side=popper_side
                side_offset=popper_side_offset
                align=popper_align
                align_offset=popper_align_offset
                avoid_collisions=popper_avoid_collisions
                collision_padding=popper_collision_padding
                as_child=as_child
                node_ref=composed_refs
                attr:id=content_id.get_value()
                attr:role="tooltip"
                attr:data-state=move || if ctx_open.get() {
                    if context.was_instant_open.get() { "instant-open" } else { "delayed-open" }
                } else { "closed" }
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
            </PopperContent>
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
        <PopperArrow width=width height=height as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </PopperArrow>
    }
}
