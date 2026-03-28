use leptix_core::compose_refs::use_composed_refs;
use leptix_core::dismissable_layer::use_dismissable_layer;
use leptix_core::focus_scope::use_focus_scope;
use leptix_core::id::use_id;
use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
struct MenuContextValue {
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    trigger_ref: AnyNodeRef,
    content_id: String,
}

#[component]
pub fn DropdownMenu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] dir: MaybeProp<leptix_core::direction::Direction>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let _dir = dir;

    let (open_state, set_open) = leptix_core::use_controllable_state::use_controllable_state(
        leptix_core::use_controllable_state::UseControllableStateParams {
            prop: open,
            on_change: on_open_change.map(|cb| {
                Callback::new(move |v: Option<bool>| {
                    if let Some(v) = v {
                        cb.run(v);
                    }
                })
            }),
            default_prop: default_open,
        },
    );
    let open = Signal::derive(move || open_state.get().unwrap_or(false));
    let base_id = use_id(None).get();

    let context = MenuContextValue {
        open,
        on_open_change: Callback::new(move |v: bool| set_open.run(Some(v))),
        trigger_ref: AnyNodeRef::new(),
        content_id: format!("{}-content", base_id),
    };

    view! {
        <Provider value=context>
            {children.with_value(|c| c())}
        </Provider>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let refs = use_composed_refs(vec![node_ref, ctx.trigger_ref]);

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=refs
            attr:r#type="button"
            attr:aria-haspopup="menu"
            attr:aria-expanded=move || ctx.open.get().to_string()
            attr:aria-controls=move || ctx.open.get().then(|| ctx.content_id.clone())
            attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
            on:click=move |_| ctx.on_open_change.run(!ctx.open.get())
            on:keydown=move |event: KeyboardEvent| {
                match event.key().as_str() {
                    "ArrowDown" | "Enter" | " " => {
                        event.prevent_default();
                        ctx.on_open_change.run(true);
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
pub fn DropdownMenuPortal(children: TypedChildrenFn<impl IntoView + 'static>) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    view! { <Show when=move || ctx.open.get()>{children.with_value(|c| c())}</Show> }
}

#[component]
pub fn DropdownMenuContent(
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::PointerEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let present = Signal::derive(move || ctx.open.get());
    let presence = use_presence(present);

    let focus_ref = use_focus_scope(Signal::derive(|| true), Signal::derive(|| true), None, None);
    let dismiss_ref = use_dismissable_layer(
        on_escape_key_down,
        on_pointer_down_outside,
        None,
        None,
        Some(Callback::new(move |()| ctx.on_open_change.run(false))),
        Signal::derive(move || !ctx.open.get()),
    );
    let refs = use_composed_refs(vec![node_ref, presence.node_ref, focus_ref, dismiss_ref]);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=refs
                attr:id=ctx.content_id.clone()
                attr:role="menu"
                attr:aria-orientation="vertical"
                attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
                on:keydown=move |event: KeyboardEvent| {
                    match event.key().as_str() {
                        "ArrowDown" => { event.prevent_default(); focus_next_item(&event, true); }
                        "ArrowUp" => { event.prevent_default(); focus_next_item(&event, false); }
                        _ => {}
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Show>
    }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:role="menuitem"
            attr:data-disabled=move || disabled.get().then_some("")
            attr:tabindex="-1"
            on:click=move |_| {
                if !disabled.get() {
                    if let Some(on_select) = on_select { on_select.run(()); }
                    ctx.on_open_change.run(false);
                }
            }
            on:keydown=move |event: KeyboardEvent| {
                if (event.key() == "Enter" || event.key() == " ") && !disabled.get() {
                    event.prevent_default();
                    if let Some(on_select) = on_select { on_select.run(()); }
                    ctx.on_open_change.run(false);
                }
            }
            on:pointerenter=move |_| {
                if !disabled.get()
                    && let Some(target) = web_sys::window()
                        .and_then(|w| w.document())
                        .and_then(|d| d.active_element())
                {
                    use web_sys::wasm_bindgen::JsCast;
                    if let Ok(el) = target.dyn_into::<web_sys::HtmlElement>() { let _ = el.blur(); }
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn DropdownMenuSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! { <Primitive element=html::div as_child=as_child node_ref=node_ref attr:role="separator">{""}</Primitive> }
}

#[component]
pub fn DropdownMenuLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c())}
        </Primitive>
    }
}

fn focus_next_item(event: &KeyboardEvent, forward: bool) {
    let Some(container) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    let Ok(items) = container.query_selector_all("[role='menuitem']:not([data-disabled])") else {
        return;
    };
    let mut nodes = vec![];
    for i in 0..items.length() {
        if let Some(n) = items.item(i) {
            nodes.push(n);
        }
    }

    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());
    let idx = active.as_ref().and_then(|a| {
        use web_sys::wasm_bindgen::JsCast;
        let n: &web_sys::Node = a.unchecked_ref();
        nodes.iter().position(|node| node == n)
    });

    let next = if forward {
        idx.map(|i| i + 1).filter(|i| *i < nodes.len()).or(Some(0))
    } else {
        idx.and_then(|i| i.checked_sub(1))
            .or(Some(nodes.len().saturating_sub(1)))
    };

    if let Some(idx) = next
        && let Some(node) = nodes.get(idx)
    {
        use web_sys::wasm_bindgen::JsCast;
        if let Ok(el) = node.clone().dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}
