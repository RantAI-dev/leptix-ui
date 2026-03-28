//! Leptix UI — Radix-style accessible UI primitives for Leptos.
//!
//! This is the facade crate that re-exports all Leptix primitives.
//! Use feature flags to include only the components you need.

pub use leptix_core as core;

#[cfg(feature = "label")]
pub use leptix_label as label;

#[cfg(feature = "separator")]
pub use leptix_separator as separator;

#[cfg(feature = "accessible-icon")]
pub use leptix_accessible_icon as accessible_icon;

#[cfg(feature = "aspect-ratio")]
pub use leptix_aspect_ratio as aspect_ratio;

#[cfg(feature = "progress")]
pub use leptix_progress as progress;

#[cfg(feature = "toggle")]
pub use leptix_toggle as toggle;

#[cfg(feature = "switch")]
pub use leptix_switch as switch;

#[cfg(feature = "avatar")]
pub use leptix_avatar as avatar;

#[cfg(feature = "checkbox")]
pub use leptix_checkbox as checkbox;

#[cfg(feature = "collapsible")]
pub use leptix_collapsible as collapsible;

#[cfg(feature = "toggle-group")]
pub use leptix_toggle_group as toggle_group;

#[cfg(feature = "radio-group")]
pub use leptix_radio_group as radio_group;

#[cfg(feature = "tabs")]
pub use leptix_tabs as tabs;

#[cfg(feature = "accordion")]
pub use leptix_accordion as accordion;

#[cfg(feature = "slider")]
pub use leptix_slider as slider;

#[cfg(feature = "dialog")]
pub use leptix_dialog as dialog;

#[cfg(feature = "alert-dialog")]
pub use leptix_alert_dialog as alert_dialog;

#[cfg(feature = "popover")]
pub use leptix_popover as popover;

#[cfg(feature = "tooltip")]
pub use leptix_tooltip as tooltip;

#[cfg(feature = "hover-card")]
pub use leptix_hover_card as hover_card;

#[cfg(feature = "toolbar")]
pub use leptix_toolbar as toolbar;

#[cfg(feature = "dropdown-menu")]
pub use leptix_dropdown_menu as dropdown_menu;

#[cfg(feature = "context-menu")]
pub use leptix_context_menu as context_menu;

#[cfg(feature = "menubar")]
pub use leptix_menubar as menubar;

#[cfg(feature = "select")]
pub use leptix_select as select;

#[cfg(feature = "navigation-menu")]
pub use leptix_navigation_menu as navigation_menu;

#[cfg(feature = "toast")]
pub use leptix_toast as toast;

#[cfg(feature = "scroll-area")]
pub use leptix_scroll_area as scroll_area;

#[cfg(feature = "form")]
pub use leptix_form as form;
