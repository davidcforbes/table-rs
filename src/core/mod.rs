//! Framework-agnostic table logic shared by the Yew, Dioxus, and Leptos
//! backends.
//!
//! This module is always compiled (no feature gate), mirroring
//! [`crate::motion`]. The backends are thin presentation + state-wiring
//! layers over these pure functions, so the sort/filter/paginate
//! behavior has exactly one source of truth.
//!
//! The shared data model is `Vec<HashMap<&'static str, String>>` with
//! rows addressed by a `Vec<usize>` of indices. Nothing here touches
//! `web-sys` or any framework — URL sync and input debouncing stay in
//! the backends, which is where they legitimately differ.

pub mod filter;
pub mod paginate;
pub mod sort;

pub use filter::filter_indices;
pub use paginate::{Page, paginate};
pub use sort::{SortOrder, sort_indices, toggle_sort};

#[cfg(test)]
mod tests;
