use crate::dioxus::ripple::use_ripple;
use crate::dioxus::types::Column;
use crate::dioxus::types::SortOrder;
use crate::dioxus::types::TableClasses;
use dioxus::prelude::*;

/// A table header component that renders sortable column headers for use within the `Table` component.
///
/// This component produces the `<thead>` section of a table using the provided column definitions,
/// handling rendering, sorting indicators (`aria-sort`), and user interaction to trigger sort changes.
///
/// # Props
/// - `columns`: A `Vec<Column>` defining the columns to display in the header. Each `Column` may be sortable and have optional styles or class overrides.
/// - `sort_column`: A `Signal<Option<&'static str>>` indicating which column (if any) is currently being sorted.
/// - `sort_order`: A `Signal<SortOrder>` indicating the current sort direction (`Asc` or `Desc`).
/// - `on_sort_column`: An `EventHandler<&'static str>` triggered when a sortable header cell is clicked. The column ID is passed as the event payload.
/// - `classes`: A `TableClasses` struct allowing custom class names for `<thead>`, `<tr>`, and `<th>` elements.
///
/// # Behavior
/// - Sortable columns show proper `aria-sort` attributes for accessibility (`ascending`, `descending`, or `none`).
/// - Clicking a sortable column emits an event to update sort state.
/// - Each column can override default styles and classes via `Column::style` and `Column::class`.
///
/// # Returns
/// Returns a `Dioxus` `Element` containing the `<thead>` with all column headers rendered as `<th>` elements.
///
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// use maplit::hashmap;
/// use table_rs::dioxus::table::Table;
/// use table_rs::dioxus::types::{Column, TableClasses, SortOrder};
/// use table_rs::dioxus::header::TableHeader;
///
///
/// fn App() -> Element {
///     let columns = vec![
///         Column { id: "name", header: "Name", sortable: true, ..Default::default() },
///         Column { id: "email", header: "Email", sortable: false, ..Default::default() },
///     ];
///
///     let sort_column = use_signal(|| Some("name"));
///     let sort_order = use_signal(|| SortOrder::Asc);
///
///     rsx! {
///         TableHeader {
///             columns: columns,
///             sort_column: sort_column,
///             sort_order: sort_order,
///             on_sort_column: move |col_id| println!("Sort column changed: {}", col_id),
///             classes: TableClasses::default(),
///         }
///     }
/// }
/// ```
///
/// # See Also
/// - [MDN `<thead>` Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/thead)
#[component]
pub fn TableHeader(
    columns: Vec<Column>,
    sort_column: Signal<Option<&'static str>>,
    sort_order: Signal<SortOrder>,
    on_sort_column: EventHandler<&'static str>,
    classes: TableClasses,
) -> Element {
    let header_cells = columns.iter().map(|col| {
        let col_id = col.id;
        let is_sorted = sort_column() == Some(col_id);
        let aria_sort = if is_sorted {
            match sort_order() {
                SortOrder::Asc => "ascending",
                SortOrder::Desc => "descending",
            }
        } else {
            "none"
        };
        // Sort-arrow drives the `trs-sort-arrow` motion class off
        // `data-direction`. Always rendered for sortable columns; without
        // the motion preamble the class is inert and the arrow stays
        // static.
        let arrow_direction = if col.sortable {
            if is_sorted {
                match sort_order() {
                    SortOrder::Asc => "asc",
                    SortOrder::Desc => "desc",
                }
            } else {
                "none"
            }
        } else {
            ""
        };

        let class = format!("{} {}", classes.header_cell, col.class.unwrap_or_default());

        rsx! {
            HeaderCell {
                key: "{col_id}",
                col_id,
                header: col.header,
                sortable: col.sortable,
                cell_class: class,
                style: col.style.unwrap_or_default(),
                arrow_direction,
                aria_sort,
                on_sort_column,
            }
        }
    });

    rsx! {
        thead { class: "{classes.thead}",
            tr { class: "{classes.row}", role: "row",
                {header_cells}
            }
        }
    }
}

/// A single `<th>`. Owns its own ripple so each header cell radiates
/// independently — extracted into a component because Dioxus hooks
/// (`use_ripple`) can't be called inside the column-mapping loop.
#[component]
fn HeaderCell(
    col_id: &'static str,
    header: &'static str,
    sortable: bool,
    cell_class: String,
    style: &'static str,
    arrow_direction: &'static str,
    aria_sort: &'static str,
    on_sort_column: EventHandler<&'static str>,
) -> Element {
    let ripple = use_ripple();
    // Ripple active only on sortable headers opted into motion
    // (`trs-ripple-host` added by `TableClasses::with_motion`).
    let ripple_on = sortable && cell_class.contains("trs-ripple-host");

    let onclick = move |e: Event<MouseData>| {
        if ripple_on {
            ripple.trigger(&e);
        }
        if sortable {
            on_sort_column.call(col_id);
        }
    };

    rsx! {
        th {
            role: "columnheader",
            class: "{cell_class}",
            style: "{style}",
            aria_sort: "{aria_sort}",
            onclick,
            "{header}"
            if sortable {
                span {
                    class: "trs-sort-arrow",
                    "data-direction": "{arrow_direction}",
                    aria_hidden: "true",
                    "▲"
                }
            }
            {ripple.overlay()}
        }
    }
}
