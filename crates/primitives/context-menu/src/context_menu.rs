use leptix_core::compose_refs::use_composed_refs;
use leptix_core::dismissable_layer::use_dismissable_layer;
use leptix_core::focus_scope::use_focus_scope;
use leptix_core::id::use_id;
use leptix_core::presence::use_presence;
use leptix_core::primitive::Primitive;
use leptos::{context::Provider, ev::KeyboardEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Debug)]
struct ContextMenuContextValue {
    open: RwSignal<bool>,
    content_id: String,
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
                    ctx.open.set(true);
                }
            }
        >
            {children.with_value(|c| c())}
        </Primitive>
    }
}

#[component]
pub fn ContextMenuPortal(children: TypedChildrenFn<impl IntoView + 'static>) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ContextMenuContextValue>();
    view! { <Show when=move || ctx.open.get()>{children.with_value(|c| c())}</Show> }
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
                on:keydown=move |event: KeyboardEvent| {
                    if event.key() == "ArrowDown" { event.prevent_default(); focus_menu_item(&event, true); }
                    else if event.key() == "ArrowUp" { event.prevent_default(); focus_menu_item(&event, false); }
                }
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Show>
    }
}

pub use leptix_dropdown_menu::{
    DropdownMenuItem as ContextMenuItem, DropdownMenuLabel as ContextMenuLabel,
    DropdownMenuSeparator as ContextMenuSeparator,
};

fn focus_menu_item(event: &KeyboardEvent, forward: bool) {
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
