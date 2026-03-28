use leptix_core::primitive::Primitive;
use leptix_core::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToggleGroupType {
    Single,
    Multiple,
}

#[derive(Clone, Debug)]
struct ToggleGroupContextValue {
    group_type: ToggleGroupType,
    value: Signal<Vec<String>>,
    on_item_activate: Callback<String>,
    disabled: Signal<bool>,
}

#[component]
pub fn ToggleGroup(
    #[prop(into)] r#type: ToggleGroupType,
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let (value, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        on_change: on_value_change.map(|cb| {
            Callback::new(move |value: Option<Vec<String>>| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
        default_prop: default_value,
    });
    let value = Signal::derive(move || value.get().unwrap_or_default());

    let group_type = r#type;
    let context_value = ToggleGroupContextValue {
        group_type,
        value,
        on_item_activate: Callback::new(move |item_value: String| {
            let current = value.get();
            let next = match group_type {
                ToggleGroupType::Single => {
                    if current.contains(&item_value) {
                        vec![]
                    } else {
                        vec![item_value]
                    }
                }
                ToggleGroupType::Multiple => {
                    if current.contains(&item_value) {
                        current.into_iter().filter(|v| *v != item_value).collect()
                    } else {
                        let mut next = current;
                        next.push(item_value);
                        next
                    }
                }
            };
            set_value.run(Some(next));
        }),
        disabled,
    };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:role="group"
                attr:data-disabled=move || disabled.get().then_some("")
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ToggleGroupItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ToggleGroupContextValue>();
    let item_value = value.clone();
    let item_value_click = value.clone();

    let disabled =
        Signal::derive(move || context.disabled.get() || disabled.get().unwrap_or(false));
    let is_pressed = Signal::derive(move || context.value.get().contains(&item_value));

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
            attr:role=match context.group_type {
                ToggleGroupType::Single => "radio",
                ToggleGroupType::Multiple => "button",
            }
            attr:aria-pressed=move || match context.group_type {
                ToggleGroupType::Single => None,
                ToggleGroupType::Multiple => Some(is_pressed.get().to_string()),
            }
            attr:aria-checked=move || match context.group_type {
                ToggleGroupType::Single => Some(is_pressed.get().to_string()),
                ToggleGroupType::Multiple => None,
            }
            attr:data-state=move || if is_pressed.get() { "on" } else { "off" }
            attr:data-disabled=move || disabled.get().then_some("")
            attr:disabled=move || disabled.get().then_some("")
            on:click=move |_| {
                if !disabled.get() {
                    context.on_item_activate.run(item_value_click.clone());
                }
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}
