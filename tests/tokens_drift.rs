//! Drift detection for the motion constants inlined in
//! `src/motion/tokens.rs`.
//!
//! table-rs inlines the Fluent 2 motion/elevation values rather than
//! depending on the canonical `ui-tokens` crate, because `ui-tokens` is
//! not published to crates.io and a path-dependency (even an optional
//! one) breaks `cargo publish`. This test instead reads the `ui-tokens`
//! source **from disk** and asserts the inlined values still match —
//! with zero impact on the published manifest.
//!
//! It is **gated on the `UI_TOKENS_SRC` environment variable** (the path
//! to the `ui-tokens` crate's `src/` directory). When unset — the normal
//! case for crates.io consumers running `cargo test` — the test prints a
//! skip notice and passes. CI sets the variable after checking out the
//! sibling repo:
//!
//! ```sh
//! UI_TOKENS_SRC=../Rust-DeskApp/crates/ui-tokens/src \
//!   cargo test --test tokens_drift
//! ```

use table_rs::motion::tokens::{
    DURATION_FAST_MS, DURATION_NORMAL_MS, DURATION_SLOW_MS, EASINGS, ELEVATIONS,
};

const EPS: f32 = 1e-6;

/// Extract `pub const <name>: <ty> = <value>;` and parse the value.
fn extract_u32(src: &str, name: &str) -> u32 {
    let line = src
        .lines()
        .find(|l| l.contains(&format!("const {name}")))
        .unwrap_or_else(|| panic!("ui-tokens: const {name} not found"));
    let rhs = line.split('=').nth(1).expect("no `=` in const line");
    rhs.trim()
        .trim_end_matches(';')
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("ui-tokens: could not parse {name}"))
}

/// Read the first 4 comma-separated floats inside the parentheses that
/// follow `marker` in `src`.
fn extract_tuple4(src: &str, marker: &str) -> (f32, f32, f32, f32) {
    let start = src
        .find(marker)
        .unwrap_or_else(|| panic!("ui-tokens: marker {marker:?} not found"));
    let after = &src[start + marker.len()..];
    let open = after.find('(').expect("no `(` after marker");
    let close = after[open..].find(')').expect("no `)` after marker") + open;
    let inner = &after[open + 1..close];
    let nums: Vec<f32> = inner
        .split(',')
        .map(|s| s.trim().parse().expect("non-float in tuple"))
        .collect();
    assert!(nums.len() >= 4, "expected 4 floats after {marker:?}");
    (nums[0], nums[1], nums[2], nums[3])
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
}

#[test]
fn inline_tokens_match_ui_tokens_crate() {
    let Ok(dir) = std::env::var("UI_TOKENS_SRC") else {
        eprintln!(
            "skipping tokens drift check: set UI_TOKENS_SRC to the ui-tokens \
             crate's src/ dir to enable (e.g. \
             ../Rust-DeskApp/crates/ui-tokens/src)"
        );
        return;
    };

    let motion =
        std::fs::read_to_string(format!("{dir}/motion.rs")).expect("read ui-tokens motion.rs");
    let elevation = std::fs::read_to_string(format!("{dir}/elevation.rs"))
        .expect("read ui-tokens elevation.rs");

    // Durations.
    assert_eq!(DURATION_FAST_MS, extract_u32(&motion, "DURATION_FAST_MS"));
    assert_eq!(
        DURATION_NORMAL_MS,
        extract_u32(&motion, "DURATION_NORMAL_MS")
    );
    assert_eq!(DURATION_SLOW_MS, extract_u32(&motion, "DURATION_SLOW_MS"));

    // Easing bezier control points — parsed from the `bezier()` match arms.
    for (name, x1, y1, x2, y2) in EASINGS {
        let variant = match *name {
            "linear" => "Easing::Linear =>",
            "standard" => "Easing::Standard =>",
            "decelerate" => "Easing::Decelerate =>",
            "accelerate" => "Easing::Accelerate =>",
            other => panic!("unexpected easing name {other:?}"),
        };
        let (cx1, cy1, cx2, cy2) = extract_tuple4(&motion, variant);
        assert!(
            close(*x1, cx1) && close(*y1, cy1) && close(*x2, cx2) && close(*y2, cy2),
            "easing {name} drifted: inline ({x1},{y1},{x2},{y2}) vs canonical ({cx1},{cy1},{cx2},{cy2})"
        );
    }

    // Elevation shadows — parsed from `LEVEL_N: Shadow = Shadow::new(...)`.
    for (name, ox, oy, blur, opacity) in ELEVATIONS {
        let marker = format!("LEVEL_{name}: Shadow = Shadow::new");
        let (cox, coy, cblur, copacity) = extract_tuple4(&elevation, &marker);
        assert!(
            close(*ox, cox) && close(*oy, coy) && close(*blur, cblur) && close(*opacity, copacity),
            "elevation {name} drifted: inline ({ox},{oy},{blur},{opacity}) vs canonical ({cox},{coy},{cblur},{copacity})"
        );
    }
}
