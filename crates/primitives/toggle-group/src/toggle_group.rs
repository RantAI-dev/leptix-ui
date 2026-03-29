use leptix_core::direction::{Direction, use_direction};
use leptix_core::primitive::Primitive;
use leptix_core::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
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
    orientation: Signal<String>,
}

#[component]
pub fn ToggleGroup(
    #[prop(into)] r#type: ToggleGroupType,
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let orientation = Signal::derive(move || orientation.get().unwrap_or("horizontal".into()));
    let direction = use_direction(dir);
    let do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));

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
        orientation,
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
                attr:data-orientation=move || orientation.get()
                attr:data-disabled=move || disabled.get().then_some("")
                attr:dir=move || direction.get().to_string()
                on:keydown=move |event: KeyboardEvent| {
                    let is_vertical = orientation.get() != "horizontal";
                    let is_horizontal = orientation.get() != "vertical";
                    let is_rtl = direction.get() == Direction::Rtl;

                    let next = match event.key().as_str() {
                        "ArrowUp" if is_vertical => Some(false),
                        "ArrowDown" if is_vertical => Some(true),
                        "ArrowLeft" if is_horizontal => Some(is_rtl),
                        "ArrowRight" if is_horizontal => Some(!is_rtl),
                        "Home" => {
                            event.prevent_default();
                            roving_focus_items(&event, true, true);
                            None
                        }
                        "End" => {
                            event.prevent_default();
                            roving_focus_items(&event, false, true);
                            None
                        }
                        _ => None,
                    };

                    if let Some(forward) = next {
                        event.prevent_default();
                        roving_focus_items(&event, forward, do_loop.get());
                    }
                }
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
            attr:data-orientation=move || context.orientation.get()
            attr:data-disabled=move || disabled.get().then_some("")
            attr:disabled=move || disabled.get().then_some("")
            attr:tabindex=move || {
                if is_pressed.get() { "0" } else { "-1" }
            }
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

/// Helper: move focus between sibling toggle-group items using DOM queries.
fn roving_focus_items(event: &KeyboardEvent, forward: bool, do_loop: bool) {
    let target = event.current_target();
    let Some(group) = target.and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    let Ok(items) = group.query_selector_all("button:not([disabled])") else {
        return;
    };

    let mut nodes = vec![];
    for i in 0..items.length() {
        if let Some(node) = items.item(i) {
            nodes.push(node);
        }
    }

    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());

    let current_index = active
        .as_ref()
        .and_then(|a| {
            use web_sys::wasm_bindgen::JsCast;
            let a_node: &web_sys::Node = a.unchecked_ref();
            nodes.iter().position(|n| n == a_node)
        })
        .unwrap_or(0);

    let next_index = if forward {
        if current_index + 1 < nodes.len() {
            Some(current_index + 1)
        } else if do_loop {
            Some(0)
        } else {
            None
        }
    } else if current_index > 0 {
        Some(current_index - 1)
    } else if do_loop {
        Some(nodes.len().saturating_sub(1))
    } else {
        None
    };

    if let Some(idx) = next_index
        && let Some(node) = nodes.get(idx)
    {
        use web_sys::wasm_bindgen::JsCast;
        if let Ok(el) = node.clone().dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}
