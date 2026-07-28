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
                        // The caller's style FIRST, then the widths — so a
                        // width set here wins over one in the style string, and
                        // the typed field is the one that decides.
                        let style = width_style(col.style.unwrap_or_default(), col.min_width, col.max_width);
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

/// Append `min-width`/`max-width` to a caller-supplied style string.
///
/// Emitted as inline style rather than a class because the values are per
/// column and numeric — a class per width would be a stylesheet that grows with
/// the data.
///
/// `0` means "no bound", for both: that is what lets an existing table lay out
/// exactly as it did before these fields were read at all.
fn width_style(base: &str, min_width: u32, max_width: u32) -> String {
    let mut out = base.trim().to_string();
    let mut push = |decl: String| {
        if !out.is_empty() && !out.ends_with(';') {
            out.push(';');
        }
        out.push_str(&decl);
    };
    if min_width > 0 {
        push(format!("min-width:{min_width}px;"));
    }
    if max_width > 0 {
        push(format!("max-width:{max_width}px;"));
    }
    out
}

#[cfg(test)]
mod width_tests {
    use super::width_style;

    /// Zero means no bound — an existing table must lay out exactly as it did
    /// before either field was read.
    #[test]
    fn zero_emits_nothing() {
        assert_eq!(width_style("padding:8px;", 0, 0), "padding:8px;");
        assert_eq!(width_style("", 0, 0), "");
    }

    #[test]
    fn a_maximum_lets_a_short_column_stop_growing() {
        assert_eq!(width_style("", 0, 120), "max-width:120px;");
    }

    /// Both together are the useful case: "between this and that" is what a
    /// column actually wants, rather than a fixed percentage that is wrong at
    /// every width except the one it was chosen at.
    #[test]
    fn a_minimum_and_a_maximum_bracket_the_column() {
        let s = width_style("", 80, 200);
        assert!(s.contains("min-width:80px;"), "{s}");
        assert!(s.contains("max-width:200px;"), "{s}");
    }

    /// The caller's own style survives, and is not run together with the
    /// widths into one unparseable declaration.
    #[test]
    fn a_caller_style_without_a_trailing_semicolon_is_still_separated() {
        let s = width_style("text-align:left", 0, 90);
        assert_eq!(s, "text-align:left;max-width:90px;");
    }
}
