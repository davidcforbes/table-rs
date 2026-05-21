//! String-assertion tests for the motion module.
//!
//! These run under `cargo test --no-default-features` — no framework
//! feature flag is required because the module is framework-agnostic.

use super::*;

// -- Inline value pins --------------------------------------------------------

#[test]
fn duration_constants_pin_canonical_values() {
    assert_eq!(DURATION_FAST_MS, 83);
    assert_eq!(DURATION_NORMAL_MS, 200);
    assert_eq!(DURATION_SLOW_MS, 300);
}

#[test]
fn durations_are_strictly_ascending() {
    assert!(DURATION_FAST_MS < DURATION_NORMAL_MS);
    assert!(DURATION_NORMAL_MS < DURATION_SLOW_MS);
}

#[test]
fn easings_named_in_canonical_order() {
    let names: Vec<&str> = EASINGS.iter().map(|(n, ..)| *n).collect();
    assert_eq!(
        names,
        vec!["linear", "standard", "decelerate", "accelerate"]
    );
}

#[test]
fn elevations_named_in_canonical_order() {
    let names: Vec<&str> = ELEVATIONS.iter().map(|(n, ..)| *n).collect();
    assert_eq!(names, vec!["2", "4", "8", "16", "64"]);
}

#[test]
fn elevation_blur_grows_monotonically() {
    for window in ELEVATIONS.windows(2) {
        let (.., b1, _) = window[0];
        let (.., b2, _) = window[1];
        assert!(b1 < b2, "blur not increasing: {} -> {}", b1, b2);
    }
}

// -- tokens_css emission ------------------------------------------------------

#[test]
fn tokens_css_starts_with_root_selector() {
    let css = tokens_css();
    assert!(css.trim_start().starts_with(":root {"));
    assert!(css.trim_end().ends_with("}"));
}

#[test]
fn tokens_css_emits_all_three_duration_variables() {
    let css = tokens_css();
    assert!(css.contains("--trs-duration-fast: 83ms;"));
    assert!(css.contains("--trs-duration-normal: 200ms;"));
    assert!(css.contains("--trs-duration-slow: 300ms;"));
}

#[test]
fn tokens_css_emits_all_four_named_easings() {
    let css = tokens_css();
    assert!(css.contains("--trs-ease-linear:"));
    assert!(css.contains("--trs-ease-standard:"));
    assert!(css.contains("--trs-ease-decelerate:"));
    assert!(css.contains("--trs-ease-accelerate:"));
    // Standard curve's control points must be the canonical Fluent 2 values.
    assert!(css.contains("cubic-bezier(0.33, 0, 0.67, 1)"));
}

#[test]
fn tokens_css_emits_every_elevation_tier() {
    let css = tokens_css();
    for level in ["2", "4", "8", "16", "64"] {
        assert!(
            css.contains(&format!("--trs-elevation-{}:", level)),
            "missing --trs-elevation-{}",
            level
        );
    }
}

#[test]
fn tokens_css_elevation_4_matches_canonical_shadow() {
    let css = tokens_css();
    // LEVEL_4 = (0, 2, 4, 0.14) → "0px 2px 4px rgba(0, 0, 0, 0.14)"
    assert!(css.contains("--trs-elevation-4: 0px 2px 4px rgba(0, 0, 0, 0.14);"));
}

#[test]
fn tokens_css_has_no_html_unsafe_characters() {
    let css = tokens_css();
    // The string is embedded directly in a `<style>` block; closing tags
    // would terminate it prematurely.
    assert!(!css.contains('<'));
    assert!(!css.contains('>'));
    assert!(!css.contains('&'));
}

// -- animations_css emission --------------------------------------------------

#[test]
fn animations_css_defines_state_transition_classes() {
    let css = animations_css();
    assert!(css.contains(".trs-eased"));
    assert!(css.contains(".trs-pressable:active"));
    assert!(css.contains(".trs-elevated"));
    assert!(css.contains(".trs-focus-ring:focus-visible"));
}

#[test]
fn animations_css_respects_reduced_motion() {
    let css = animations_css();
    assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
    // Within the guard, transitions and animations must be neutralised.
    assert!(css.contains("transition: none;"));
    assert!(css.contains("animation: none;"));
}

#[test]
fn animations_css_has_no_html_unsafe_characters() {
    let css = animations_css();
    assert!(!css.contains('<'));
    assert!(!css.contains('>'));
    assert!(!css.contains('&'));
}

#[test]
fn animations_css_references_token_variables() {
    let css = animations_css();
    // Sanity-check that the animation block actually consumes the tokens
    // produced by `tokens_css`.
    assert!(css.contains("var(--trs-duration-fast)"));
    assert!(css.contains("var(--trs-duration-normal)"));
    assert!(css.contains("var(--trs-ease-standard)"));
    assert!(css.contains("var(--trs-elevation-4)"));
    assert!(css.contains("var(--trs-elevation-8)"));
}

#[test]
fn animations_css_defines_sort_arrow_with_directional_states() {
    let css = animations_css();
    assert!(css.contains(".trs-sort-arrow"));
    // The two directional states must rotate to opposite orientations.
    assert!(css.contains(".trs-sort-arrow[data-direction='asc']"));
    assert!(css.contains(".trs-sort-arrow[data-direction='desc']"));
    assert!(css.contains("rotate(180deg)"));
}

#[test]
fn animations_css_defines_fade_in_keyframes_and_class() {
    let css = animations_css();
    assert!(css.contains("@keyframes trs-fade-in"));
    assert!(css.contains(".trs-fade-in"));
    // Fade duration is the slow tier, per Slice 2 spec.
    assert!(css.contains("animation: trs-fade-in var(--trs-duration-slow)"));
}

// Cross-crate drift detection (asserting these inline consts match the
// canonical `ui-tokens` crate) lives in `tests/tokens_drift.rs`, gated on
// the `UI_TOKENS_SRC` env var so it reads the sibling crate from disk
// without a Cargo dependency (which would break `cargo publish`, since
// `ui-tokens` is not on crates.io). The duration-pin tests above
// (`duration_constants_pin_canonical_values`) additionally catch the most
// likely drift target on every `cargo test`.
