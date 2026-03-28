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
