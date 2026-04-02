use std::sync::Arc;

use floating_ui_leptos::Padding;
use leptix_core::popper::{
    Popper, PopperAnchor, PopperArrow, PopperContent, Sticky, UpdatePositionStrategy, parse_align,
    parse_side,
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

    let ctx = StoredValue::new(context_value.clone());

    view! {
        <Provider value=context_value>
            <Popper>
                {move || ctx.with_value(|_| children.with_value(|children| children()))}
            </Popper>
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

    let on_pointer_enter = StoredValue::new(on_pointer_enter);
    let on_pointer_leave = StoredValue::new(on_pointer_leave);

    view! {
        <PopperAnchor>
            {move || view! {
                <Primitive
                    element=html::a
                    as_child=as_child
                    node_ref=node_ref
                    attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                    on:pointerenter=move |e: leptos::ev::PointerEvent| on_pointer_enter.with_value(|f| f(e))
                    on:pointerleave=move |e: leptos::ev::PointerEvent| on_pointer_leave.with_value(|f| f(e))
                    on:focus=move |_| context.on_open.run(())
                    on:blur=move |_| context.on_close.run(())
                >
                    {children.with_value(|children| children())}
                </Primitive>
            }}
        </PopperAnchor>
    }
}

#[component]
pub fn HoverCardPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<HoverCardContextValue>();

    let context_for_portal = StoredValue::new(context.clone());
    view! {
        <Show when=move || context.open.get()>
            <Portal container=container>
                <Provider value=context_for_portal.get_value()>
                    {children.with_value(|children| children())}
                </Provider>
            </Portal>
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
    #[prop(into, optional)] collision_boundary: MaybeProp<Vec<web_sys::Element>>,
    #[prop(into, optional)] sticky: MaybeProp<String>,
    #[prop(into, optional)] hide_when_detached: MaybeProp<bool>,
    #[prop(into, optional)] update_position_strategy: MaybeProp<String>,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_open_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    let popper_side = Signal::derive(move || parse_side(&side.get().unwrap_or("bottom".into())));
    let popper_align = Signal::derive(move || parse_align(&align.get().unwrap_or("center".into())));
    let popper_side_offset = Signal::derive(move || side_offset.get().unwrap_or(0.0));
    let popper_align_offset = Signal::derive(move || align_offset.get().unwrap_or(0.0));
    let popper_avoid_collisions = Signal::derive(move || avoid_collisions.get().unwrap_or(true));
    let popper_collision_padding =
        Signal::derive(move || Padding::All(collision_padding.get().unwrap_or(0.0)));
    let popper_collision_boundary =
        Signal::derive(move || SendWrapper::new(collision_boundary.get().unwrap_or_default()));
    let popper_sticky = Signal::derive(move || match sticky.get().as_deref() {
        Some("always") => Sticky::Always,
        _ => Sticky::Partial,
    });
    let popper_hide_when_detached =
        Signal::derive(move || hide_when_detached.get().unwrap_or(false));
    let popper_update_position_strategy =
        Signal::derive(move || match update_position_strategy.get().as_deref() {
            Some("always") => UpdatePositionStrategy::Always,
            _ => UpdatePositionStrategy::Optimized,
        });

    let context = expect_context::<HoverCardContextValue>();

    let present = Signal::derive(move || context.open.get());
    let presence = use_presence(present);

    // Fire auto-focus callbacks when content opens/closes.
    let prev_open = RwSignal::new(false);
    Effect::new(move |_| {
        let is_open = context.open.get();
        if is_open && !prev_open.get_untracked() {
            if let Some(cb) = on_open_auto_focus {
                cb.run(web_sys::Event::new("focusin").unwrap());
            }
        } else if !is_open && prev_open.get_untracked() {
            if let Some(cb) = on_close_auto_focus {
                cb.run(web_sys::Event::new("focusout").unwrap());
            }
        }
        prev_open.set(is_open);
    });

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
    let content_id = StoredValue::new(context.content_id.clone());
    let open_delay = context.open_delay;
    let close_delay = context.close_delay;
    let open_timer_id = context.open_timer_id;
    let close_timer_id = context.close_timer_id;
    let ctx_on_open = context.on_open;
    let ctx_on_close = context.on_close;

    view! {
        <Show when=move || force_mount.get() || presence.is_present.get()>
            <PopperContent
                side=popper_side
                side_offset=popper_side_offset
                align=popper_align
                align_offset=popper_align_offset
                avoid_collisions=popper_avoid_collisions
                collision_padding=popper_collision_padding
                collision_boundary=popper_collision_boundary
                sticky=popper_sticky
                hide_when_detached=popper_hide_when_detached
                update_position_strategy=popper_update_position_strategy
                node_ref=composed_refs
                as_child=as_child
                attr:id=content_id.with_value(|id| id.clone())
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
            </PopperContent>
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
        <PopperArrow width=width height=height as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </PopperArrow>
    }
}
