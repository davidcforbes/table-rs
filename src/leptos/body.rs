//! Leptos table body — loading / empty / data rows.

use std::collections::HashMap;

use leptos::prelude::*;

use crate::leptos::types::{Column, TableClasses, TableTexts};

type Row = HashMap<&'static str, String>;

/// Renders the `<tbody>`. `rows` is the pre-filtered / pre-sorted /
/// pre-paginated page slice produced by the parent `Table`.
#[component]
pub fn TableBody(
    columns: Vec<Column>,
    #[prop(into)] rows: Signal<Vec<Row>>,
    #[prop(default = false)] loading: bool,
    classes: TableClasses,
    texts: TableTexts,
) -> impl IntoView {
    let tbody_class = classes.tbody;
    let row_class = classes.row;
    let body_cell = classes.body_cell;
    let loading_row = classes.loading_row;
    let empty_row = classes.empty_row;
    let loading_text = texts.loading;
    let empty_text = texts.empty;
    let col_count = columns.len();

    view! {
        <tbody class=tbody_class>
            {move || {
                if loading {
                    view! {
                        <tr class=loading_row>
                            <td colspan=col_count.to_string()>{loading_text}</td>
                        </tr>
                    }
                        .into_any()
                } else if rows.get().is_empty() {
                    view! {
                        <tr class=empty_row>
                            <td colspan=col_count.to_string()>{empty_text}</td>
                        </tr>
                    }
                        .into_any()
                } else {
                    rows.get()
                        .into_iter()
                        .map(|row| {
                            let cells = columns
                                .iter()
                                .map(|col| {
                                    let v = row.get(col.id).cloned().unwrap_or_default();
                                    view! {
                                        <td class=body_cell role="cell">{v}</td>
                                    }
                                })
                                .collect_view();
                            view! {
                                <tr class=row_class role="row">{cells}</tr>
                            }
                        })
                        .collect_view()
                        .into_any()
                }
            }}
        </tbody>
    }
}
