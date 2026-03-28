use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
struct HoverCardContextValue {
    open: Signal<bool>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    content_id: String,
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
    let _open_delay = Signal::derive(move || open_delay.get().unwrap_or(700));
    let _close_delay = Signal::derive(move || close_delay.get().unwrap_or(300));

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

    let context_value = HoverCardContextValue {
        open,
        on_open: Callback::new(move |()| set_open.run(Some(true))),
        on_close: Callback::new(move |()| set_open.run(Some(false))),
        content_id: format!("{}-hovercard", base_id),
    };

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

    view! {
        <Primitive
            element=html::a
            as_child=as_child
            node_ref=node_ref
            attr:data-state=move || if context.open.get() { "open" } else { "closed" }
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
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<HoverCardContextValue>();

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
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                on:pointerenter=move |_| context.on_open.run(())
                on:pointerleave=move |_| context.on_close.run(())
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Show>
    }
}
