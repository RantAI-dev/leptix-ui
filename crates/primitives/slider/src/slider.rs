use leptix_core::direction::{Direction, use_direction};
use leptix_core::number::clamp;
use leptix_core::primitive::Primitive;
use leptix_core::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev::KeyboardEvent, ev::PointerEvent, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct SliderContextValue {
    min: Signal<f64>,
    max: Signal<f64>,
    step: Signal<f64>,
    value: Signal<Vec<f64>>,
    disabled: Signal<bool>,
    orientation: Signal<String>,
    direction: Signal<Direction>,
    inverted: Signal<bool>,
    on_value_change: Callback<Vec<f64>>,
    on_value_commit: Option<Callback<Vec<f64>>>,
    /// Index of the thumb currently being dragged.
    active_thumb: RwSignal<Option<usize>>,
    /// Auto-incrementing counter for thumb index assignment.
    thumb_count: RwSignal<usize>,
    /// Thumb alignment: "center" (default) or "contain" (stays within track bounds).
    thumb_alignment: Signal<String>,
}

#[component]
pub fn Slider(
    #[prop(into, optional)] value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] on_value_commit: Option<Callback<Vec<f64>>>,
    #[prop(into, optional, default = 0.0.into())] min: MaybeProp<f64>,
    #[prop(into, optional, default = 100.0.into())] max: MaybeProp<f64>,
    #[prop(into, optional, default = 1.0.into())] step: MaybeProp<f64>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] inverted: MaybeProp<bool>,
    /// Thumb alignment: "center" (default) or "contain" (thumb stays within track bounds).
    #[prop(into, optional)]
    thumb_alignment: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let min = Signal::derive(move || min.get().unwrap_or(0.0));
    let max = Signal::derive(move || max.get().unwrap_or(100.0));
    let step = Signal::derive(move || step.get().unwrap_or(1.0));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let orientation = Signal::derive(move || orientation.get().unwrap_or("horizontal".into()));
    let direction = use_direction(dir);
    let inverted = Signal::derive(move || inverted.get().unwrap_or(false));
    let thumb_alignment =
        Signal::derive(move || thumb_alignment.get().unwrap_or_else(|| "center".into()));

    let (value, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        on_change: on_value_change.map(|cb| {
            Callback::new(move |value: Option<Vec<f64>>| {
                if let Some(value) = value {
                    cb.run(value);
                }
            })
        }),
        default_prop: default_value,
    });
    let value = Signal::derive(move || value.get().unwrap_or_else(|| vec![min.get()]));

    let context_value = SliderContextValue {
        min,
        max,
        step,
        value,
        disabled,
        orientation,
        direction,
        inverted,
        on_value_change: Callback::new(move |val: Vec<f64>| {
            set_value.run(Some(val));
        }),
        on_value_commit,
        active_thumb: RwSignal::new(None),
        thumb_count: RwSignal::new(0),
        thumb_alignment,
    };

    // Drag state lives at the root so it works from track, range, or thumb
    let dragging_idx = RwSignal::new(Option::<usize>::None);

    let calc_value_from_pointer =
        move |client_x: f64, client_y: f64, el: &web_sys::HtmlElement| -> f64 {
            let rect = el.get_bounding_client_rect();
            let is_horizontal = orientation.get() == "horizontal";
            let is_inv = inverted.get();
            let percent = if is_horizontal {
                let offset = client_x - rect.left();
                let ratio = offset / rect.width();
                let ratio = if direction.get() == Direction::Rtl {
                    1.0 - ratio
                } else {
                    ratio
                };
                if is_inv { 1.0 - ratio } else { ratio }
            } else {
                let offset = client_y - rect.top();
                let ratio = 1.0 - (offset / rect.height());
                if is_inv { 1.0 - ratio } else { ratio }
            };
            let new_value = min.get() + percent * (max.get() - min.get());
            let new_value = snap_to_step(new_value, step.get(), min.get());
            clamp(new_value, [min.get(), max.get()])
        };

    view! {
        <Provider value=context_value>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-orientation=move || orientation.get()
                attr:data-disabled=move || disabled.get().then_some("")
                attr:aria-disabled=move || disabled.get().to_string()
                attr:dir=move || direction.get().to_string()
                attr:style="touch-action:none;position:relative"
                on:pointerdown=move |event: PointerEvent| {
                    if disabled.get() {
                        return;
                    }
                    event.prevent_default();
                    if let Some(target) = event.current_target()
                        && let Ok(el) = target.dyn_into::<web_sys::HtmlElement>()
                    {
                        let _ = el.set_pointer_capture(event.pointer_id());

                        let new_value = calc_value_from_pointer(
                            event.client_x() as f64,
                            event.client_y() as f64,
                            &el,
                        );

                        let values = value.get();
                        let closest_idx = values.iter().enumerate()
                            .min_by(|(_, a), (_, b)| {
                                ((**a - new_value).abs()).partial_cmp(&((**b - new_value).abs())).unwrap()
                            })
                            .map(|(idx, _)| idx)
                            .unwrap_or(0);

                        dragging_idx.set(Some(closest_idx));

                        let mut new_values = values;
                        if let Some(v) = new_values.get_mut(closest_idx) {
                            *v = new_value;
                        }
                        set_value.run(Some(new_values));
                    }
                }
                on:pointermove=move |event: PointerEvent| {
                    if let Some(idx) = dragging_idx.get_untracked()
                        && let Some(target) = event.current_target()
                        && let Ok(el) = target.dyn_into::<web_sys::HtmlElement>()
                    {
                        let new_value = calc_value_from_pointer(
                            event.client_x() as f64,
                            event.client_y() as f64,
                            &el,
                        );

                        let mut new_values = value.get();
                        if let Some(v) = new_values.get_mut(idx) {
                            *v = new_value;
                        }
                        set_value.run(Some(new_values));
                    }
                }
                on:pointerup=move |_: PointerEvent| {
                    if dragging_idx.get_untracked().is_some()
                        && let Some(on_commit) = on_value_commit
                    {
                        on_commit.run(value.get());
                    }
                    dragging_idx.set(None);
                }
                on:pointercancel=move |_: PointerEvent| {
                    dragging_idx.set(None);
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

#[component]
pub fn SliderTrack(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<SliderContextValue>();

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:data-orientation=move || context.orientation.get()
            attr:data-disabled=move || context.disabled.get().then_some("")
            attr:style="position:relative"
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

#[component]
pub fn SliderRange(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<SliderContextValue>();

    let offset_start = Signal::derive(move || {
        let values = context.value.get();
        let min = context.min.get();
        let max = context.max.get();
        let range = max - min;
        if range == 0.0 {
            return 0.0;
        }
        let min_val = values.iter().copied().fold(f64::INFINITY, f64::min);
        ((min_val - min) / range) * 100.0
    });

    let offset_end = Signal::derive(move || {
        let values = context.value.get();
        let min = context.min.get();
        let max = context.max.get();
        let range = max - min;
        if range == 0.0 {
            return 0.0;
        }
        let max_val = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        100.0 - ((max_val - min) / range) * 100.0
    });

    let is_horizontal = Signal::derive(move || context.orientation.get() == "horizontal");
    let is_inverted = Signal::derive(move || context.inverted.get());

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:data-orientation=move || context.orientation.get()
            attr:data-disabled=move || context.disabled.get().then_some("")
            attr:style=move || {
                let (start, end) = if is_inverted.get() {
                    (offset_end.get(), offset_start.get())
                } else {
                    (offset_start.get(), offset_end.get())
                };
                if is_horizontal.get() {
                    format!("position:absolute;left:{}%;right:{}%", start, end)
                } else {
                    format!("position:absolute;bottom:{}%;top:{}%", start, end)
                }
            }
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

#[component]
pub fn SliderThumb(
    #[prop(into, optional)] index: Option<usize>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<SliderContextValue>();
    // Auto-assign index from render order if not explicitly provided.
    let thumb_index = index.unwrap_or_else(|| {
        let idx = context.thumb_count.get_untracked();
        context.thumb_count.update(|c| *c += 1);
        idx
    });

    let thumb_value = Signal::derive(move || {
        context
            .value
            .get()
            .get(thumb_index)
            .copied()
            .unwrap_or(context.min.get())
    });

    let percent = Signal::derive(move || {
        let range = context.max.get() - context.min.get();
        if range == 0.0 {
            return 0.0;
        }
        let pct = ((thumb_value.get() - context.min.get()) / range) * 100.0;
        if context.inverted.get() {
            100.0 - pct
        } else {
            pct
        }
    });

    let is_horizontal = Signal::derive(move || context.orientation.get() == "horizontal");

    let is_contain = Signal::derive(move || context.thumb_alignment.get() == "contain");

    view! {
        <span
            style=move || {
                let pos = percent.get();
                if is_contain.get() {
                    // "contain" mode: thumb stays within track bounds.
                    // Use linear interpolation: at 0% thumb left edge is at 0, at 100% right edge is at 100%.
                    if is_horizontal.get() {
                        format!("position:absolute;left:{pos}%;pointer-events:none")
                    } else {
                        format!("position:absolute;bottom:{pos}%;pointer-events:none")
                    }
                } else {
                    // "center" mode (default): thumb center is at the value position.
                    if is_horizontal.get() {
                        format!("position:absolute;left:{pos}%;transform:translateX(-50%);pointer-events:none")
                    } else {
                        format!("position:absolute;bottom:{pos}%;transform:translateY(50%);pointer-events:none")
                    }
                }
            }
        >
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attr:role="slider"
                attr:aria-valuemin=move || context.min.get().to_string()
                attr:aria-valuemax=move || context.max.get().to_string()
                attr:aria-valuenow=move || thumb_value.get().to_string()
                attr:aria-orientation=move || context.orientation.get()
                attr:data-orientation=move || context.orientation.get()
                attr:data-disabled=move || context.disabled.get().then_some("")
                attr:disabled=move || context.disabled.get().then_some("")
                attr:tabindex=move || if context.disabled.get() { None } else { Some("0") }
                attr:style="pointer-events:none;cursor:grab"
                on:keydown=move |event: KeyboardEvent| {
                    if context.disabled.get() {
                        return;
                    }
                    let step = context.step.get();
                    let big_step = step * 10.0;
                    let is_rtl = context.direction.get() == Direction::Rtl;
                    let is_inverted = context.inverted.get();

                    // When inverted, arrow semantics flip (increase becomes decrease and vice versa).
                    let invert = |v: f64| if is_inverted { -v } else { v };

                    let delta = match event.key().as_str() {
                        "ArrowRight" => Some(invert(if is_rtl { -step } else { step })),
                        "ArrowLeft" => Some(invert(if is_rtl { step } else { -step })),
                        "ArrowUp" => Some(invert(step)),
                        "ArrowDown" => Some(invert(-step)),
                        "PageUp" => Some(invert(big_step)),
                        "PageDown" => Some(invert(-big_step)),
                        "Home" => Some(context.min.get() - thumb_value.get()),
                        "End" => Some(context.max.get() - thumb_value.get()),
                        _ => None,
                    };

                    if let Some(delta) = delta {
                        event.prevent_default();
                        let new_value = snap_to_step(thumb_value.get() + delta, step, context.min.get());
                        let new_value = clamp(new_value, [context.min.get(), context.max.get()]);
                        let mut values = context.value.get();
                        if let Some(v) = values.get_mut(thumb_index) {
                            *v = new_value;
                        }
                        context.on_value_change.run(values.clone());
                        if let Some(on_commit) = context.on_value_commit {
                            on_commit.run(values);
                        }
                    }
                }
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </span>
    }
}

fn snap_to_step(value: f64, step: f64, min: f64) -> f64 {
    let steps = ((value - min) / step).round();
    min + steps * step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snap_to_step_basic() {
        assert_eq!(snap_to_step(5.0, 1.0, 0.0), 5.0);
        assert_eq!(snap_to_step(5.3, 1.0, 0.0), 5.0);
        assert_eq!(snap_to_step(5.7, 1.0, 0.0), 6.0);
    }

    #[test]
    fn snap_to_step_with_offset_min() {
        assert_eq!(snap_to_step(7.0, 5.0, 2.0), 7.0);
        assert_eq!(snap_to_step(8.0, 5.0, 2.0), 7.0);
        assert_eq!(snap_to_step(10.0, 5.0, 2.0), 12.0);
    }

    #[test]
    fn snap_to_step_fractional() {
        let result = snap_to_step(0.15, 0.1, 0.0);
        assert!((result - 0.2).abs() < f64::EPSILON || (result - 0.1).abs() < 0.01);
    }

    #[test]
    fn snap_to_step_at_boundaries() {
        assert_eq!(snap_to_step(0.0, 10.0, 0.0), 0.0);
        assert_eq!(snap_to_step(100.0, 10.0, 0.0), 100.0);
    }
}
