use leptix_core::primitive::Primitive;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScrollAreaType {
    #[default]
    Hover,
    Scroll,
    Auto,
    Always,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ScrollAreaContextValue {
    scroll_type: ScrollAreaType,
    dir: Signal<String>,
    viewport_ref: AnyNodeRef,
}

#[component]
pub fn ScrollArea(
    #[prop(into, optional)] r#type: Option<ScrollAreaType>,
    #[prop(into, optional)] dir: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let scroll_type = r#type.unwrap_or_default();
    let dir = Signal::derive(move || dir.get().unwrap_or("ltr".into()));

    let ctx = ScrollAreaContextValue {
        scroll_type,
        dir,
        viewport_ref: AnyNodeRef::new(),
    };

    view! {
        <Provider value=ctx>
            <Primitive element=html::div as_child=as_child node_ref=node_ref
                attr:dir=move || dir.get()
                attr:style="position:relative;overflow:hidden"
            >
                {children.with_value(|c| c())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn ScrollAreaViewport(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let ctx = expect_context::<ScrollAreaContextValue>();
    let refs = leptix_core::compose_refs::use_composed_refs(vec![node_ref, ctx.viewport_ref]);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=refs
            attr:data-leptix-scroll-area-viewport=""
            attr:style="overflow:scroll;height:100%;width:100%"
            attr:tabindex="0"
        >
            <div style="min-width:100%;display:table">
                {children.with_value(|c| c())}
            </div>
        </Primitive>
    }
}

#[component]
pub fn ScrollAreaScrollbar(
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let orientation = Signal::derive(move || orientation.get().unwrap_or("vertical".into()));

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:role="scrollbar"
            attr:aria-orientation=move || orientation.get()
            attr:data-orientation=move || orientation.get()
            attr:style=move || {
                if orientation.get() == "vertical" {
                    "position:absolute;top:0;right:0;bottom:0;width:8px"
                } else {
                    "position:absolute;bottom:0;left:0;right:0;height:8px"
                }
            }
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}

#[component]
pub fn ScrollAreaThumb(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:data-leptix-scroll-area-thumb=""
            attr:style="position:relative;border-radius:9999px;background:rgba(0,0,0,0.3)"
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}

#[component]
pub fn ScrollAreaCorner(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref
            attr:style="position:absolute;right:0;bottom:0"
        >
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </Primitive>
    }
}
