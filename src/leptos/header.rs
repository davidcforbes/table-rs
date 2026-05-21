//! Leptos table header — sortable column headers with the `trs-sort-arrow`
//! indicator.

use leptos::prelude::*;

use crate::leptos::ripple::{RippleOverlay, use_ripple};
use crate::leptos::types::{Column, SortOrder, TableClasses};

/// Renders the `<thead>` row. Clicking a sortable header invokes
/// `on_sort` with the column id; the parent owns the sort state.
#[component]
pub fn TableHeader(
    columns: Vec<Column>,
    #[prop(into)] sort_column: Signal<Option<&'static str>>,
    #[prop(into)] sort_order: Signal<SortOrder>,
    on_sort: Callback<&'static str>,
    classes: TableClasses,
) -> impl IntoView {
    let thead_class = classes.thead;
    let row_class = classes.row;
    let header_cell = classes.header_cell;

    view! {
        <thead class=thead_class>
            <tr class=row_class role="row">
                {columns
                    .into_iter()
                    .map(move |col| {
                        let col_id = col.id;
                        let sortable = col.sortable;
                        let header = col.header;
                        let style = col.style.unwrap_or_default();
                        let cell_class =
                            format!("{} {}", header_cell, col.class.unwrap_or("")).trim().to_string();

                        // Ripple is active only on sortable headers that are
                        // `trs-ripple-host` (i.e. motion opted in via
                        // `with_motion`). See the controls module for rationale.
                        let ripple_on = sortable && cell_class.contains("trs-ripple-host");
                        let ripple = use_ripple();

                        let aria_sort = move || {
                            if sort_column.get() == Some(col_id) {
                                match sort_order.get() {
                                    SortOrder::Asc => "ascending",
                                    SortOrder::Desc => "descending",
                                }
                            } else {
                                "none"
                            }
                        };
                        let arrow_direction = move || {
                            if !sortable {
                                ""
                            } else if sort_column.get() == Some(col_id) {
                                match sort_order.get() {
                                    SortOrder::Asc => "asc",
                                    SortOrder::Desc => "desc",
                                }
                            } else {
                                "none"
                            }
                        };

                        view! {
                            <th
                                class=cell_class
                                role="columnheader"
                                style=style
                                aria-sort=aria_sort
                                on:click=move |ev| {
                                    if ripple_on {
                                        ripple.trigger.run(ev);
                                    }
                                    if sortable {
                                        on_sort.run(col_id);
                                    }
                                }
                            >
                                {header}
                                {move || {
                                    sortable.then(|| {
                                        view! {
                                            <span
                                                class="trs-sort-arrow"
                                                data-direction=arrow_direction
                                                aria-hidden="true"
                                            >
                                                "▲"
                                            </span>
                                        }
                                    })
                                }}
                                <RippleOverlay handle=ripple />
                            </th>
                        }
                    })
                    .collect_view()}
            </tr>
        </thead>
    }
}
