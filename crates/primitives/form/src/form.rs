use leptix_core::id::use_id;
use leptix_core::primitive::{Primitive, VoidPrimitive};
use leptos::{context::Provider, html, prelude::*, web_sys};
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn Form(
    #[prop(into, optional)] on_submit: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_clear_server_errors: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let _on_clear = on_clear_server_errors;

    view! {
        <Primitive element=html::form as_child=as_child node_ref=node_ref
            on:submit=move |event: web_sys::SubmitEvent| {
                let event: web_sys::Event = event.into();
                if let Some(cb) = on_submit { cb.run(event); }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[derive(Clone, Debug)]
struct FormFieldContextValue {
    id: String,
    name: String,
}

#[component]
pub fn FormField(
    #[prop(into)] name: String,
    #[prop(into, optional)] server_invalid: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let _server_invalid = server_invalid;
    let id = use_id(None).get();
    let ctx = FormFieldContextValue { id, name };

    view! {
        <Provider value=ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref>
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn FormLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<FormFieldContextValue>();

    view! {
        <Primitive element=html::label as_child=as_child node_ref=node_ref
            attr:r#for=format!("{}-control", ctx.id)
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn FormControl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<FormFieldContextValue>();

    view! {
        <VoidPrimitive element=html::input as_child=as_child node_ref=node_ref
            attr:id=format!("{}-control", ctx.id)
            attr:name=ctx.name.clone()
            attr:aria-describedby=format!("{}-message", ctx.id)
        >
            {children.with_value(|c| c())}
        </VoidPrimitive>
    }
}

#[component]
pub fn FormMessage(
    #[prop(into, optional)] r#match: MaybeProp<String>,
    #[prop(into, optional)] force_match: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<FormFieldContextValue>();
    let _match_type = r#match;
    let _force = force_match;

    view! {
        <Primitive element=html::span as_child=as_child node_ref=node_ref
            attr:id=format!("{}-message", ctx.id)
            attr:role="alert"
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn FormSubmit(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="submit"
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[derive(Clone, Debug)]
pub struct FormValidityStateContext {
    pub valid: Signal<bool>,
    pub field_name: String,
}

#[component]
pub fn FormValidityState(children: TypedChildrenFn<impl IntoView + 'static>) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<FormFieldContextValue>();
    let control_id = format!("{}-control", ctx.id);

    let valid = RwSignal::new(true);

    Effect::new(move |_| {
        if let Some(document) = web_sys::window().and_then(|w| w.document())
            && let Some(el) = document.get_element_by_id(&control_id)
        {
            use web_sys::wasm_bindgen::JsCast;
            if let Ok(input) = el.dyn_into::<web_sys::HtmlInputElement>() {
                valid.set(input.check_validity());
            }
        }
    });

    let validity_ctx = FormValidityStateContext {
        valid: valid.into(),
        field_name: ctx.name.clone(),
    };

    view! {
        <Provider value=validity_ctx>
            {children.with_value(|c| c())}
        </Provider>
    }
}
