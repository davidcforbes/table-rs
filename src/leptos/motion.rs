//! Leptos `MotionPreamble` component.
//!
//! Mounts the framework-agnostic motion CSS once at app root. Renders two
//! `<style>` elements (tokens + animations). See [`crate::motion`] for
//! the full design.
//!
//! ## Usage
//!
//! ```ignore
//! use leptos::prelude::*;
//! use table_rs::leptos::motion::MotionPreamble;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <MotionPreamble/>
//!         // ... rest of your app ...
//!     }
//! }
//! ```
//!
//! Mounting twice in the same document is harmless but wasteful — the
//! second copy of the same CSS overrides the first identically.

use crate::motion::{animations_css, tokens_css};
use leptos::prelude::*;

/// Leptos component that emits the motion-system tokens and animations
/// CSS. Render once near the root of your app.
#[component]
pub fn MotionPreamble() -> impl IntoView {
    view! {
        <style>{tokens_css()}</style>
        <style>{animations_css()}</style>
    }
}
