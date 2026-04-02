use leptix_core::direction::{Direction, use_direction};
use leptix_core::id::use_id;
use leptix_core::portal::Portal;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct MenubarContextValue {
    active_menu: RwSignal<Option<String>>,
    direction: Signal<Direction>,
    do_loop: Signal<bool>,
}

#[component]
pub fn Menubar(
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let direction = use_direction(dir);
    let do_loop = Signal::derive(move || r#loop.get().unwrap_or(true));
    let ctx = MenubarContextValue {
        active_menu: RwSignal::new(None),
        direction,
        do_loop,
    };

    view! {
        <Provider value=ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:role="menubar"
                attr:dir=move || direction.get().to_string()
                on:keydown=move |event: KeyboardEvent| {
                    let is_rtl = direction.get() == Direction::Rtl;
                    let do_loop = do_loop.get();
                    match event.key().as_str() {
                        "ArrowLeft" => { event.prevent_default(); focus_menubar_trigger(&event, is_rtl, do_loop); }
                        "ArrowRight" => { event.prevent_default(); focus_menubar_trigger(&event, !is_rtl, do_loop); }
                        _ => {}
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn MenubarMenu(
    #[prop(into, optional)] value: Option<String>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let id = value.unwrap_or_else(|| use_id(None).get());
    let ctx = expect_context::<MenubarContextValue>();
    let id_clone = id.clone();
    let open = Signal::derive(move || ctx.active_menu.get().as_deref() == Some(id_clone.as_str()));
    let menu_id = id.clone();

    let menu_ctx = MenubarMenuContextValue {
        open,
        on_open: Callback::new(move |()| ctx.active_menu.set(Some(menu_id.clone()))),
        on_close: Callback::new(move |()| ctx.active_menu.set(None)),
        content_id: format!("{}-content", id),
    };

    view! {
        <Provider value=menu_ctx>
            {children.with_value(|c| c())}
        </Provider>
    }
}

#[derive(Clone, Debug)]
struct MenubarMenuContextValue {
    open: Signal<bool>,
    on_open: Callback<()>,
    on_close: Callback<()>,
    content_id: String,
}

#[component]
pub fn MenubarTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenubarMenuContextValue>();

    view! {
        <Primitive element=html::button as_child=as_child node_ref=node_ref
            attr:r#type="button"
            attr:role="menuitem"
            attr:aria-haspopup="menu"
            attr:aria-expanded=move || ctx.open.get().to_string()
            attr:aria-controls=move || ctx.open.get().then(|| ctx.content_id.clone())
            attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
            on:click=move |_| {
                if ctx.open.get() { ctx.on_close.run(()); } else { ctx.on_open.run(()); }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn MenubarPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(into, optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] _force_mount: MaybeProp<bool>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenubarMenuContextValue>();
    view! {
        <Show when=move || ctx.open.get()>
            <Portal container=container container_ref=container_ref>
                {children.with_value(|c| c())}
            </Portal>
        </Show>
    }
}

#[component]
pub fn MenubarContent(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenubarMenuContextValue>();
    let present = Signal::derive(move || ctx.open.get());
    let presence = leptix_core::presence::use_presence(present);
    let dismiss_ref = leptix_core::dismissable_layer::use_dismissable_layer(
        None,
        None,
        None,
        None,
        Some(Callback::new(move |()| ctx.on_close.run(()))),
        Signal::derive(move || !ctx.open.get()),
    );
    let refs = leptix_core::compose_refs::use_composed_refs(vec![
        node_ref,
        presence.node_ref,
        dismiss_ref,
    ]);

    let search_buffer: RwSignal<String> = RwSignal::new(String::new());
    let search_timer: RwSignal<Option<i32>> = RwSignal::new(None);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive element=html::div as_child=as_child node_ref=refs
                attr:id=ctx.content_id.clone()
                attr:role="menu"
                attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
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

// Components that don't read MenuContextValue can remain re-exports.
pub use leptix_dropdown_menu::{
    DropdownMenuGroup as MenubarGroup, DropdownMenuLabel as MenubarLabel,
    DropdownMenuSeparator as MenubarSeparator,
};

// ---------------------------------------------------------------------------
// Native menubar item components — read MenubarMenuContextValue instead of
// the dropdown-menu-private MenuContextValue.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct MenubarItemCheckedContextValue {
    checked: Signal<bool>,
}

#[derive(Clone, Debug)]
struct MenubarRadioGroupContextValue {
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
}

#[derive(Clone, Debug)]
struct MenubarSubMenuContextValue {
    open: RwSignal<bool>,
    content_id: String,
    trigger_ref: AnyNodeRef,
}

#[component]
pub fn MenubarItem(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenubarMenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let handle_select = move || {
        if !disabled.get() {
            if let Some(on_select) = on_select {
                on_select.run(());
            }
            ctx.on_close.run(());
        }
    };

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="menuitem"
            attr:data-disabled=move || disabled.get().then_some("")
            attr:tabindex="-1"
            on:click=move |_| handle_select()
            on:keydown=move |event: KeyboardEvent| {
                if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get() {
                    event.prevent_default();
                    handle_select();
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn MenubarCheckboxItem(
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenubarMenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let checked = Signal::derive(move || checked.get().unwrap_or(false));

    let item_checked_ctx = MenubarItemCheckedContextValue { checked };

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
                        ctx.on_close.run(());
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get() {
                        event.prevent_default();
                        if let Some(cb) = on_checked_change { cb.run(!checked.get()); }
                        if let Some(cb) = on_select { cb.run(()); }
                        ctx.on_close.run(());
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn MenubarRadioGroup(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let value = Signal::derive(move || value.get());
    let radio_ctx = MenubarRadioGroupContextValue {
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
pub fn MenubarRadioItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenubarMenuContextValue>();
    let radio_ctx = expect_context::<MenubarRadioGroupContextValue>();
    let item_value = value.clone();
    let item_value_click = value.clone();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let checked =
        Signal::derive(move || radio_ctx.value.get().as_deref() == Some(item_value.as_str()));
    let item_checked_ctx = MenubarItemCheckedContextValue { checked };

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
                        ctx.on_close.run(());
                    }
                }
                on:keydown=move |event: KeyboardEvent| {
                    if matches!(event.key().as_str(), "Enter" | " ") && !disabled.get() {
                        event.prevent_default();
                        radio_ctx.on_value_change.run(value.clone());
                        if let Some(cb) = on_select { cb.run(()); }
                        ctx.on_close.run(());
                    }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn MenubarItemIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let checked_ctx = expect_context::<MenubarItemCheckedContextValue>();

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

#[component]
pub fn MenubarArrow(
    #[prop(into, optional)] width: MaybeProp<f64>,
    #[prop(into, optional)] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <leptix_core::popper::PopperArrow width=width height=height as_child=as_child node_ref=node_ref>
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </leptix_core::popper::PopperArrow>
    }
}

#[component]
pub fn MenubarSub(
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
    let sub_ctx = MenubarSubMenuContextValue {
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
pub fn MenubarSubTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let bar_ctx = expect_context::<MenubarContextValue>();
    let sub_ctx = expect_context::<MenubarSubMenuContextValue>();
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let refs = leptix_core::compose_refs::use_composed_refs(vec![node_ref, sub_ctx.trigger_ref]);

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
                let open_key = if bar_ctx.direction.get() == Direction::Rtl { "ArrowLeft" } else { "ArrowRight" };
                if event.key() == open_key && !disabled.get() {
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
pub fn MenubarSubContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let bar_ctx = expect_context::<MenubarContextValue>();
    let sub_ctx = expect_context::<MenubarSubMenuContextValue>();
    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let present = Signal::derive(move || force_mount.get() || sub_ctx.open.get());
    let presence = leptix_core::presence::use_presence(present);

    let dismiss_ref = leptix_core::dismissable_layer::use_dismissable_layer(
        None,
        None,
        None,
        None,
        Some(Callback::new(move |()| sub_ctx.open.set(false))),
        Signal::derive(move || !sub_ctx.open.get()),
    );
    let refs = leptix_core::compose_refs::use_composed_refs(vec![
        node_ref,
        presence.node_ref,
        dismiss_ref,
    ]);

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive element=html::div as_child=as_child node_ref=refs
                attr:id=sub_ctx.content_id.clone()
                attr:role="menu"
                attr:aria-orientation="vertical"
                attr:data-state=move || if sub_ctx.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
                on:keydown=move |event: KeyboardEvent| {
                    let close_key = if bar_ctx.direction.get() == Direction::Rtl { "ArrowRight" } else { "ArrowLeft" };
                    match event.key().as_str() {
                        "ArrowDown" => { event.prevent_default(); focus_menu_item(&event, true); }
                        "ArrowUp" => { event.prevent_default(); focus_menu_item(&event, false); }
                        k if k == close_key => { event.prevent_default(); sub_ctx.open.set(false); }
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

fn focus_menu_item(event: &KeyboardEvent, forward: bool) {
    let Some(container) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };

    let Ok(items) =
        container.query_selector_all("[role='menuitem']:not([data-disabled]), [role='menuitemcheckbox']:not([data-disabled]), [role='menuitemradio']:not([data-disabled])")
    else {
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

fn focus_menubar_trigger(event: &KeyboardEvent, forward: bool, do_loop: bool) {
    let Some(bar) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };
    let Ok(items) = bar.query_selector_all("[role='menuitem']") else {
        return;
    };
    let mut nodes = vec![];
    for i in 0..items.length() {
        if let Some(n) = items.item(i) {
            nodes.push(n);
        }
    }
    if nodes.is_empty() {
        return;
    }
    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());
    let idx = active
        .as_ref()
        .and_then(|a| {
            nodes
                .iter()
                .position(|n| n == <web_sys::Element as AsRef<web_sys::Node>>::as_ref(a))
        })
        .unwrap_or(0);
    let next = if forward {
        let candidate = idx + 1;
        if candidate < nodes.len() {
            Some(candidate)
        } else if do_loop {
            Some(0)
        } else {
            None
        }
    } else {
        if idx > 0 {
            Some(idx - 1)
        } else if do_loop {
            Some(nodes.len() - 1)
        } else {
            None
        }
    };
    if let Some(next) = next
        && let Some(n) = nodes.get(next)
    {
        use web_sys::wasm_bindgen::JsCast;
        if let Ok(el) = n.clone().dyn_into::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}

const MENU_ITEM_SELECTOR: &str = "[role='menuitem']:not([data-disabled]), [role='menuitemcheckbox']:not([data-disabled]), [role='menuitemradio']:not([data-disabled])";

fn focus_menu_item_edge(event: &KeyboardEvent, first: bool) {
    let Some(container) = event.current_target().and_then(|t| {
        use web_sys::wasm_bindgen::JsCast;
        t.dyn_into::<web_sys::Element>().ok()
    }) else {
        return;
    };
    let Ok(items) = container.query_selector_all(MENU_ITEM_SELECTOR) else { return; };
    let target_idx = if first { 0 } else { items.length().saturating_sub(1) };
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
        && let Some(w) = web_sys::window() { w.clear_timeout_with_handle(id); }
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
    let Ok(items) = container.query_selector_all(MENU_ITEM_SELECTOR) else { return; };
    for i in 0..items.length() {
        if let Some(node) = items.item(i) {
            use web_sys::wasm_bindgen::JsCast;
            if let Some(text) = node.text_content()
                && text.trim().to_lowercase().starts_with(&search)
                    && let Ok(el) = node.dyn_into::<web_sys::HtmlElement>() {
                        let _ = el.focus();
                        return;
                    }
        }
    }
}
