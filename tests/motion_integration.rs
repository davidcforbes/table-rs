//! Integration test for the motion module.
//!
//! These run as an external test binary (separate from the unit tests in
//! `src/motion/tests.rs`), so they exercise the crate via its `pub` API
//! exactly as a downstream consumer would. They are intentionally cheap:
//! CSS string emission is pure and deterministic, so a few structural
//! sanity-checks here are enough to catch the worst regressions in the
//! generated `<style>` content.

use table_rs::motion::{animations_css, tokens_css};

/// Strip CRLF so the checks are insensitive to line-ending conversion on
/// Windows checkouts that lack `.gitattributes`.
fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n")
}

#[test]
fn css_blocks_have_balanced_braces() {
    for css in [&normalize(&tokens_css())[..], &normalize(animations_css())[..]] {
        let opens = css.matches('{').count();
        let closes = css.matches('}').count();
        assert_eq!(
            opens, closes,
            "unbalanced braces in CSS: {} open vs {} close",
            opens, closes
        );
    }
}

#[test]
fn css_blocks_have_no_html_unsafe_characters() {
    for css in [&normalize(&tokens_css())[..], &normalize(animations_css())[..]] {
        // The strings will be embedded inside a `<style>` element. A `<`,
        // `>`, or `&` would let an attacker (or accidental refactor)
        // terminate the style block early or trigger HTML entity parsing.
        assert!(!css.contains('<'));
        assert!(!css.contains('>'));
        assert!(!css.contains('&'));
    }
}

#[test]
fn animations_css_references_only_defined_token_vars() {
    let animations = normalize(animations_css());
    let tokens = normalize(&tokens_css());

    // Every `var(--trs-*)` in animations.rs must correspond to a token
    // emitted by tokens.rs. If we ever rename a token without updating
    // the animations block (or vice versa), this catches the drift.
    let mut cursor = 0;
    while let Some(start) = animations[cursor..].find("var(--") {
        let abs = cursor + start + "var(".len();
        let end = animations[abs..]
            .find(')')
            .expect("unterminated var() reference in animations CSS");
        let var_name = &animations[abs..abs + end];
        assert!(
            tokens.contains(&format!("{}:", var_name)),
            "animations.rs references {} but tokens.rs does not define it",
            var_name
        );
        cursor = abs + end;
    }
}
