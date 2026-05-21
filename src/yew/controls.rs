use crate::yew::ripple::use_ripple;
use crate::yew::types::PaginationControlsProps;
use web_sys::MouseEvent;
use yew::prelude::*;

#[function_component(PaginationControls)]
pub fn pagination_controls(props: &PaginationControlsProps) -> Html {
    let PaginationControlsProps {
        page,
        total_pages,
        classes,
        texts,
    } = props;
    let page_val = **page;

    // Ripple is active only when the button is a `trs-ripple-host`
    // (motion opted in via `TableClasses::with_motion`). Gating on the
    // class keeps default tables ripple-free and avoids accumulating
    // instances that would never animate/dismiss.
    let ripple_on = classes.pagination_button.contains("trs-ripple-host");
    let prev_ripple = use_ripple();
    let next_ripple = use_ripple();

    let on_prev = {
        let page = page.clone();
        let ripple = prev_ripple.clone();
        Callback::from(move |e: MouseEvent| {
            if ripple_on {
                ripple.trigger(&e);
            }
            if *page > 0 {
                page.set(*page - 1);
            }
        })
    };

    let on_next = {
        let page = page.clone();
        let total_pages = *total_pages;
        let ripple = next_ripple.clone();
        Callback::from(move |e: MouseEvent| {
            if ripple_on {
                ripple.trigger(&e);
            }
            // Only increment if we're not on the last page
            if *page + 1 < total_pages {
                page.set(*page + 1);
            }
        })
    };

    // Pre-compute page indicator to avoid multiple string allocations
    let page_indicator_text = texts
        .page_indicator
        .replace("{current}", &(page_val + 1).to_string())
        .replace("{total}", &total_pages.to_string());

    html! {
        <div class={classes.pagination}>
            <button class={classes.pagination_button} onclick={on_prev} disabled={page_val == 0}>
                { texts.previous_button }
                { prev_ripple.overlay() }
            </button>
            <span>
                { page_indicator_text }
            </span>
            <button
                class={classes.pagination_button}
                onclick={on_next}
                disabled={page_val + 1 >= *total_pages}
            >
                { texts.next_button }
                { next_ripple.overlay() }
            </button>
        </div>
    }
}
