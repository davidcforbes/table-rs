//! Motion + elevation design-token constants and their CSS-variable emission.
//!
//! These values are intentionally **inlined** rather than depending on the
//! upstream `ui-tokens` crate. table-rs publishes to crates.io; a path-dep
//! poisons `cargo publish`, and a git-dep locks downstream consumers to a
//! single commit hash. Inlining the six numeric duration constants plus
//! the four Bezier easing tuples plus the five elevation tier tuples is a
//! one-time copy — the optional [`tokens-pin`] dev-feature (see
//! `Cargo.toml`) pulls the canonical crate in for a single drift-detecting
//! test under `#[cfg(feature = "tokens-pin")]`, so values can't silently
//! drift past CI.
//!
//! Canonical source of truth: `Rust-DeskApp/crates/ui-tokens/src/motion.rs`
//! and `…/elevation.rs`.

// -- Duration constants (milliseconds) ---------------------------------------

/// Fast state-change duration (color / opacity transitions on hover etc.).
pub const DURATION_FAST_MS: u32 = 83;
/// General-purpose transition duration (elevation lift, focus ring, ripple).
pub const DURATION_NORMAL_MS: u32 = 200;
/// Slow transition for emphasized entrances (loading / empty state fade-in).
pub const DURATION_SLOW_MS: u32 = 300;

// -- Easing curves (cubic-Bezier control points) -----------------------------

/// Named easing curves and their `cubic-bezier(x1, y1, x2, y2)` control
/// points. Values mirror the Fluent 2 motion ramp.
pub const EASINGS: &[(&str, f32, f32, f32, f32)] = &[
    // Strict linear interpolation.
    ("linear", 0.0, 0.0, 1.0, 1.0),
    // Gentle ease-in-out, the general-purpose default for state transitions.
    ("standard", 0.33, 0.0, 0.67, 1.0),
    // Content / element entering the viewport.
    ("decelerate", 0.1, 0.9, 0.2, 1.0),
    // Content leaving the viewport.
    ("accelerate", 0.7, 0.0, 1.0, 0.5),
];

// -- Elevation tiers (Fluent 2 drop-shadow ramp) -----------------------------
//
// Tuple shape: (level_name, offset_x_px, offset_y_px, blur_px, opacity).
// All Fluent 2 shadows use no horizontal offset; alpha is applied to pure
// black so callers don't need a colour token.

/// All five elevation tiers in ascending order.
pub const ELEVATIONS: &[(&str, f32, f32, f32, f32)] = &[
    ("2", 0.0, 1.0, 2.0, 0.10),  // Subtle outline used on resting controls.
    ("4", 0.0, 2.0, 4.0, 0.14),  // Card resting elevation.
    ("8", 0.0, 4.0, 8.0, 0.16),  // Hovered card / popover.
    ("16", 0.0, 8.0, 16.0, 0.18), // Dialog / modal.
    ("64", 0.0, 32.0, 64.0, 0.22), // Top-most overlay.
];

// -- CSS emission ------------------------------------------------------------

/// Build the CSS custom-property block exposing every motion + elevation
/// design token as a `--trs-*` variable on `:root`.
///
/// Mount this once at the root of an app via the framework-specific
/// `MotionPreamble` component (`crate::yew::motion::MotionPreamble`,
/// `crate::dioxus::motion::MotionPreamble`, or
/// `crate::leptos::motion::MotionPreamble`). Any rule downstream can then
/// reference `var(--trs-duration-fast)`, `var(--trs-ease-standard)`,
/// `var(--trs-elevation-4)`, etc.
pub fn tokens_css() -> String {
    let mut css = String::with_capacity(1024);
    css.push_str(":root {\n");

    css.push_str(&format!(
        "  --trs-duration-fast: {}ms;\n",
        DURATION_FAST_MS
    ));
    css.push_str(&format!(
        "  --trs-duration-normal: {}ms;\n",
        DURATION_NORMAL_MS
    ));
    css.push_str(&format!(
        "  --trs-duration-slow: {}ms;\n",
        DURATION_SLOW_MS
    ));

    for (name, x1, y1, x2, y2) in EASINGS {
        css.push_str(&format!(
            "  --trs-ease-{}: cubic-bezier({}, {}, {}, {});\n",
            name, x1, y1, x2, y2
        ));
    }

    for (name, ox, oy, blur, opacity) in ELEVATIONS {
        css.push_str(&format!(
            "  --trs-elevation-{}: {}px {}px {}px rgba(0, 0, 0, {:.2});\n",
            name, ox, oy, blur, opacity
        ));
    }

    css.push_str("}\n");
    css
}
