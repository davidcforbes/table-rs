//! CSS class-name constants used by the motion system.
//!
//! All classes share the `trs-` prefix (short for **t**able-**rs**) to avoid
//! collisions with daisyUI's `.table`, Tailwind utilities, and the
//! sibling library leptos-daisyui-rs which uses `ld-*`.

/// The shared prefix for every motion-system CSS class.
pub const PREFIX: &str = "trs";

/// Smooth transitions on opacity/color/transform/shadow.
pub const CLASS_EASED: &str = "trs-eased";

/// Subtle `scale(0.97)` on `:active`. Pair with [`CLASS_EASED`] so the
/// scale-down/up itself eases.
pub const CLASS_PRESSABLE: &str = "trs-pressable";

/// Resting LEVEL_4 shadow that lifts to LEVEL_8 + a 1px translate on hover.
pub const CLASS_ELEVATED: &str = "trs-elevated";

/// Animated keyboard focus ring (only on `:focus-visible`).
pub const CLASS_FOCUS_RING: &str = "trs-focus-ring";

/// Click-radiating ripple host. The bookkeeping (per-instance state) lives
/// per-framework under follow-up bead `table-rs-611`; the CSS for this
/// class is in place today.
pub const CLASS_RIPPLE_HOST: &str = "trs-ripple-host";

/// Click-radiating ripple element. See [`CLASS_RIPPLE_HOST`].
pub const CLASS_RIPPLE_ELEMENT: &str = "trs-ripple-element";
