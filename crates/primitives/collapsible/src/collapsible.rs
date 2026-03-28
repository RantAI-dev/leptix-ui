use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptix_core::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
struct CollapsibleContextValue {
    content_id: String,
    open: Signal<bool>,
    disabled: Signal<bool>,
    on_open_toggle: Callback<()>,
}

#[component]
pub fn Collapsible(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let (open, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        on_change: on_open_change.map(|cb| {
            Callback::new(move |value| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
        default_prop: default_open,
    });
    let open = Signal::derive(move || open.get().unwrap_or(false));

    let context_value = CollapsibleContextValue {
        content_id: format!(
            "leptix-collapsible-content-{}",
            leptix_core::id::use_id(None).get()
        ),
        open,
        disabled,
        on_open_toggle: Callback::new(move |()| {
            set_open.run(Some(!open.get()));
        }),
    };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-state=move || if open.get() { "open" } else { "closed" }
                attr:data-disabled=move || disabled.get().then_some("")
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn CollapsibleTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<CollapsibleContextValue>();

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
            attr:aria-controls=context.content_id.clone()
            attr:aria-expanded=move || context.open.get().to_string()
            attr:data-state=move || if context.open.get() { "open" } else { "closed" }
            attr:data-disabled=move || context.disabled.get().then_some("")
            attr:disabled=move || context.disabled.get().then_some("")
            on:click=move |_| context.on_open_toggle.run(())
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn CollapsibleContent(
    /// Used to force mounting when more control is needed.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<CollapsibleContextValue>();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    let present = Signal::derive(move || force_mount.get() || context.open.get());
    let presence = use_presence(present);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:id=context.content_id.clone()
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:data-disabled=move || context.disabled.get().then_some("")
                attr:role="region"
                attr:hidden=move || (!context.open.get()).then_some("")
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Show>
    }
}
