use leptix_core::compose_refs::use_composed_refs;
use leptix_core::dismissable_layer::use_dismissable_layer;
use leptix_core::focus_scope::use_focus_scope;
use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
struct DialogContextValue {
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    modal: bool,
    trigger_ref: AnyNodeRef,
    content_id: String,
    title_id: String,
    description_id: String,
}

#[component]
pub fn Dialog(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] modal: Option<bool>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let modal = modal.unwrap_or(true);

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
    let context_value = DialogContextValue {
        open,
        on_open_change: Callback::new(move |val: bool| {
            set_open.run(Some(val));
        }),
        modal,
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{}-content", base_id),
        title_id: format!("{}-title", base_id),
        description_id: format!("{}-description", base_id),
    };

    view! {
        <Provider value=context_value>
            {children.with_value(|children| children())}
        </Provider>
    }
}

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<DialogContextValue>();
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

/// Renders dialog content in a portal when open.
#[component]
pub fn DialogPortal(children: TypedChildrenFn<impl IntoView + 'static>) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<DialogContextValue>();

    view! {
        <Show when=move || context.open.get()>
            {children.with_value(|children| children())}
        </Show>
    }
}

/// Full-screen overlay behind the dialog.
#[component]
pub fn DialogOverlay(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<DialogContextValue>();
    let present = Signal::derive(move || context.open.get());
    let presence = use_presence(present);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:style="position:fixed;inset:0"
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Show>
    }
}

/// The main content of the dialog.
#[component]
pub fn DialogContent(
    /// Event handler called when focus moves into the component after opening.
    #[prop(into, optional)]
    on_open_auto_focus: Option<Callback<web_sys::Event>>,
    /// Event handler called when focus moves to the trigger after closing.
    #[prop(into, optional)]
    on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<DialogContextValue>();

    let present = Signal::derive(move || context.open.get());
    let presence = use_presence(present);

    let focus_scope_ref = use_focus_scope(
        Signal::derive(move || context.modal),
        Signal::derive(|| true),
        on_open_auto_focus,
        on_close_auto_focus,
    );

    let dismissable_ref = use_dismissable_layer(
        on_escape_key_down,
        on_pointer_down_outside,
        None,
        on_interact_outside,
        Some(Callback::new(move |()| {
            context.on_open_change.run(false);
        })),
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
                attr:aria-describedby=context.description_id.clone()
                attr:aria-labelledby=context.title_id.clone()
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Show>
    }
}

#[component]
pub fn DialogClose(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<DialogContextValue>();

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
pub fn DialogTitle(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<DialogContextValue>();

    view! {
        <Primitive
            element=html::h2
            as_child=as_child
            node_ref=node_ref
            attr:id=context.title_id.clone()
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn DialogDescription(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<DialogContextValue>();

    view! {
        <Primitive
            element=html::p
            as_child=as_child
            node_ref=node_ref
            attr:id=context.description_id.clone()
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}
