use leptix_core::compose_refs::use_composed_refs;
use leptix_core::primitive::{Primitive, compose_callbacks};
use leptix_core::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptix_core::use_previous::use_previous;
use leptix_core::use_size::use_size;
use leptos::{context::Provider, ev::MouseEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
struct SwitchContextValue {
    checked: Signal<bool>,
    disabled: Signal<bool>,
}

#[component]
pub fn Switch(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] default_checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let name = Signal::derive(move || name.get());
    let required = Signal::derive(move || required.get().unwrap_or(false));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let value = Signal::derive(move || value.get().unwrap_or("on".into()));

    let button_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, button_ref]);

    let is_form_control = Signal::derive(move || {
        button_ref
            .get()
            .and_then(|button| button.closest("form").ok())
            .flatten()
            .is_some()
    });
    let (checked, set_checked) = use_controllable_state(UseControllableStateParams {
        prop: checked,
        on_change: on_checked_change.map(|on_checked_change| {
            Callback::new(move |value| {
                if let Some(value) = value {
                    on_checked_change.run(value);
                }
            })
        }),
        default_prop: default_checked,
    });
    let checked = Signal::derive(move || checked.get().unwrap_or(false));

    let context_value = SwitchContextValue { checked, disabled };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_refs
                attr:r#type="button"
                attr:role="switch"
                attr:aria-checked=move || checked.get().to_string()
                attr:aria-required=move || required.get().to_string()
                attr:data-state=move || get_state(checked.get())
                attr:data-disabled=move || disabled.get().then_some("")
                attr:disabled=move || disabled.get().then_some("")
                attr:value=move || value.get()
                on:click=compose_callbacks(on_click, Some(Callback::new(move |event: MouseEvent| {
                    set_checked.run(Some(!checked.get()));

                    if is_form_control.get() {
                        event.stop_propagation();
                    }
                })), None)
            >
                {children.with_value(|children| children())}
            </Primitive>
            <Show when=move || is_form_control.get()>
                <BubbleInput
                    control_ref=button_ref
                    bubbles=Signal::derive(|| true)
                    name=name
                    value=value
                    checked=checked
                    required=required
                    disabled=disabled
                />
            </Show>
        </Provider>
    }
}

#[component]
pub fn SwitchThumb(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SwitchContextValue>();

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:data-state=move || get_state(context.checked.get())
            attr:data-disabled=move || context.disabled.get().then_some("")
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

#[component]
fn BubbleInput(
    #[prop(into)] control_ref: AnyNodeRef,
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] bubbles: Signal<bool>,
    #[prop(into)] required: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] name: Signal<Option<String>>,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = NodeRef::new();
    let prev_checked = use_previous(checked);
    let control_size = use_size(control_ref);

    // Bubble checked change to parents
    Effect::new(move |_| {
        if let Some(input) = node_ref.get()
            && prev_checked.get() != checked.get()
        {
            let init = web_sys::EventInit::new();
            init.set_bubbles(bubbles.get());

            let event = web_sys::Event::new_with_event_init_dict("click", &init)
                .expect("Click event should be instantiated.");

            input.set_checked(checked.get());

            input
                .dispatch_event(&event)
                .expect("Click event should be dispatched.");
        }
    });

    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            aria-hidden="true"
            checked=move || checked.get().then_some("")
            required=move || required.get().then_some("")
            disabled=move || disabled.get().then_some("")
            name=move || name.get()
            value=move || value.get()
            tab-index="-1"
            style:transform="translateX(-100%)"
            style:width=move || control_size.get().map(|size| format!("{}px", size.width))
            style:height=move || control_size.get().map(|size| format!("{}px", size.height))
            style:position="absolute"
            style:pointer-events="none"
            style:opacity="0"
            style:margin="0px"
        />
    }
}

fn get_state(checked: bool) -> String {
    (match checked {
        true => "checked",
        false => "unchecked",
    })
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_state_values() {
        assert_eq!(get_state(true), "checked");
        assert_eq!(get_state(false), "unchecked");
    }
}
