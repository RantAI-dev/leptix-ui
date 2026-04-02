use leptix_core::direction::{Direction, use_direction};
use leptix_core::id::use_id;
use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct NavigationMenuContextValue {
    value: RwSignal<Option<String>>,
    direction: Signal<Direction>,
    orientation: Signal<String>,
    base_id: String,
}

#[component]
pub fn NavigationMenu(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let direction = use_direction(dir);
    let orientation = Signal::derive(move || orientation.get().unwrap_or("horizontal".into()));
    let active = RwSignal::new(value.get().or(default_value.get()));
    let base_id = use_id(None).get();

    if let Some(cb) = on_value_change {
        Effect::new(move |_| {
            if let Some(v) = active.get() {
                cb.run(v);
            }
        });
    }

    let ctx = NavigationMenuContextValue {
        value: active,
        direction,
        orientation,
        base_id,
    };

    view! {
        <Provider value=ctx>
            <Primitive element=html::nav as_child=as_child node_ref=node_ref
                attr:aria-label="Main"
                attr:data-orientation=move || orientation.get()
                attr:dir=move || direction.get().to_string()
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn NavigationMenuList(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let ctx = expect_context::<NavigationMenuContextValue>();

    view! {
        <Primitive element=html::ul as_child=as_child node_ref=node_ref
            attr:data-orientation={move || ctx.orientation.get()}
            attr:style="list-style:none;margin:0;padding:0;display:flex"
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[derive(Clone, Debug)]
struct NavigationMenuItemContextValue {
    value: String,
}

#[component]
pub fn NavigationMenuItem(
    #[prop(into, optional)] value: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let value = value.unwrap_or_else(|| use_id(None).get());
    let item_ctx = NavigationMenuItemContextValue { value };

    view! {
        <Provider value=item_ctx>
            <Primitive element=html::li as_child=as_child node_ref=node_ref>
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn NavigationMenuTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<NavigationMenuContextValue>();
    let item_ctx = expect_context::<NavigationMenuItemContextValue>();
    let item_value = item_ctx.value.clone();
    let item_value2 = item_ctx.value.clone();
    let is_open = Signal::derive(move || ctx.value.get().as_deref() == Some(item_value.as_str()));

    let item_value_kb = item_ctx.value.clone();

    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            attr:aria-expanded=move || is_open.get().to_string()
            attr:data-state=move || if is_open.get() { "open" } else { "closed" }
            on:click=move |_| {
                if is_open.get() { ctx.value.set(None); } else { ctx.value.set(Some(item_value2.clone())); }
            }
            on:pointerenter=move |_| {
                ctx.value.set(Some(item_ctx.value.clone()));
            }
            on:keydown=move |event: KeyboardEvent| {
                let is_horizontal = ctx.orientation.get_untracked() != "vertical";
                let is_rtl = ctx.direction.get_untracked() == Direction::Rtl;
                match event.key().as_str() {
                    "ArrowDown" if is_horizontal => {
                        // Open the content panel
                        event.prevent_default();
                        ctx.value.set(Some(item_value_kb.clone()));
                    }
                    "ArrowRight" if is_horizontal => {
                        event.prevent_default();
                        focus_nav_trigger(&event, !is_rtl);
                    }
                    "ArrowLeft" if is_horizontal => {
                        event.prevent_default();
                        focus_nav_trigger(&event, is_rtl);
                    }
                    "Enter" | " " => {
                        event.prevent_default();
                        if is_open.get_untracked() { ctx.value.set(None); }
                        else { ctx.value.set(Some(item_value_kb.clone())); }
                    }
                    "Escape" => {
                        ctx.value.set(None);
                    }
                    _ => {}
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn NavigationMenuContent(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<NavigationMenuContextValue>();
    let item_ctx = expect_context::<NavigationMenuItemContextValue>();
    let item_value = item_ctx.value.clone();
    let is_open = Signal::derive(move || ctx.value.get().as_deref() == Some(item_value.as_str()));
    let presence = use_presence(is_open);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive element=html::div as_child=as_child
                node_ref=leptix_core::compose_refs::use_composed_refs(vec![node_ref, presence.node_ref])
                attr:data-state=move || if is_open.get() { "open" } else { "closed" }
                on:pointerleave=move |_| { ctx.value.set(None); }
                on:keydown=move |event: KeyboardEvent| {
                    if event.key() == "Escape" {
                        event.prevent_default();
                        ctx.value.set(None);
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Show>
    }
}

#[component]
pub fn NavigationMenuLink(
    #[prop(into, optional)] active: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<NavigationMenuContextValue>();
    let active = Signal::derive(move || active.get().unwrap_or(false));

    view! {
        <Primitive element=html::a as_child=as_child node_ref=node_ref
            attr:data-active=move || active.get().then_some("")
            on:click=move |_| {
                if let Some(cb) = on_select { cb.run(()); }
                ctx.value.set(None);
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn NavigationMenuViewport(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let ctx = expect_context::<NavigationMenuContextValue>();
    let is_open = Signal::derive(move || ctx.value.get().is_some());

    view! {
        <Show when=move || is_open.get()>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:data-state=move || if is_open.get() { "open" } else { "closed" }
                attr:data-orientation=move || ctx.orientation.get()
            >
                {children.with_value(|c| c.as_ref().map(|c| c()))}
            </Primitive>
        </Show>
    }
}

#[component]
pub fn NavigationMenuSub(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let parent_ctx = expect_context::<NavigationMenuContextValue>();
    let orientation = Signal::derive(move || orientation.get().unwrap_or("horizontal".into()));
    let active = RwSignal::new(value.get().or(default_value.get()));
    let base_id = use_id(None).get();

    if let Some(cb) = on_value_change {
        Effect::new(move |_| {
            if let Some(v) = active.get() {
                cb.run(v);
            }
        });
    }

    let ctx = NavigationMenuContextValue {
        value: active,
        direction: parent_ctx.direction,
        orientation,
        base_id,
    };

    view! {
        <Provider value=ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:data-orientation=move || orientation.get()
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn NavigationMenuIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let ctx = expect_context::<NavigationMenuContextValue>();
    let is_visible = Signal::derive(move || ctx.value.get().is_some());
    let presence = use_presence(is_visible);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive element=html::div as_child=as_child
                node_ref=leptix_core::compose_refs::use_composed_refs(vec![node_ref, presence.node_ref])
                attr:data-state=move || if is_visible.get() { "visible" } else { "hidden" }
                attr:data-orientation=move || ctx.orientation.get()
                attr:aria-hidden="true"
            >
                {children.with_value(|c| c.as_ref().map(|c| c()))}
            </Primitive>
        </Show>
    }
}

// Helper: focus the next/previous trigger button in the navigation menu.
fn focus_nav_trigger(event: &KeyboardEvent, forward: bool) {
    let Some(current) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };
    // Walk up to the nav element to find all trigger buttons
    let nav = {
        let mut el = current.clone();
        loop {
            if el.tag_name().eq_ignore_ascii_case("NAV") {
                break Some(el);
            }
            match el.parent_element() {
                Some(p) => el = p,
                None => break None,
            }
        }
    };
    let Some(nav) = nav else { return };
    let Ok(buttons) = nav.query_selector_all("button[aria-expanded]") else {
        return;
    };

    let mut nodes = vec![];
    for i in 0..buttons.length() {
        if let Some(n) = buttons.item(i) {
            nodes.push(n);
        }
    }

    use web_sys::wasm_bindgen::JsCast;
    let current_node: &web_sys::Node = current.unchecked_ref();
    let idx = nodes.iter().position(|n| n == current_node).unwrap_or(0);

    let next = if forward {
        if idx + 1 < nodes.len() { idx + 1 } else { 0 }
    } else if idx > 0 {
        idx - 1
    } else {
        nodes.len().saturating_sub(1)
    };

    if let Some(node) = nodes.get(next)
        && let Ok(el) = node.clone().dyn_into::<web_sys::HtmlElement>()
    {
        let _ = el.focus();
    }
}
