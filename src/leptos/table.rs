//! Leptos `Table` — orchestrates search + sort + pagination over the
//! framework-agnostic [`crate::core`], mirroring the Yew/Dioxus backends.

use std::collections::HashMap;

use leptos::prelude::*;
use web_sys::wasm_bindgen::JsValue;

use crate::core;
use crate::leptos::body::TableBody;
use crate::leptos::controls::PaginationControls;
use crate::leptos::header::TableHeader;
use crate::leptos::types::{Column, SortOrder, TableClasses, TableTexts};

type Row = HashMap<&'static str, String>;

/// Read the initial `?search=` value from the URL (empty outside a browser).
fn initial_search() -> String {
    web_sys::window()
        .and_then(|w| w.location().search().ok())
        .and_then(|s| web_sys::UrlSearchParams::new_with_str(&s).ok())
        .and_then(|p| p.get("search"))
        .unwrap_or_default()
}

/// Reflect the current query into the URL's `?search=` param without
/// navigating. No-op outside a browser. Matches the Dioxus backend's
/// immediate (non-debounced) update.
fn update_search_url(query: &str) {
    let _ = web_sys::window().and_then(|window| {
        let url = window.location().href().ok()?;
        let url_obj = web_sys::Url::new(&url).ok()?;
        let params = url_obj.search_params();
        params.set("search", query);
        url_obj.set_search(&params.to_string().as_string().unwrap_or_default());
        window
            .history()
            .ok()?
            .replace_state_with_url(&JsValue::NULL, "", Some(&url_obj.href()))
            .ok()
    });
}

/// A data table with optional search, sorting, and pagination.
///
/// The business logic (filter / sort / paginate) is the shared
/// [`crate::core`]; this component only wires it to Leptos signals and
/// renders. Pass [`TableClasses::with_motion`] (and mount
/// [`crate::leptos::motion::MotionPreamble`]) to opt into the motion
/// system.
#[component]
pub fn Table(
    /// Row data — each row is a `HashMap` keyed by column id.
    data: Vec<Row>,
    /// Column definitions.
    columns: Vec<Column>,
    /// Rows per page (default 10).
    #[prop(default = 10)]
    page_size: usize,
    /// Whether the table is in a loading state.
    #[prop(default = false)]
    loading: bool,
    /// Class names for each part of the table.
    #[prop(optional)]
    classes: TableClasses,
    /// Show pagination controls.
    #[prop(default = false)]
    paginate: bool,
    /// Show the search input (filters client-side, syncs to `?search=`).
    #[prop(default = false)]
    search: bool,
    /// UI text strings.
    #[prop(optional)]
    texts: TableTexts,
) -> impl IntoView {
    let page = RwSignal::new(0usize);
    let sort_column = RwSignal::new(None::<&'static str>);
    let sort_order = RwSignal::new(SortOrder::default());
    let search_query = RwSignal::new(initial_search());

    // Reset to the first page whenever the query changes.
    Effect::new(move |_| {
        let _ = search_query.get();
        page.set(0);
    });

    let col_ids: Vec<&'static str> = columns.iter().map(|c| c.id).collect();

    // filter → sort → paginate, recomputed reactively via the core.
    let data_for_rows = data.clone();
    let rows_memo = Memo::new(move |_| {
        let query = search_query.get();
        let mut idx = core::filter_indices(&data_for_rows, &col_ids, &query);
        if let Some(col_id) = sort_column.get() {
            core::sort_indices(&mut idx, &data_for_rows, col_id, sort_order.get());
        }
        let window = core::paginate(idx.len(), page_size, page.get());
        let rows: Vec<Row> = idx[window.start..window.end]
            .iter()
            .map(|&i| data_for_rows[i].clone())
            .collect();
        (rows, window.total_pages)
    });
    let rows = Signal::derive(move || rows_memo.get().0);
    let total_pages = Signal::derive(move || rows_memo.get().1);

    let on_sort = Callback::new(move |id: &'static str| {
        let (next_col, next_order) =
            core::toggle_sort(sort_column.get_untracked(), sort_order.get_untracked(), id);
        sort_column.set(next_col);
        sort_order.set(next_order);
    });

    let container = classes.container;
    let table_class = classes.table;
    let search_input_class = classes.search_input;
    let search_placeholder = texts.search_placeholder;
    let header_columns = columns.clone();
    let body_columns = columns;
    let header_classes = classes.clone();
    let body_classes = classes.clone();
    let controls_classes = classes;
    let body_texts = texts.clone();
    let controls_texts = texts;

    view! {
        <div class=container>
            {search
                .then(|| {
                    view! {
                        <input
                            class=search_input_class
                            type="text"
                            placeholder=search_placeholder
                            aria-label="Search table"
                            prop:value=move || search_query.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                update_search_url(&value);
                                search_query.set(value);
                            }
                        />
                    }
                })}
            <table class=table_class role="table">
                <TableHeader
                    columns=header_columns
                    sort_column=sort_column
                    sort_order=sort_order
                    on_sort=on_sort
                    classes=header_classes
                />
                <TableBody
                    columns=body_columns
                    rows=rows
                    loading=loading
                    classes=body_classes
                    texts=body_texts
                />
            </table>
            {paginate
                .then(|| {
                    view! {
                        <PaginationControls
                            page=page
                            total_pages=total_pages
                            classes=controls_classes
                            texts=controls_texts
                        />
                    }
                })}
        </div>
    }
}
