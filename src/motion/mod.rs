//! Framework-agnostic motion + elevation primitives for table-rs.
//!
//! This module is always compiled (no feature gate). It exposes two
//! string-emission functions ([`tokens_css`] and [`animations_css`]) plus
//! a set of CSS class-name constants. Each framework backend
//! (`crate::yew`, `crate::dioxus`, `crate::leptos`) ships a tiny
//! `MotionPreamble` wrapper component that mounts the two strings as
//! `<style>` elements in its idiomatic way.
//!
//! ## Design
//!
//! The values flow:
//!
//! ```text
//!   ui-tokens canonical consts (Rust-DeskApp/crates/ui-tokens)
//!       │ (inlined; pinned by `tokens-pin` dev-feature)
//!       ▼
//!   crate::motion::tokens — Rust consts + tokens_css() String
//!       │
//!       ▼
//!   <style>:root { --trs-* }</style> (mounted by MotionPreamble)
//!       │
//!       ▼
//!   crate::motion::animations — animations_css() &'static str
//!       │
//!       ▼
//!   <style>.trs-eased { transition: ... var(--trs-*) ... }</style>
//!       │
//!       ▼
//!   Consumed by elements that include `trs-eased`, `trs-pressable`, etc.
//!   in their class attribute (typically via `TableClasses::with_motion()`).
//! ```
//!
//! ## Quick start
//!
//! 1. Mount the framework's `MotionPreamble` once at app root.
//! 2. Pass `TableClasses::with_motion()` to your `Table` props.
//!
//! See the `examples/yew` and `examples/dioxus` projects for working setups.

pub mod animations;
pub mod prefix;
pub mod tokens;

pub use animations::animations_css;
pub use prefix::*;
pub use tokens::{
    tokens_css, DURATION_FAST_MS, DURATION_NORMAL_MS, DURATION_SLOW_MS, EASINGS, ELEVATIONS,
};

#[cfg(test)]
mod tests;
