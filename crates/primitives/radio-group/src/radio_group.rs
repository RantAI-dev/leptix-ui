use leptix_core::compose_refs::use_composed_refs;
use leptix_core::direction::{Direction, use_direction};
use leptix_core::primitive::Primitive;
use leptix_core::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptix_core::use_previous::use_previous;
use leptix_core::use_size::use_size;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct RadioGroupContextValue {
    name: Signal<Option<String>>,
    value: Signal<Option<String>>,
    required: Signal<bool>,
    disabled: Signal<bool>,
    on_value_change: Callback<String>,
}

#[component]
pub fn RadioGroup(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let name = Signal::derive(move || name.get());
    let required = Signal::derive(move || required.get().unwrap_or(false));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let orientation = Signal::derive(move || orientation.get());
    let direction = use_direction(dir);
    let do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));

    let (value, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        on_change: on_value_change.map(|cb| {
            Callback::new(move |value: Option<String>| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
        default_prop: default_value,
    });
    let value = Signal::derive(move || value.get());

    let context_value = RadioGroupContextValue {
        name,
        value,
        required,
        disabled,
        on_value_change: Callback::new(move |val: String| {
            set_value.run(Some(val));
        }),
    };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:role="radiogroup"
                attr:aria-required=move || required.get().to_string()
                attr:aria-orientation=move || orientation.get()
                attr:data-disabled=move || disabled.get().then_some("")
                attr:dir=move || direction.get().to_string()
                on:keydown=move |event: KeyboardEvent| {
                    let key = event.key();
                    let is_vertical = orientation.get().as_deref() != Some("horizontal");
                    let is_horizontal = orientation.get().as_deref() != Some("vertical");
                    let is_rtl = direction.get() == Direction::Rtl;

                    let next = match key.as_str() {
                        "ArrowUp" if is_vertical => Some(false),
                        "ArrowDown" if is_vertical => Some(true),
                        "ArrowLeft" if is_horizontal => Some(is_rtl),
                        "ArrowRight" if is_horizontal => Some(!is_rtl),
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
pub fn RadioGroupItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<RadioGroupContextValue>();
    let item_value = value.clone();
    let item_value_click = value.clone();
    let item_value_form = StoredValue::new(value.clone());

    let disabled =
        Signal::derive(move || context.disabled.get() || disabled.get().unwrap_or(false));
    let is_checked =
        Signal::derive(move || context.value.get().as_deref() == Some(item_value.as_str()));

    let button_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, button_ref]);

    let is_form_control = Signal::derive(move || {
        button_ref
            .get()
            .and_then(|button| button.closest("form").ok())
            .flatten()
            .is_some()
    });

    let item_ctx = RadioItemContextValue { is_checked };

    view! {
        <Provider value=item_ctx>
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=composed_refs
            attr:r#type="button"
            attr:role="radio"
            attr:aria-checked=move || is_checked.get().to_string()
            attr:data-state=move || if is_checked.get() { "checked" } else { "unchecked" }
            attr:data-disabled=move || disabled.get().then_some("")
            attr:disabled=move || disabled.get().then_some("")
            attr:tabindex=move || if is_checked.get() { "0" } else { "-1" }
            on:click=move |_| {
                if !disabled.get() {
                    context.on_value_change.run(item_value_click.clone());
                }
            }
            on:focus=move |_| {
                // When an item receives focus (e.g. via arrow key), auto-select it
                if !disabled.get() && !is_checked.get() {
                    context.on_value_change.run(value.clone());
                }
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
        <Show when=move || is_form_control.get()>
            <BubbleRadioInput
                control_ref=button_ref
                name=context.name
                value=Signal::derive(move || item_value_form.get_value())
                checked=is_checked
                required=context.required
                disabled=disabled
            />
        </Show>
        </Provider>
    }
}

#[component]
pub fn RadioGroupIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<RadioGroupContextValue>();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    // RadioGroupIndicator is rendered inside a RadioGroupItem.
    // We need to check the parent item's checked state via a local context.
    let item_context = expect_context::<RadioItemContextValue>();

    view! {
        <Show when=move || force_mount.get() || item_context.is_checked.get()>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attr:data-state=move || if item_context.is_checked.get() { "checked" } else { "unchecked" }
                attr:data-disabled=move || context.disabled.get().then_some("")
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Show>
    }
}

#[derive(Clone, Debug)]
struct RadioItemContextValue {
    is_checked: Signal<bool>,
}

// ---------------------------------------------------------------------------
// BubbleRadioInput — hidden native <input type="radio"> for form submission
// ---------------------------------------------------------------------------

#[component]
fn BubbleRadioInput(
    #[prop(into)] control_ref: AnyNodeRef,
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] required: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] name: Signal<Option<String>>,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = NodeRef::new();
    let prev_checked = use_previous(Signal::derive(move || checked.get()));
    let control_size = use_size(control_ref);

    // Dispatch synthetic click when checked state changes so form libraries are notified.
    Effect::new(move |_| {
        let current = checked.get();
        if let Some(input) = node_ref.get()
            && prev_checked.get() != current
        {
            let init = web_sys::EventInit::new();
            init.set_bubbles(true);

            let event = web_sys::Event::new_with_event_init_dict("click", &init)
                .expect("Click event should be instantiated.");

            input.set_checked(current);

            input
                .dispatch_event(&event)
                .expect("Click event should be dispatched.");
        }
    });

    view! {
        <input
            node_ref=node_ref
            type="radio"
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

/// Helper: move focus between sibling radio items using DOM queries.
fn roving_focus_items(event: &KeyboardEvent, forward: bool, do_loop: bool) {
    let target = event.current_target();
    let Some(group) = target.and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    let Ok(items) = group.query_selector_all("[role='radio']:not([disabled])") else {
        return;
    };

    let mut nodes = vec![];
    for i in 0..items.length() {
        if let Some(node) = items.item(i) {
            nodes.push(node);
        }
    }

    // Find current focused element
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
