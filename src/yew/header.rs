use crate::yew::ripple::use_ripple;
use crate::yew::types::{SortOrder, TableHeaderProps};
use web_sys::MouseEvent;
use yew::prelude::*;

/// A table header component that renders column headers with optional sorting functionality.
///
/// This component is part of the `table_rs` Yew integration and is responsible for rendering
/// the `<thead>` section of a table. It supports sortable columns and emits sort events when
/// a sortable header is clicked.
///
/// # Arguments
/// * `props` - The properties passed to the component.
///   - `columns` - A list of column definitions (`Vec<Column>`) specifying the headers to render.
///   - `sort_column` - An `Option<&'static str>` indicating the currently sorted column, if any.
///   - `sort_order` - A `SortOrder` indicating whether the sort is ascending or descending.
///   - `on_sort_column` - A `Callback<&'static str>` triggered when a sortable column is clicked.
///   - `classes` - A `TableClasses` object defining CSS class names for customization.
///
/// # Returns
/// (Html): A rendered `<thead>` element containing the table header row and interactive sorting logic.
///
/// # Examples
/// ```rust
/// use table_rs::yew::header::TableHeader;
/// use table_rs::yew::types::{TableHeaderProps, Column, SortOrder, TableClasses};
/// use yew::prelude::*;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let columns = vec![
///         Column { id: "name", header: "Name", sortable: true, ..Default::default() },
///         Column { id: "email", header: "Email", sortable: false, ..Default::default() },
///     ];
///
///     let sort_order = use_state(|| SortOrder::Asc);
///     let sort_column = use_state(|| Some("name"));
///
///     let props = TableHeaderProps {
///         columns,
///         sort_column: sort_column,
///         sort_order: sort_order,
///         on_sort_column: Callback::from(|col_id| web_sys::console::log_1(&format!("Sort: {}", col_id).into())),
///         classes: Default::default(),
///     };
///    
///     html! {
///         <TableHeader ..props />
///     }
/// };
/// ```
///
/// # See Also
/// - [MDN thead Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/thead)
#[function_component(TableHeader)]
pub fn header(props: &TableHeaderProps) -> Html {
    let TableHeaderProps {
        columns,
        sort_column,
        sort_order,
        on_sort_column,
        classes,
    } = props;

    html! {
        <thead class={classes.thead}>
            <tr class={classes.row} role="row">
                { for columns.iter().map(|col| {
                    let sorted = Some(col.id) == **sort_column;
                    // Sort-arrow drives the `trs-sort-arrow` motion class
                    // off `data-direction`; aria-sort mirrors it for a11y.
                    let (arrow_direction, aria_sort) = if col.sortable {
                        if sorted {
                            match **sort_order {
                                SortOrder::Asc => ("asc", "ascending"),
                                SortOrder::Desc => ("desc", "descending"),
                            }
                        } else {
                            ("none", "none")
                        }
                    } else {
                        ("", "none")
                    };

                    html! {
                        <HeaderCell
                            col_id={col.id}
                            header={col.header}
                            sortable={col.sortable}
                            cell_class={format!("{} {}", classes.header_cell, col.class.unwrap_or("")).trim().to_string()}
                            style={col.style.unwrap_or_default()}
                            arrow_direction={arrow_direction}
                            aria_sort={aria_sort}
                            on_sort_column={on_sort_column.clone()}
                        />
                    }
                }) }
            </tr>
        </thead>
    }
}

/// Props for [`HeaderCell`]. The sort/aria values are pre-computed by
/// [`TableHeader`] (which re-renders on sort-state change), so this
/// component only needs plain values plus the click callback.
#[derive(Properties, PartialEq)]
pub struct HeaderCellProps {
    pub col_id: &'static str,
    pub header: &'static str,
    pub sortable: bool,
    pub cell_class: String,
    pub style: &'static str,
    pub arrow_direction: &'static str,
    pub aria_sort: &'static str,
    pub on_sort_column: Callback<&'static str>,
}

/// A single `<th>`. Owns its own ripple so each header cell radiates
/// independently — extracted into a component because Yew hooks
/// (`use_ripple`) can't be called inside the column-mapping loop.
#[function_component(HeaderCell)]
fn header_cell(props: &HeaderCellProps) -> Html {
    let ripple = use_ripple();
    // Ripple active only on sortable headers opted into motion
    // (`trs-ripple-host` added by `TableClasses::with_motion`).
    let ripple_on = props.sortable && props.cell_class.contains("trs-ripple-host");

    let onclick = {
        let on_sort = props.on_sort_column.clone();
        let col_id = props.col_id;
        let sortable = props.sortable;
        let ripple = ripple.clone();
        Callback::from(move |e: MouseEvent| {
            if ripple_on {
                ripple.trigger(&e);
            }
            if sortable {
                on_sort.emit(col_id);
            }
        })
    };

    html! {
        <th
            {onclick}
            role="columnheader"
            class={props.cell_class.clone()}
            style={props.style}
            aria-sort={props.aria_sort}
        >
            { props.header }
            if props.sortable {
                <span class="trs-sort-arrow" data-direction={props.arrow_direction} aria-hidden="true">{ "▲" }</span>
            }
            { ripple.overlay() }
        </th>
    }
}
