//! Framework-agnostic search/filter primitive.

use std::collections::HashMap;

/// Return the indices of rows that match `query`, by case-insensitive
/// substring across the given `column_ids`.
///
/// An empty query matches every row (returns `0..data.len()`). Takes
/// `column_ids` rather than the framework-bound `Column` type so the
/// core stays decoupled from Yew/Dioxus/Leptos prop derives; each
/// backend passes `columns.iter().map(|c| c.id)`.
pub fn filter_indices(
    data: &[HashMap<&'static str, String>],
    column_ids: &[&'static str],
    query: &str,
) -> Vec<usize> {
    if query.is_empty() {
        return (0..data.len()).collect();
    }
    let needle = query.to_lowercase();
    data.iter()
        .enumerate()
        .filter(|(_, row)| {
            column_ids.iter().any(|id| {
                row.get(*id)
                    .map(|v| v.to_lowercase().contains(&needle))
                    .unwrap_or(false)
            })
        })
        .map(|(idx, _)| idx)
        .collect()
}
