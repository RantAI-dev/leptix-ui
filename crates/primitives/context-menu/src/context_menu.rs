use leptix_core::compose_refs::use_composed_refs;
use leptix_core::dismissable_layer::use_dismissable_layer;
use leptix_core::focus_scope::use_focus_scope;
use leptix_core::id::use_id;
use leptix_core::portal::Portal;
use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
struct ContextMenuContextValue {
    open: RwSignal<bool>,
    content_id: String,
    position_x: RwSignal<f64>,
    position_y: RwSignal<f64>,
}

#[component]
pub fn ContextMenu(
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let open = RwSignal::new(false);
    let base_id = use_id(None).get();

    Effect::new(move |_| {
        if let Some(cb) = on_open_change {
            cb.run(open.get());
        }
    });

    let ctx = ContextMenuContextValue {
        open,
        content_id: format!("{}-ctx", base_id),
        position_x: RwSignal::new(0.0),
        position_y: RwSignal::new(0.0),
    };

    view! {
        <Provider value=ctx>
            {children.with_value(|c| c())}
        </Provider>
    }
}

#[component]
pub fn ContextMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ContextMenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
            attr:data-disabled=move || disabled.get().then_some("")
            on:contextmenu=move |event: web_sys::MouseEvent| {
                if !disabled.get() {
                    event.prevent_default();
                    // Use page coordinates (includes scroll offset) with position:absolute
                    // for correct positioning regardless of scroll position.
                    ctx.position_x.set(event.page_x() as f64);
                    ctx.position_y.set(event.page_y() as f64);
                    ctx.open.set(true);
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ContextMenuPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(into, optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] _force_mount: MaybeProp<bool>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ContextMenuContextValue>();
    let ctx_for_portal = StoredValue::new(ctx.clone());
    view! {
        <Show when=move || ctx.open.get()>
            <Portal container=container container_ref=container_ref>
                <Provider value=ctx_for_portal.get_value()>
                    {children.with_value(|c| c())}
                </Provider>
            </Portal>
        </Show>
    }
}

#[component]
pub fn ContextMenuContent(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ContextMenuContextValue>();
    let open = Signal::derive(move || ctx.open.get());
    let presence = use_presence(open);

    let focus_ref = use_focus_scope(Signal::derive(|| true), Signal::derive(|| true), None, None);
    let dismiss_ref = use_dismissable_layer(
        None,
        None,
        None,
        None,
        Some(Callback::new(move |()| ctx.open.set(false))),
        Signal::derive(move || !ctx.open.get()),
    );
    let refs = use_composed_refs(vec![node_ref, presence.node_ref, focus_ref, dismiss_ref]);
    let search_buffer: RwSignal<String> = RwSignal::new(String::new());
    let search_timer: RwSignal<Option<i32>> = RwSignal::new(None);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=refs
                attr:id=ctx.content_id.clone()
                attr:role="menu"
                attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
                attr:style=move || format!("position:absolute;left:{}px;top:{}px;z-index:50;", ctx.position_x.get(), ctx.position_y.get())
                on:keydown=move |event: KeyboardEvent| {
                    match event.key().as_str() {
                        "Tab" => { event.prevent_default(); }
                        "ArrowDown" | "PageDown" => { event.prevent_default(); focus_menu_item(&event, true); }
                        "ArrowUp" | "PageUp" => { event.prevent_default(); focus_menu_item(&event, false); }
                        "Home" => { event.prevent_default(); focus_menu_item_edge(&event, true); }
                        "End" => { event.prevent_default(); focus_menu_item_edge(&event, false); }
                        key if key.len() == 1 && !event.ctrl_key() && !event.meta_key() => {
                            handle_typeahead(&event, key, search_buffer, search_timer);
                        }
                        _ => {}
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Show>
    }
}

// Simple item components that use ContextMenuContextValue directly
// (cannot re-export from dropdown-menu because it expects MenuContextValue)

#[component]
pub fn ContextMenuItem(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ContextMenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="menuitem"
            attr:data-disabled=move || disabled.get().then_some("")
            attr:tabindex="-1"
            on:click=move |_| {
                if !disabled.get() {
                    if let Some(cb) = on_select { cb.run(()); }
                    ctx.open.set(false);
                }
            }
            on:keydown=move |event: KeyboardEvent| {
                if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get() {
                    event.prevent_default();
                    if let Some(cb) = on_select { cb.run(()); }
                    ctx.open.set(false);
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ContextMenuSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="separator"
            attr:aria-orientation="horizontal"
        >
            {""}
        </Primitive>
    }
}

#[component]
pub fn ContextMenuLabel(
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

#[component]
pub fn ContextMenuGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref attr:role="group">
            {children.with_value(|c| c())}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// CheckboxItem
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct ContextMenuItemCheckedContextValue {
    checked: Signal<bool>,
}

#[component]
pub fn ContextMenuCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ContextMenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let checked = Signal::derive(move || checked.get().unwrap_or(false));

    let item_checked_ctx = ContextMenuItemCheckedContextValue { checked };

    view! {
        <Provider value=item_checked_ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:role="menuitemcheckbox"
                attr:aria-checked=move || checked.get().to_string()
                attr:data-state=move || if checked.get() { "checked" } else { "unchecked" }
                attr:data-disabled=move || disabled.get().then_some("")
                attr:tabindex="-1"
                on:click=move |_| {
                    if !disabled.get() {
                        if let Some(cb) = on_checked_change { cb.run(!checked.get()); }
                        if let Some(cb) = on_select { cb.run(()); }
                        ctx.open.set(false);
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get() {
                        event.prevent_default();
                        if let Some(cb) = on_checked_change { cb.run(!checked.get()); }
                        if let Some(cb) = on_select { cb.run(()); }
                        ctx.open.set(false);
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

// ---------------------------------------------------------------------------
// RadioGroup + RadioItem
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct ContextMenuRadioGroupContextValue {
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
}

#[component]
pub fn ContextMenuRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let value = Signal::derive(move || value.get());
    let radio_ctx = ContextMenuRadioGroupContextValue {
        value,
        on_value_change: Callback::new(move |v: String| {
            if let Some(cb) = on_value_change {
                cb.run(v);
            }
        }),
    };

    view! {
        <Provider value=radio_ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref attr:role="group">
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ContextMenuRadioItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ContextMenuContextValue>();
    let radio_ctx = expect_context::<ContextMenuRadioGroupContextValue>();
    let item_value = value.clone();
    let item_value_click = value.clone();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let checked =
        Signal::derive(move || radio_ctx.value.get().as_deref() == Some(item_value.as_str()));
    let item_checked_ctx = ContextMenuItemCheckedContextValue { checked };

    view! {
        <Provider value=item_checked_ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:role="menuitemradio"
                attr:aria-checked=move || checked.get().to_string()
                attr:data-state=move || if checked.get() { "checked" } else { "unchecked" }
                attr:data-disabled=move || disabled.get().then_some("")
                attr:tabindex="-1"
                on:click=move |_| {
                    if !disabled.get() {
                        radio_ctx.on_value_change.run(item_value_click.clone());
                        if let Some(cb) = on_select { cb.run(()); }
                        ctx.open.set(false);
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get() {
                        event.prevent_default();
                        radio_ctx.on_value_change.run(value.clone());
                        if let Some(cb) = on_select { cb.run(()); }
                        ctx.open.set(false);
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

// ---------------------------------------------------------------------------
// ItemIndicator
// ---------------------------------------------------------------------------

#[component]
pub fn ContextMenuItemIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let checked_ctx = expect_context::<ContextMenuItemCheckedContextValue>();

    view! {
        <Show when=move || force_mount.get() || checked_ctx.checked.get()>
            <Primitive element=html::span as_child=as_child node_ref=node_ref
                attr:data-state=move || if checked_ctx.checked.get() { "checked" } else { "unchecked" }
            >
                {children.with_value(|c| c.as_ref().map(|c| c()))}
            </Primitive>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Sub / SubTrigger / SubContent
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct ContextMenuSubContextValue {
    open: RwSignal<bool>,
    content_id: String,
    trigger_ref: AnyNodeRef,
}

#[component]
pub fn ContextMenuSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let open_state = RwSignal::new(open.get().or(default_open.get()).unwrap_or(false));

    Effect::new(move |_| {
        if let Some(o) = open.get() {
            open_state.set(o);
        }
    });
    Effect::new(move |_| {
        if let Some(cb) = on_open_change {
            cb.run(open_state.get());
        }
    });

    let base_id = use_id(None).get();
    let sub_ctx = ContextMenuSubContextValue {
        open: open_state,
        content_id: format!("{}-sub", base_id),
        trigger_ref: AnyNodeRef::new(),
    };

    view! {
        <Provider value=sub_ctx>
            {children.with_value(|c| c())}
        </Provider>
    }
}

#[component]
pub fn ContextMenuSubTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let sub_ctx = expect_context::<ContextMenuSubContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let refs = use_composed_refs(vec![node_ref, sub_ctx.trigger_ref]);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=refs
            attr:role="menuitem"
            attr:aria-haspopup="menu"
            attr:aria-expanded=move || sub_ctx.open.get().to_string()
            attr:aria-controls=move || sub_ctx.open.get().then(|| sub_ctx.content_id.clone())
            attr:data-state=move || if sub_ctx.open.get() { "open" } else { "closed" }
            attr:data-disabled=move || disabled.get().then_some("")
            attr:tabindex="-1"
            on:click=move |_| {
                if !disabled.get() { sub_ctx.open.set(!sub_ctx.open.get()); }
            }
            on:pointerenter=move |_| {
                if !disabled.get() { sub_ctx.open.set(true); }
            }
            on:keydown=move |event: KeyboardEvent| {
                if event.key() == "ArrowRight" && !disabled.get() {
                    event.prevent_default();
                    sub_ctx.open.set(true);
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ContextMenuSubContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let sub_ctx = expect_context::<ContextMenuSubContextValue>();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let present = Signal::derive(move || force_mount.get() || sub_ctx.open.get());
    let presence = use_presence(present);

    let dismiss_ref = use_dismissable_layer(
        None,
        None,
        None,
        None,
        Some(Callback::new(move |()| sub_ctx.open.set(false))),
        Signal::derive(move || !sub_ctx.open.get()),
    );
    let refs = use_composed_refs(vec![node_ref, presence.node_ref, dismiss_ref]);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive element=html::div as_child=as_child node_ref=refs
                attr:id=sub_ctx.content_id.clone()
                attr:role="menu"
                attr:aria-orientation="vertical"
                attr:data-state=move || if sub_ctx.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
                on:keydown=move |event: KeyboardEvent| {
                    match event.key().as_str() {
                        "ArrowDown" => { event.prevent_default(); focus_menu_item(&event, true); }
                        "ArrowUp" => { event.prevent_default(); focus_menu_item(&event, false); }
                        "ArrowLeft" => { event.prevent_default(); sub_ctx.open.set(false); }
                        "Escape" => { sub_ctx.open.set(false); }
                        _ => {}
                    }
                }
                on:pointerleave=move |_| { sub_ctx.open.set(false); }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Show>
    }
}

// ---------------------------------------------------------------------------
// Arrow
// ---------------------------------------------------------------------------

#[component]
pub fn ContextMenuArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    // Context menus use fixed positioning, not Popper, so Arrow is a no-op placeholder.
    // It renders a span for API compatibility.
    let _width = width;
    let _height = height;
    view! {
        <Primitive element=html::span as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn focus_menu_item(event: &KeyboardEvent, forward: bool) {
    let Some(container) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };
    let Ok(items) = container.query_selector_all(
        "[role='menuitem']:not([data-disabled]), [role='menuitemcheckbox']:not([data-disabled]), [role='menuitemradio']:not([data-disabled])"
    ) else {
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
        nodes
            .iter()
            .position(|n| n == <web_sys::Element as AsRef<web_sys::Node>>::as_ref(a))
    });
    let next = if forward {
        idx.map(|i| i + 1).filter(|i| *i < nodes.len()).or(Some(0))
    } else {
        idx.and_then(|i| i.checked_sub(1))
            .or(Some(nodes.len().saturating_sub(1)))
    };
    if let Some(i) = next
        && let Some(n) = nodes.get(i)
    {
        use web_sys::wasm_bindgen::JsCast;
        if let Ok(el) = n.clone().dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}

fn focus_menu_item_edge(event: &KeyboardEvent, first: bool) {
    let Some(container) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };
    let Ok(items) = container.query_selector_all(MENU_ITEM_SELECTOR) else {
        return;
    };
    let target_idx = if first {
        0
    } else {
        items.length().saturating_sub(1)
    };
    if let Some(node) = items.item(target_idx) {
        use web_sys::wasm_bindgen::JsCast;
        if let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}

fn handle_typeahead(
    event: &KeyboardEvent,
    key: &str,
    search_buffer: RwSignal<String>,
    search_timer: RwSignal<Option<i32>>,
) {
    let Some(container) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };
    if let Some(id) = search_timer.get_untracked()
        && let Some(w) = web_sys::window()
    {
        w.clear_timeout_with_handle(id);
    }
    search_buffer.update(|buf| buf.push_str(key));
    let id = web_sys::window().and_then(|w| {
        w.set_timeout_with_callback_and_timeout_and_arguments_0(
            web_sys::wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                search_buffer.set(String::new());
            })
            .into_js_value()
            .unchecked_ref(),
            1000,
        )
        .ok()
    });
    search_timer.set(id);

    let search = search_buffer.get_untracked().to_lowercase();
    let Ok(items) = container.query_selector_all(MENU_ITEM_SELECTOR) else {
        return;
    };
    for i in 0..items.length() {
        if let Some(node) = items.item(i) {
            use web_sys::wasm_bindgen::JsCast;
            if let Some(text) = node.text_content()
                && text.trim().to_lowercase().starts_with(&search)
                && let Ok(el) = node.dyn_into::<web_sys::HtmlElement>()
            {
                let _ = el.focus();
                return;
            }
        }
    }
}

const MENU_ITEM_SELECTOR: &str = "[role='menuitem']:not([data-disabled]), [role='menuitemcheckbox']:not([data-disabled]), [role='menuitemradio']:not([data-disabled])";
