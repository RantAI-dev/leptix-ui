use leptix_core::direction::{Direction, use_direction};
use leptix_core::id::use_id;
use leptix_core::primitive::Primitive;
use leptix_core::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
struct TabsContextValue {
    base_id: String,
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
    orientation: Signal<String>,
    direction: Signal<Direction>,
    activation_mode: ActivationMode,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ActivationMode {
    #[default]
    Automatic,
    Manual,
}

#[component]
pub fn Tabs(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] activation_mode: Option<ActivationMode>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let orientation = Signal::derive(move || orientation.get().unwrap_or("horizontal".into()));
    let direction = use_direction(dir);
    let activation_mode = activation_mode.unwrap_or_default();

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

    let base_id = use_id(None).get();

    let context_value = TabsContextValue {
        base_id,
        value,
        on_value_change: Callback::new(move |val: String| {
            set_value.run(Some(val));
        }),
        orientation,
        direction,
        activation_mode,
    };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-orientation=move || orientation.get()
                attr:dir=move || direction.get().to_string()
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn TabsList(
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TabsContextValue>();
    let do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:role="tablist"
            attr:aria-orientation=move || context.orientation.get()
            attr:data-orientation=move || context.orientation.get()
            on:keydown=move |event: KeyboardEvent| {
                let is_vertical = context.orientation.get() == "vertical";
                let is_horizontal = context.orientation.get() == "horizontal";
                let is_rtl = context.direction.get() == Direction::Rtl;

                let next = match event.key().as_str() {
                    "ArrowUp" if is_vertical => Some(false),
                    "ArrowDown" if is_vertical => Some(true),
                    "ArrowLeft" if is_horizontal => Some(is_rtl),
                    "ArrowRight" if is_horizontal => Some(!is_rtl),
                    "Home" => {
                        event.prevent_default();
                        focus_tab_trigger(&event, true, true);
                        None
                    }
                    "End" => {
                        event.prevent_default();
                        focus_tab_trigger(&event, false, true);
                        None
                    }
                    _ => None,
                };

                if let Some(forward) = next {
                    event.prevent_default();
                    roving_focus_tabs(&event, forward, do_loop.get());
                }
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn TabsTrigger(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TabsContextValue>();
    let trigger_value = value.clone();
    let trigger_value_click = value.clone();
    let trigger_value_focus = value.clone();
    let trigger_value_key = value.clone();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let is_selected =
        Signal::derive(move || context.value.get().as_deref() == Some(trigger_value.as_str()));

    let trigger_id = format!("{}-trigger-{}", context.base_id, value);
    let content_id = format!("{}-content-{}", context.base_id, value);

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
            attr:role="tab"
            attr:id=trigger_id
            attr:aria-selected=move || is_selected.get().to_string()
            attr:aria-controls=content_id
            attr:data-state=move || if is_selected.get() { "active" } else { "inactive" }
            attr:data-orientation=move || context.orientation.get()
            attr:data-disabled=move || disabled.get().then_some("")
            attr:disabled=move || disabled.get().then_some("")
            attr:tabindex=move || if is_selected.get() { "0" } else { "-1" }
            on:mousedown=move |event: leptos::ev::MouseEvent| {
                if !disabled.get() && event.button() == 0 && !event.ctrl_key() {
                    context.on_value_change.run(trigger_value_click.clone());
                } else {
                    event.prevent_default();
                }
            }
            on:keydown=move |event: KeyboardEvent| {
                if event.key() == " " || event.key() == "Enter" {
                    event.prevent_default();
                    context.on_value_change.run(trigger_value_key.clone());
                }
            }
            on:focus=move |_| {
                if context.activation_mode == ActivationMode::Automatic && !disabled.get() {
                    context.on_value_change.run(trigger_value_focus.clone());
                }
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn TabsContent(
    #[prop(into)] value: String,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<TabsContextValue>();
    let content_value = value.clone();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));

    let is_selected =
        Signal::derive(move || context.value.get().as_deref() == Some(content_value.as_str()));

    let trigger_id = format!("{}-trigger-{}", context.base_id, value);
    let content_id = format!("{}-content-{}", context.base_id, value);

    view! {
        <Show when=move || force_mount.get() || is_selected.get()>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:id=content_id.clone()
                attr:role="tabpanel"
                attr:aria-labelledby=trigger_id.clone()
                attr:data-state=move || if is_selected.get() { "active" } else { "inactive" }
                attr:data-orientation=move || context.orientation.get()
                attr:tabindex="0"
                attr:hidden=move || (!is_selected.get()).then_some("")
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Show>
    }
}

fn roving_focus_tabs(event: &KeyboardEvent, forward: bool, do_loop: bool) {
    focus_tab_trigger(event, forward, do_loop);
}

fn focus_tab_trigger(event: &KeyboardEvent, forward: bool, do_loop: bool) {
    let target = event.current_target();
    let Some(tablist) = target.and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    let Ok(items) = tablist.query_selector_all("[role='tab']:not([disabled])") else {
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
