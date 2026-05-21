use crate::dioxus::ripple::use_ripple;
use crate::dioxus::types::TableClasses;
use crate::dioxus::types::TableTexts;
use dioxus::prelude::*;

#[component]
pub fn PaginationControls(
    mut page: Signal<usize>,
    total_pages: usize,
    classes: TableClasses,
    texts: TableTexts,
) -> Element {
    // Ripple is active only when the button is a `trs-ripple-host`
    // (motion opted in via `TableClasses::with_motion`). Gating on the
    // class keeps default tables ripple-free and avoids accumulating
    // instances that would never animate/dismiss.
    let ripple_on = classes.pagination_button.contains("trs-ripple-host");
    let prev_ripple = use_ripple();
    let next_ripple = use_ripple();

    let on_prev = move |e: Event<MouseData>| {
        if ripple_on {
            prev_ripple.trigger(&e);
        }
        if page() > 0 {
            page.set(page() - 1);
        }
    };

    let on_next = move |e: Event<MouseData>| {
        if ripple_on {
            next_ripple.trigger(&e);
        }
        if page() + 1 < total_pages {
            page.set(page() + 1);
        }
    };

    // Pre-compute page indicator to avoid multiple string allocations
    let page_indicator_text = texts
        .page_indicator
        .replace("{current}", &(page() + 1).to_string())
        .replace("{total}", &total_pages.to_string());

    rsx! {
        div { class: classes.pagination,
            button {
                class: classes.pagination_button,
                onclick: on_prev,
                disabled: page() == 0,
                "{texts.previous_button}"
                {prev_ripple.overlay()}
            }
            span {
                "{ page_indicator_text }"
            }
            button {
                class: classes.pagination_button,
                onclick: on_next,
                disabled: page() + 1 >= total_pages,
                "{texts.next_button}"
                {next_ripple.overlay()}
            }
        }
    }
}
