use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptix_core::visually_hidden::VisuallyHidden;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
struct TooltipContextValue {
    open: Signal<bool>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    trigger_ref: AnyNodeRef,
    content_id: String,
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
    let _delay_duration = Signal::derive(move || delay_duration.get().unwrap_or(700));

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

    let context_value = TooltipContextValue {
        open,
        on_open: Callback::new(move |()| set_open.run(Some(true))),
        on_close: Callback::new(move |()| set_open.run(Some(false))),
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{}-tooltip", base_id),
    };

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

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=composed_refs
            attr:aria-describedby=move || context.open.get().then(|| context.content_id.clone())
            attr:data-state=move || if context.open.get() { "delayed-open" } else { "closed" }
            on:pointerenter=move |_| context.on_open.run(())
            on:pointerleave=move |_| context.on_close.run(())
            on:focus=move |_| context.on_open.run(())
            on:blur=move |_| context.on_close.run(())
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
    /// Which side to position the tooltip on.
    #[prop(into, optional)]
    side: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TooltipContextValue>();
    let _side = Signal::derive(move || side.get().unwrap_or("top".into()));

    let present = Signal::derive(move || context.open.get());
    let presence = use_presence(present);

    let composed_refs =
        leptix_core::compose_refs::use_composed_refs(vec![node_ref, presence.node_ref]);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_refs
                attr:id=context.content_id.clone()
                attr:role="tooltip"
                attr:data-state=move || if context.open.get() { "delayed-open" } else { "closed" }
                attr:style="pointer-events:auto"
                on:pointerenter=move |_| context.on_open.run(())
                on:pointerleave=move |_| context.on_close.run(())
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
