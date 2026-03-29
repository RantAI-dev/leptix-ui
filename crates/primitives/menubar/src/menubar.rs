use leptix_core::direction::{Direction, use_direction};
use leptix_core::id::use_id;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

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
pub fn MenubarPortal(children: TypedChildrenFn<impl IntoView + 'static>) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<MenubarMenuContextValue>();
    view! { <Show when=move || ctx.open.get()>{children.with_value(|c| c())}</Show> }
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

    view! {
        <Show when=move || presence.is_present.get()>
            <Primitive element=html::div as_child=as_child node_ref=refs
                attr:id=ctx.content_id.clone()
                attr:role="menu"
                attr:data-state=move || if ctx.open.get() { "open" } else { "closed" }
                attr:tabindex="-1"
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Show>
    }
}

pub use leptix_dropdown_menu::{
    DropdownMenuArrow as MenubarArrow, DropdownMenuCheckboxItem as MenubarCheckboxItem,
    DropdownMenuGroup as MenubarGroup, DropdownMenuItem as MenubarItem,
    DropdownMenuItemIndicator as MenubarItemIndicator, DropdownMenuLabel as MenubarLabel,
    DropdownMenuRadioGroup as MenubarRadioGroup, DropdownMenuRadioItem as MenubarRadioItem,
    DropdownMenuSeparator as MenubarSeparator, DropdownMenuSub as MenubarSub,
    DropdownMenuSubContent as MenubarSubContent, DropdownMenuSubTrigger as MenubarSubTrigger,
};

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
