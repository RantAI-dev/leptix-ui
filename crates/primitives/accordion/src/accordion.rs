use leptix_collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use leptix_core::direction::{Direction, use_direction};
use leptix_core::id::use_id;
use leptix_core::primitive::Primitive;
use leptix_core::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccordionType {
    Single,
    Multiple,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct AccordionContextValue {
    accordion_type: AccordionType,
    value: Signal<Vec<String>>,
    on_item_toggle: Callback<String>,
    disabled: Signal<bool>,
    orientation: Signal<String>,
    direction: Signal<Direction>,
    collapsible: bool,
}

#[component]
pub fn Accordion(
    #[prop(into)] r#type: AccordionType,
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    /// When type is Single, allows closing content when clicking trigger for an open item.
    #[prop(into, optional)]
    collapsible: Option<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let orientation = Signal::derive(move || orientation.get().unwrap_or("vertical".into()));
    let direction = use_direction(dir);
    let collapsible_flag = collapsible.unwrap_or(false);
    let accordion_type = r#type;

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

    let context_value = AccordionContextValue {
        accordion_type,
        value,
        on_item_toggle: Callback::new(move |item_value: String| {
            let current = value.get();
            let next = match accordion_type {
                AccordionType::Single => {
                    if current.contains(&item_value) {
                        if collapsible_flag { vec![] } else { current }
                    } else {
                        vec![item_value]
                    }
                }
                AccordionType::Multiple => {
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
        orientation,
        direction,
        collapsible: collapsible_flag,
    };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-orientation=move || orientation.get()
                attr:data-disabled=move || disabled.get().then_some("")
                on:keydown=move |event: KeyboardEvent| {
                    let is_vertical = orientation.get() == "vertical";
                    let is_horizontal = orientation.get() == "horizontal";
                    let is_rtl = direction.get() == Direction::Rtl;

                    let next = match event.key().as_str() {
                        "ArrowUp" if is_vertical => Some(false),
                        "ArrowDown" if is_vertical => Some(true),
                        "ArrowLeft" if is_horizontal => Some(is_rtl),
                        "ArrowRight" if is_horizontal => Some(!is_rtl),
                        "Home" => {
                            event.prevent_default();
                            focus_accordion_trigger(&event, true, true);
                            None
                        }
                        "End" => {
                            event.prevent_default();
                            focus_accordion_trigger(&event, false, true);
                            None
                        }
                        _ => None,
                    };

                    if let Some(forward) = next {
                        event.prevent_default();
                        focus_accordion_trigger(&event, forward, false);
                    }
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct AccordionItemContextValue {
    value: String,
    trigger_id: String,
    content_id: String,
}

#[component]
pub fn AccordionItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let context = expect_context::<AccordionContextValue>();
    let item_value = value.clone();
    let item_disabled =
        Signal::derive(move || context.disabled.get() || disabled.get().unwrap_or(false));
    let is_open = Signal::derive(move || context.value.get().contains(&item_value));

    let base_id = use_id(None).get();
    let item_context = AccordionItemContextValue {
        value: value.clone(),
        trigger_id: format!("{}-trigger", base_id),
        content_id: format!("{}-content", base_id),
    };

    let children = StoredValue::new(children.into_inner());

    view! {
        <Provider value=item_context>
            <Collapsible
                open=is_open
                disabled=item_disabled
                as_child=as_child
                node_ref=node_ref
                on_open_change=Callback::new(move |_open: bool| {
                    context.on_item_toggle.run(value.clone());
                })
            >
                {children.with_value(|children| children())}
            </Collapsible>
        </Provider>
    }
}

#[component]
pub fn AccordionTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let _item_context = expect_context::<AccordionItemContextValue>();
    let _accordion_context = expect_context::<AccordionContextValue>();

    // Note: AccordionHeader provides the <h3> wrapper. AccordionTrigger should NOT
    // add its own <h3> to avoid double-nesting (invalid HTML).
    view! {
        <CollapsibleTrigger
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children())}
        </CollapsibleTrigger>
    }
}

#[component]
pub fn AccordionContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let item_context = expect_context::<AccordionItemContextValue>();
    let _accordion_context = expect_context::<AccordionContextValue>();
    let trigger_id = item_context.trigger_id.clone();

    view! {
        <CollapsibleContent
            force_mount=force_mount
            as_child=as_child
            node_ref=node_ref
            attr:aria-labelledby=trigger_id
        >
            {children.with_value(|children| children())}
        </CollapsibleContent>
    }
}

#[component]
pub fn AccordionHeader(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let accordion_context = expect_context::<AccordionContextValue>();
    let item_context = expect_context::<AccordionItemContextValue>();

    view! {
        <Primitive element=html::h3 as_child=as_child node_ref=node_ref
            attr:data-orientation=move || accordion_context.orientation.get()
            attr:data-state=move || if accordion_context.value.get().contains(&item_context.value) { "open" } else { "closed" }
            attr:data-disabled=move || accordion_context.disabled.get().then_some("")
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

fn focus_accordion_trigger(event: &KeyboardEvent, forward: bool, jump_to_end: bool) {
    let target = event.current_target();
    let Some(container) = target.and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    let Ok(items) = container.query_selector_all(
        "button[data-leptix-collapsible-trigger]:not([disabled]), [role='button']:not([disabled])",
    ) else {
        // Fallback: try generic button triggers
        let Ok(items) = container.query_selector_all("h3 > button:not([disabled])") else {
            return;
        };
        focus_nth_node(&items, forward, jump_to_end);
        return;
    };

    focus_nth_node(&items, forward, jump_to_end);
}

fn focus_nth_node(items: &web_sys::NodeList, forward: bool, jump_to_end: bool) {
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

    let next_index = if jump_to_end {
        if forward {
            Some(0)
        } else {
            Some(nodes.len().saturating_sub(1))
        }
    } else if forward {
        if current_index + 1 < nodes.len() {
            Some(current_index + 1)
        } else {
            None
        }
    } else if current_index > 0 {
        Some(current_index - 1)
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
