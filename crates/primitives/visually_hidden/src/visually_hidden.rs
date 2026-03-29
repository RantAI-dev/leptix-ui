use leptix_core::primitive::Primitive;
use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

#[component]
pub fn VisuallyHidden(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:style="position:absolute;border:0;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;word-wrap:normal"
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}
