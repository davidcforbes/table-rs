#![doc(
    html_logo_url = "https://raw.githubusercontent.com/opensass/image-rs/refs/heads/main/assets/logo.webp",
    html_favicon_url = "https://github.com/opensass/image-rs/blob/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

/// Framework-agnostic table logic (sort / filter / paginate).
///
/// Always compiled; the Yew, Dioxus, and Leptos backends are thin
/// wrappers over these pure functions.
pub mod core;

/// Framework-agnostic motion + elevation primitives.
///
/// Always compiled; pair with the framework-specific `MotionPreamble`
/// component in [`yew::motion`], [`dioxus::motion`], or `leptos::motion`
/// to wire it into your app.
pub mod motion;

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "dio")]
pub mod dioxus;

#[cfg(feature = "lep")]
pub mod leptos;
