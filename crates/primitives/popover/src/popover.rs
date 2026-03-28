use leptix_core::compose_refs::use_composed_refs;
use leptix_core::dismissable_layer::use_dismissable_layer;
use leptix_core::focus_scope::use_focus_scope;
use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

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

    view! {
        <Provider value=context_value>
            {children.with_value(|children| children())}
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

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=composed_refs
            attr:r#type="button"
            attr:aria-haspopup="dialog"
            attr:aria-expanded=move || context.open.get().to_string()
            attr:aria-controls=move || context.open.get().then(|| context.content_id.clone())
            attr:data-state=move || if context.open.get() { "open" } else { "closed" }
            on:click=move |_| context.on_open_change.run(!context.open.get())
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn PopoverPortal(children: TypedChildrenFn<impl IntoView + 'static>) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<PopoverContextValue>();

    view! {
        <Show when=move || context.open.get()>
            {children.with_value(|children| children())}
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
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<PopoverContextValue>();

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
        <Show when=move || presence.is_present.get()>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_refs
                attr:id=context.content_id.clone()
                attr:role="dialog"
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
            >
                {children.with_value(|children| children())}
            </Primitive>
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
