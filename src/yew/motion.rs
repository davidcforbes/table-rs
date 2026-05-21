//! Yew `MotionPreamble` component.
//!
//! Mounts the framework-agnostic motion CSS once at app root. Renders two
//! `<style>` elements (tokens + animations). See [`crate::motion`] for the
//! full design.
//!
//! ## Usage
//!
//! ```ignore
//! use table_rs::yew::motion::MotionPreamble;
//! use yew::prelude::*;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <>
//!             <MotionPreamble />
//!             // ... rest of your app, including any <Table>s that pass
//!             // TableClasses::with_motion() ...
//!         </>
//!     }
//! }
//! ```
//!
//! Mounting twice in the same document is harmless but wasteful — the
//! second copy of the same CSS overrides the first identically.

use crate::motion::{animations_css, tokens_css};
use yew::prelude::*;

/// Yew component that emits the motion-system tokens and animations CSS.
/// Render once near the root of your app.
#[function_component(MotionPreamble)]
pub fn motion_preamble() -> Html {
    html! {
        <>
            <style>{ tokens_css() }</style>
            <style>{ animations_css() }</style>
        </>
    }
}
