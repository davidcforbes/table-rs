//! Framework-agnostic sorting primitives.
//!
//! The data model throughout table-rs is `Vec<HashMap<&'static str, String>>`
//! addressed by a `Vec<usize>` of row indices. Sorting reorders the index
//! slice in place using a stable lexicographic comparison on the chosen
//! column's string value, so callers keep ownership of the underlying data.

use std::collections::HashMap;

/// Sort direction for a column.
///
/// Canonical home for this enum — the per-framework `types` modules
/// re-export it (`pub use crate::core::SortOrder;`) so consumer-facing
/// paths like `table_rs::yew::types::SortOrder` stay stable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SortOrder {
    /// Ascending order (default).
    #[default]
    Asc,
    /// Descending order.
    Desc,
}

/// Compute the next `(sort_column, sort_order)` after a header click.
///
/// Clicking the already-sorted column flips the direction; clicking a
/// different column selects it and resets to ascending. This is the
/// sort-toggle FSM shared by every backend.
pub fn toggle_sort(
    current: Option<&'static str>,
    order: SortOrder,
    clicked: &'static str,
) -> (Option<&'static str>, SortOrder) {
    if current == Some(clicked) {
        let next = match order {
            SortOrder::Asc => SortOrder::Desc,
            SortOrder::Desc => SortOrder::Asc,
        };
        (Some(clicked), next)
    } else {
        (Some(clicked), SortOrder::Asc)
    }
}

/// Stably reorder `indices` by the `col_id` column's value, in `order`.
///
/// Rows missing the column compare as the empty string. Uses
/// `slice::sort_by`, which is stable, so rows with equal keys keep their
/// prior relative order.
pub fn sort_indices(
    indices: &mut [usize],
    data: &[HashMap<&'static str, String>],
    col_id: &'static str,
    order: SortOrder,
) {
    let empty = String::new();
    indices.sort_by(|&a, &b| {
        let a_val = data[a].get(col_id).unwrap_or(&empty);
        let b_val = data[b].get(col_id).unwrap_or(&empty);
        match order {
            SortOrder::Asc => a_val.cmp(b_val),
            SortOrder::Desc => b_val.cmp(a_val),
        }
    });
}
