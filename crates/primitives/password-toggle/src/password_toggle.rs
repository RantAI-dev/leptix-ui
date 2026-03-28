use leptix_core::primitive::{Primitive, VoidPrimitive};
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
struct PasswordToggleContextValue {
    visible: RwSignal<bool>,
}

#[component]
pub fn PasswordToggleField(
    #[prop(into, optional)] default_visible: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let visible = RwSignal::new(default_visible.get().unwrap_or(false));
    let ctx = PasswordToggleContextValue { visible };

    view! {
        <Provider value=ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:data-state=move || if visible.get() { "visible" } else { "hidden" }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn PasswordToggleFieldInput(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let ctx = expect_context::<PasswordToggleContextValue>();

    view! {
        <VoidPrimitive element=html::input as_child=as_child node_ref=node_ref
            attr:r#type=move || if ctx.visible.get() { "text" } else { "password" }
            attr:data-state=move || if ctx.visible.get() { "visible" } else { "hidden" }
        >
            {""}
        </VoidPrimitive>
    }
}

#[component]
pub fn PasswordToggleFieldToggle(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<PasswordToggleContextValue>();

    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            attr:aria-label=move || if ctx.visible.get() { "Hide password" } else { "Show password" }
            attr:aria-pressed=move || ctx.visible.get().to_string()
            attr:data-state=move || if ctx.visible.get() { "visible" } else { "hidden" }
            on:click=move |_| ctx.visible.set(!ctx.visible.get())
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn PasswordToggleFieldIcon(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let ctx = expect_context::<PasswordToggleContextValue>();

    view! {
        <Primitive element=html::span as_child=as_child node_ref=node_ref
            attr:aria-hidden="true"
            attr:data-state=move || if ctx.visible.get() { "visible" } else { "hidden" }
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}

#[component]
pub fn PasswordToggleFieldSlot(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c())}
        </Primitive>
    }
}
