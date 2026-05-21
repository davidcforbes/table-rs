//! Leptos backend for table-rs.
//!
//! Ships the [`motion::MotionPreamble`] component plus a full
//! [`table::Table`] with search, sorting, and pagination — built on the
//! framework-agnostic [`crate::core`], mirroring the Yew and Dioxus
//! backends.

pub mod body;
pub mod controls;
pub mod header;
pub mod motion;
pub mod ripple;
pub mod table;
pub mod types;
