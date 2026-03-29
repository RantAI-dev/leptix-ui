use floating_ui_leptos::{Padding, Side};
use leptix_core::compose_refs::use_composed_refs;
use leptix_core::dismissable_layer::use_dismissable_layer;
use leptix_core::focus_scope::use_focus_scope;
use leptix_core::popper::{Align, Popper, PopperAnchor, PopperArrow, PopperContent};
use leptix_core::portal::Portal;
use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

fn parse_side(s: &str) -> Side {
    match s {
        "top" => Side::Top,
        "right" => Side::Right,
        "left" => Side::Left,
        _ => Side::Bottom,
    }
}

fn parse_align(s: &str) -> Align {
    match s {
        "start" => Align::Start,
        "end" => Align::End,
        _ => Align::Center,
    }
}

#[derive(Clone, Debug)]
struct PopoverContextValue {
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    trigger_ref: AnyNodeRef,
    content_id: String,
}

#[component]
pub fn Popover(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

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

    let context_value = PopoverContextValue {
        open,
        on_open_change: Callback::new(move |val: bool| set_open.run(Some(val))),
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{}-content", base_id),
    };

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
pub fn PopoverTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<PopoverContextValue>();
    let composed_refs = use_composed_refs(vec![node_ref, context.trigger_ref]);
    let content_id = StoredValue::new(context.content_id.clone());

    view! {
        <PopperAnchor>
            {move || view! {
                <Primitive
                    element=html::button
                    as_child=as_child
                    node_ref=composed_refs
                    attr:r#type="button"
                    attr:aria-haspopup="dialog"
                    attr:aria-expanded=move || context.open.get().to_string()
                    attr:aria-controls=move || context.open.get().then(|| content_id.with_value(|id| id.clone()))
                    attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                    on:click=move |_| context.on_open_change.run(!context.open.get())
                >
                    {children.with_value(|children| children())}
                </Primitive>
            }}
        </PopperAnchor>
    }
}

#[component]
pub fn PopoverPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<PopoverContextValue>();

    view! {
        <Show when=move || context.open.get()>
            <Portal container=container>
                {children.with_value(|children| children())}
            </Portal>
        </Show>
    }
}

#[component]
pub fn PopoverContent(
    #[prop(into, optional)] on_open_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::Event>>,
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
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    let popper_side = Signal::derive(move || parse_side(&side.get().unwrap_or("bottom".into())));
    let popper_align = Signal::derive(move || parse_align(&align.get().unwrap_or("center".into())));
    let popper_side_offset = Signal::derive(move || side_offset.get().unwrap_or(0.0));
    let popper_align_offset = Signal::derive(move || align_offset.get().unwrap_or(0.0));
    let popper_avoid_collisions = Signal::derive(move || avoid_collisions.get().unwrap_or(true));
    let popper_collision_padding =
        Signal::derive(move || Padding::All(collision_padding.get().unwrap_or(0.0)));

    let context = expect_context::<PopoverContextValue>();
    let content_id = StoredValue::new(context.content_id.clone());

    let present = Signal::derive(move || context.open.get());
    let presence = use_presence(present);

    let focus_scope_ref = use_focus_scope(
        Signal::derive(|| false), // Popover is not modal by default
        Signal::derive(|| true),
        on_open_auto_focus,
        on_close_auto_focus,
    );

    let dismissable_ref = use_dismissable_layer(
        on_escape_key_down,
        on_pointer_down_outside,
        None,
        on_interact_outside,
        Some(Callback::new(move |()| context.on_open_change.run(false))),
        Signal::derive(move || !context.open.get()),
    );

    let composed_refs = use_composed_refs(vec![
        node_ref,
        presence.node_ref,
        focus_scope_ref,
        dismissable_ref,
    ]);

    view! {
        <Show when=move || force_mount.get() || presence.is_present.get()>
            <PopperContent
                side=popper_side
                side_offset=popper_side_offset
                align=popper_align
                align_offset=popper_align_offset
                avoid_collisions=popper_avoid_collisions
                collision_padding=popper_collision_padding
                node_ref=composed_refs
                as_child=as_child
                attr:id=content_id.with_value(|id| id.clone())
                attr:role="dialog"
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
            >
                {children.with_value(|children| children())}
            </PopperContent>
        </Show>
    }
}

#[component]
pub fn PopoverClose(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<PopoverContextValue>();

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
            on:click=move |_| context.on_open_change.run(false)
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn PopoverAnchor(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn PopoverArrow(
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
