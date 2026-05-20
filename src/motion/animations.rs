//! Shared animation primitives — class definitions and named keyframes that
//! every framework-specific table backend consumes.
//!
//! Every duration / easing reference is a `var(--trs-*)` lookup, sourced
//! from [`super::tokens::tokens_css`]. Mount the framework-specific
//! `MotionPreamble` (which emits both the tokens block and this animations
//! block) once at app root to make the classes available globally.
//!
//! ## Available classes (Slice 1)
//!
//! - `trs-eased` — smooth transitions on opacity, color, transform, shadow.
//!   Add to any element that should ease its state changes.
//! - `trs-pressable` — adds a subtle `scale(0.97)` on `:active`. Used by
//!   pagination buttons.
//! - `trs-elevated` — resting LEVEL_4 shadow that lifts to LEVEL_8 + a 1px
//!   translate on hover. Used by Cards, but the CSS is shared.
//! - `trs-focus-ring` — animated keyboard focus ring (only on
//!   `:focus-visible`). Used by pagination buttons + search input.
//! - `trs-ripple-host` / `trs-ripple-element` / `@keyframes trs-ripple`
//!   — visual primitives for click-radiating ripples. Per-framework Rust
//!   bookkeeping ships under follow-up bead `table-rs-611`; the CSS is
//!   here now so the follow-up is a thin Rust addition.
//!
//! All motion is suppressed under `@media (prefers-reduced-motion: reduce)`.

/// Static CSS for state-transition primitives shared across table-rs
/// backends. See module docs for the available classes.
pub fn animations_css() -> &'static str {
    r#"
.trs-eased {
    transition:
        background-color var(--trs-duration-fast) var(--trs-ease-standard),
        color var(--trs-duration-fast) var(--trs-ease-standard),
        border-color var(--trs-duration-fast) var(--trs-ease-standard),
        opacity var(--trs-duration-fast) var(--trs-ease-standard),
        transform var(--trs-duration-fast) var(--trs-ease-standard),
        box-shadow var(--trs-duration-normal) var(--trs-ease-standard);
}

.trs-pressable:active:not(:disabled):not([aria-disabled='true']) {
    transform: scale(0.97);
}

.trs-elevated {
    box-shadow: var(--trs-elevation-4);
    transition:
        box-shadow var(--trs-duration-normal) var(--trs-ease-standard),
        transform var(--trs-duration-normal) var(--trs-ease-standard);
}

.trs-elevated:hover {
    box-shadow: var(--trs-elevation-8);
    transform: translateY(-1px);
}

.trs-ripple-host {
    position: relative;
    overflow: hidden;
}
.trs-ripple-element {
    position: absolute;
    width: 0;
    height: 0;
    border-radius: 50%;
    background-color: currentColor;
    opacity: 0.25;
    pointer-events: none;
    transform: translate(-50%, -50%);
    animation: trs-ripple var(--trs-duration-normal) var(--trs-ease-standard) forwards;
}
@keyframes trs-ripple {
    to {
        width: 400px;
        height: 400px;
        opacity: 0;
    }
}

.trs-focus-ring:focus-visible {
    outline: 2px solid currentColor;
    outline-offset: 2px;
    animation: trs-focus-ring-in var(--trs-duration-fast) var(--trs-ease-decelerate);
}
@keyframes trs-focus-ring-in {
    from { outline-color: transparent; }
    to   { outline-color: currentColor; }
}

@media (prefers-reduced-motion: reduce) {
    .trs-eased,
    .trs-elevated {
        transition: none;
    }
    .trs-pressable:active:not(:disabled):not([aria-disabled='true']),
    .trs-elevated:hover {
        transform: none;
    }
    .trs-ripple-element,
    .trs-focus-ring:focus-visible {
        animation: none;
    }
}
"#
}
