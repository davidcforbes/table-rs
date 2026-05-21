//! Leptos pagination controls — previous / next + page indicator.

use leptos::prelude::*;

use crate::leptos::types::{TableClasses, TableTexts};

/// Prev/next buttons around a page indicator. `page` is owned by the
/// parent `Table`; this component reads and advances it.
#[component]
pub fn PaginationControls(
    page: RwSignal<usize>,
    #[prop(into)] total_pages: Signal<usize>,
    classes: TableClasses,
    texts: TableTexts,
) -> impl IntoView {
    let pagination_class = classes.pagination;
    let button_class = classes.pagination_button;
    let prev_label = texts.previous_button;
    let next_label = texts.next_button;
    let indicator = texts.page_indicator;

    view! {
        <div class=pagination_class>
            <button
                class=button_class
                on:click=move |_| {
                    let p = page.get();
                    if p > 0 {
                        page.set(p - 1);
                    }
                }
                disabled=move || page.get() == 0
            >
                {prev_label}
            </button>
            <span>
                {move || {
                    indicator
                        .replace("{current}", &(page.get() + 1).to_string())
                        .replace("{total}", &total_pages.get().to_string())
                }}
            </span>
            <button
                class=button_class
                on:click=move |_| {
                    let p = page.get();
                    if p + 1 < total_pages.get() {
                        page.set(p + 1);
                    }
                }
                disabled=move || page.get() + 1 >= total_pages.get()
            >
                {next_label}
            </button>
        </div>
    }
}
