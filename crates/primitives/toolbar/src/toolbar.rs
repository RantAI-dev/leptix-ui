use leptix_core::direction::{Direction, use_direction};
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct ToolbarContextValue {
    orientation: Signal<String>,
    direction: Signal<Direction>,
}

#[component]
pub fn Toolbar(
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let orientation = Signal::derive(move || orientation.get().unwrap_or("horizontal".into()));
    let direction = use_direction(dir);
    let do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));

    let context_value = ToolbarContextValue {
        orientation,
        direction,
    };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:role="toolbar"
                attr:aria-orientation=move || orientation.get()
                attr:data-orientation=move || orientation.get()
                attr:dir=move || direction.get().to_string()
                on:keydown=move |event: KeyboardEvent| {
                    let is_horizontal = orientation.get() == "horizontal";
                    let is_rtl = direction.get() == Direction::Rtl;

                    let next = match event.key().as_str() {
                        "ArrowLeft" if is_horizontal => Some(is_rtl),
                        "ArrowRight" if is_horizontal => Some(!is_rtl),
                        "ArrowUp" if !is_horizontal => Some(false),
                        "ArrowDown" if !is_horizontal => Some(true),
                        "Home" => {
                            event.prevent_default();
                            focus_toolbar_item(&event, true, true);
                            None
                        }
                        "End" => {
                            event.prevent_default();
                            focus_toolbar_item(&event, false, true);
                            None
                        }
                        _ => None,
                    };

                    if let Some(forward) = next {
                        event.prevent_default();
                        focus_toolbar_item(&event, forward, do_loop.get());
                    }
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ToolbarButton(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn ToolbarLink(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <Primitive
            element=html::a
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn ToolbarSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ToolbarContextValue>();

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:role="separator"
            attr:aria-orientation=move || {
                match context.orientation.get().as_str() {
                    "horizontal" => "vertical",
                    _ => "horizontal",
                }
            }
            attr:data-orientation=move || {
                match context.orientation.get().as_str() {
                    "horizontal" => "vertical",
                    _ => "horizontal",
                }
            }
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

fn focus_toolbar_item(event: &KeyboardEvent, forward: bool, do_loop: bool) {
    let target = event.current_target();
    let Some(toolbar) = target.and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    let selector = "button:not([disabled]), a[href], input:not([disabled]), [tabindex]:not([tabindex='-1']):not([disabled])";
    let Ok(items) = toolbar.query_selector_all(selector) else {
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
