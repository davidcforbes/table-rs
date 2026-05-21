//! Dioxus `MotionPreamble` component.
//!
//! Mounts the framework-agnostic motion CSS once at app root. Renders two
//! `<style>` elements (tokens + animations). See [`crate::motion`] for the
//! full design.
//!
//! ## Usage
//!
//! ```ignore
//! use table_rs::dioxus::motion::MotionPreamble;
//! use dioxus::prelude::*;
//!
//! fn app() -> Element {
//!     rsx! {
//!         MotionPreamble {}
//!         // ... rest of your app, including any Table that passes
//!         // TableClasses::with_motion() ...
//!     }
//! }
//! ```
//!
//! Mounting twice in the same document is harmless but wasteful — the
//! second copy of the same CSS overrides the first identically.

use crate::motion::{animations_css, tokens_css};
use dioxus::prelude::*;

/// Dioxus component that emits the motion-system tokens and animations
/// CSS. Render once near the root of your app.
#[component]
pub fn MotionPreamble() -> Element {
    rsx! {
        style { "{tokens_css()}" }
        style { "{animations_css()}" }
    }
}
