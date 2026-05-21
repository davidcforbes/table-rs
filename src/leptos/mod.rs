//! Leptos backend for table-rs.
//!
//! Currently ships only the [`motion::MotionPreamble`] component — a
//! Leptos `Table` scaffold is tracked under follow-up bead
//! `table-rs-612`. Consumers hand-rolling a Leptos table on top of this
//! crate's CSS classes can use the preamble today.

pub mod motion;
