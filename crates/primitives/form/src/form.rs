use leptix_core::id::use_id;
use leptix_core::primitive::{Primitive, VoidPrimitive};
use leptos::{context::Provider, html, prelude::*, web_sys};
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn Form(
    #[prop(into, optional)] on_submit: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_clear_server_errors: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <Primitive element=html::form as_child=as_child node_ref=node_ref
            on:submit=move |event: web_sys::SubmitEvent| {
                if let Some(cb) = on_clear_server_errors { cb.run(()); }
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
    server_invalid: Signal<bool>,
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
    let server_invalid = Signal::derive(move || server_invalid.get().unwrap_or(false));
    let id = use_id(None).get();
    let ctx = FormFieldContextValue {
        id,
        name,
        server_invalid,
    };

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
    let match_type = Signal::derive(move || r#match.get());
    let force_match = Signal::derive(move || force_match.get().unwrap_or(false));
    let control_id = format!("{}-control", ctx.id);
    let server_invalid = ctx.server_invalid;

    let should_show = Signal::derive(move || {
        // force_match: always show the message
        if force_match.get() {
            return true;
        }

        // Try to find the associated form control and check validity
        let validity = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&control_id))
            .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
            .map(|input| input.validity());

        if let Some(match_name) = match_type.get() {
            // Only show when the specific ValidityState flag is true
            if let Some(v) = &validity {
                return match match_name.as_str() {
                    "valueMissing" => v.value_missing(),
                    "typeMismatch" => v.type_mismatch(),
                    "patternMismatch" => v.pattern_mismatch(),
                    "tooLong" => v.too_long(),
                    "tooShort" => v.too_short(),
                    "rangeUnderflow" => v.range_underflow(),
                    "rangeOverflow" => v.range_overflow(),
                    "stepMismatch" => v.step_mismatch(),
                    "badInput" => v.bad_input(),
                    "customError" => v.custom_error(),
                    _ => false,
                };
            }
            return false;
        }

        // Default: show when invalid or server_invalid
        let native_invalid = validity.map(|v| !v.valid()).unwrap_or(false);
        native_invalid || server_invalid.get()
    });

    let message_id = format!("{}-message", ctx.id);

    view! {
        <Show when=move || should_show.get()>
            <Primitive element=html::span as_child=as_child node_ref=node_ref
                attr:id=message_id.clone()
                attr:role="alert"
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Show>
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
