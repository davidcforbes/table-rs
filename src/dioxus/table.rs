use dioxus::prelude::*;

#[cfg(target_family = "wasm")]
use web_sys::UrlSearchParams;
#[cfg(target_family = "wasm")]
use web_sys::wasm_bindgen::JsValue;

use crate::core;
use crate::dioxus::body::TableBody;
use crate::dioxus::controls::PaginationControls;
use crate::dioxus::header::TableHeader;
use crate::dioxus::types::SortOrder;
use crate::dioxus::types::TableProps;

/// A fully featured table component with sorting, pagination, and search functionality in Dioxus.
///
/// This component renders an interactive HTML `<table>` with customizable columns, data,
/// class names, and labels. It supports client-side sorting, search with URL hydration,
/// and pagination.
///
/// # Props
/// `TableProps` defines the configuration for this component:
/// - `data`: A `Vec<HashMap<&'static str, String>>` representing row data.
/// - `columns`: A `Vec<Column>` describing each column's ID, header text, and behavior.
/// - `page_size`: Number of rows to display per page (default: `10`).
/// - `loading`: When `true`, displays a loading indicator (default: `false`).
/// - `paginate`: Enables pagination controls (default: `false`).
/// - `search`: Enables a search input for client-side filtering (default: `false`).
/// - `texts`: Customizable text labels for UI strings (default: `TableTexts::default()`).
/// - `classes`: Customizable CSS class names for each table part (default: `TableClasses::default()`).
///
/// # Features
/// - **Search**: Filters rows client-side using a text input; the query is persisted in the URL via `?search=`.
/// - **Sorting**: Clickable headers allow sorting columns ascending or descending.
/// - **Pagination**: Navigate between pages using prev/next buttons, with an indicator showing current page.
/// - **Custom Classes**: All elements are styled via `TableClasses` for full customization.
/// - **Text Overrides**: All UI strings (e.g., empty state, loading, buttons) can be customized using `TableTexts`.
///
/// # Returns
/// Returns a `Dioxus` `Element` that renders a complete table with the above features.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use maplit::hashmap;
/// use table_rs::dioxus::table::Table;
/// use table_rs::dioxus::types::Column;
///
///
/// fn App() -> Element {
///     let data = vec![
///         hashmap! { "name" => "ferris".to_string(), "email" => "ferris@opensass.org".to_string() },
///         hashmap! { "name" => "ferros".to_string(), "email" => "ferros@opensass.org".to_string() },
///     ];
///
///     let columns = vec![
///         Column { id: "name", header: "Name", sortable: true, ..Default::default() },
///         Column { id: "email", header: "Email", ..Default::default() },
///     ];
///
///     rsx! {
///         Table {
///             data: data,
///             columns: columns,
///             paginate: true,
///             search: true,
///         }
///     }
/// }
/// ```
///
/// # See Also
/// - [MDN `<table>` Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/table)
#[component]
pub fn Table(props: TableProps) -> Element {
    let TableProps {
        data,
        columns,
        page_size,
        loading,
        paginate,
        search,
        texts,
        classes,
    } = props;

    let mut page = use_signal(|| 0_usize);
    let mut sort_column = use_signal(|| None::<&'static str>);
    let mut sort_order = use_signal(SortOrder::default);
    let mut search_query = use_signal(String::new);

    // Reset page to 0 when search query changes to prevent invalid page states
    use_effect(use_reactive!(|search_query| {
        let _ = search_query; // Explicitly depend on search_query
        page.set(0);
    }));

    #[cfg(target_family = "wasm")]
    use_effect(move || {
        if let Some(search_val) = web_sys::window()
            .and_then(|w| w.location().search().ok())
            .and_then(|search| UrlSearchParams::new_with_str(&search).ok())
            .and_then(|params| params.get("search"))
        {
            search_query.set(search_val);
        }
    });

    #[cfg(target_family = "wasm")]
    let update_search_param = move |query: &str| {
        let _ = web_sys::window().and_then(|window| {
            let href = window.location().href().ok()?;
            let url = web_sys::Url::new(&href).ok()?;
            let params = url.search_params();
            params.set("search", query);
            url.set_search(&params.to_string().as_string().unwrap_or_default());

            window
                .history()
                .ok()?
                .replace_state_with_url(&JsValue::NULL, "", Some(&url.href()))
                .ok()
        });
    };

    // Filter → sort → paginate, all via the framework-agnostic core.
    // Work with indices instead of cloning data to reduce allocations.
    let column_ids: Vec<&'static str> = columns.iter().map(|c| c.id).collect();
    let mut filtered_indices = core::filter_indices(&data, &column_ids, &search_query());

    if let Some(col_id) = sort_column() {
        core::sort_indices(&mut filtered_indices, &data, col_id, sort_order());
    }

    let page_window = core::paginate(filtered_indices.len(), page_size, page());
    let total_pages = page_window.total_pages;
    let page_rows: Vec<_> = filtered_indices[page_window.start..page_window.end]
        .iter()
        .map(|&idx| data[idx].clone())
        .collect();
    let page_rows = &page_rows[..];

    let on_sort_column = move |id: &'static str| {
        let (next_col, next_order) = core::toggle_sort(sort_column(), sort_order(), id);
        sort_column.set(next_col);
        sort_order.set(next_order);
    };

    let pagination_controls = if paginate {
        rsx! {
            PaginationControls {
                page: page,
                total_pages: total_pages,
                classes: classes.clone(),
                texts: texts.clone(),
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        div {
            class: "{classes.container}",
            if search {
                input {
                    class: "{classes.search_input}",
                    r#type: "text",
                    value: "{search_query()}",
                    placeholder: "{texts.search_placeholder}",
                    oninput: move |e| {
                        let val = e.value();
                        search_query.set(val.clone());
                        page.set(0);
                        #[cfg(target_family = "wasm")]
                        update_search_param(&val);
                    }
                }
            }
            table {
                class: "{classes.table}",
                TableHeader {
                    columns: columns.clone(),
                    sort_column: sort_column,
                    sort_order: sort_order,
                    on_sort_column: on_sort_column,
                    classes: classes.clone(),
                }
                TableBody {
                    columns: columns.clone(),
                    rows: page_rows.to_vec(),
                    loading: loading,
                    classes: classes.clone(),
                    texts: texts.clone(),
                }
            }
            {pagination_controls}
        }
    }
}
