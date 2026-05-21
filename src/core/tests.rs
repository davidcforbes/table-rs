//! Unit tests for the framework-agnostic core. These are pure functions,
//! so coverage here is cheap and exhaustive — this is where the
//! sort/filter/paginate logic is actually verified (the backends just
//! wire it to their state models).

use super::*;
use std::collections::HashMap;

fn rows() -> Vec<HashMap<&'static str, String>> {
    let mk = |name: &str, age: &str| {
        let mut m = HashMap::new();
        m.insert("name", name.to_string());
        m.insert("age", age.to_string());
        m
    };
    vec![mk("Charlie", "30"), mk("alice", "25"), mk("Bob", "40")]
}

// -- filter -----------------------------------------------------------------

#[test]
fn empty_query_returns_all_indices_in_order() {
    let data = rows();
    assert_eq!(filter_indices(&data, &["name", "age"], ""), vec![0, 1, 2]);
}

#[test]
fn filter_is_case_insensitive() {
    let data = rows();
    // "ALICE" should match the lowercase "alice" row (index 1).
    assert_eq!(filter_indices(&data, &["name"], "ALICE"), vec![1]);
}

#[test]
fn filter_matches_across_multiple_columns() {
    let data = rows();
    // "40" only appears in the age column of Bob (index 2).
    assert_eq!(filter_indices(&data, &["name", "age"], "40"), vec![2]);
}

#[test]
fn filter_only_searches_named_columns() {
    let data = rows();
    // Searching "40" but only in the name column finds nothing.
    assert!(filter_indices(&data, &["name"], "40").is_empty());
}

#[test]
fn filter_no_match_returns_empty() {
    let data = rows();
    assert!(filter_indices(&data, &["name", "age"], "zzz").is_empty());
}

// -- sort -------------------------------------------------------------------

#[test]
fn sort_ascending_is_lexicographic() {
    let data = rows();
    let mut idx = vec![0, 1, 2];
    sort_indices(&mut idx, &data, "name", SortOrder::Asc);
    // Lexicographic on raw strings: uppercase < lowercase, so
    // "Bob" < "Charlie" < "alice".
    assert_eq!(idx, vec![2, 0, 1]);
}

#[test]
fn sort_descending_reverses_order() {
    let data = rows();
    let mut idx = vec![0, 1, 2];
    sort_indices(&mut idx, &data, "name", SortOrder::Desc);
    assert_eq!(idx, vec![1, 0, 2]);
}

#[test]
fn sort_missing_column_is_a_stable_noop() {
    let data = rows();
    let mut idx = vec![0, 1, 2];
    // No row has this column → all compare equal → stable order kept.
    sort_indices(&mut idx, &data, "nonexistent", SortOrder::Asc);
    assert_eq!(idx, vec![0, 1, 2]);
}

// -- toggle_sort ------------------------------------------------------------

#[test]
fn toggle_selects_new_column_ascending() {
    assert_eq!(
        toggle_sort(None, SortOrder::Asc, "name"),
        (Some("name"), SortOrder::Asc)
    );
    assert_eq!(
        toggle_sort(Some("age"), SortOrder::Desc, "name"),
        (Some("name"), SortOrder::Asc)
    );
}

#[test]
fn toggle_flips_direction_on_same_column() {
    assert_eq!(
        toggle_sort(Some("name"), SortOrder::Asc, "name"),
        (Some("name"), SortOrder::Desc)
    );
    assert_eq!(
        toggle_sort(Some("name"), SortOrder::Desc, "name"),
        (Some("name"), SortOrder::Asc)
    );
}

// -- paginate ---------------------------------------------------------------

#[test]
fn paginate_basic_window() {
    let p = paginate(25, 10, 0);
    assert_eq!(p.total_pages, 3);
    assert_eq!(p.current_page, 0);
    assert_eq!((p.start, p.end), (0, 10));
}

#[test]
fn paginate_last_partial_page() {
    let p = paginate(25, 10, 2);
    assert_eq!(p.total_pages, 3);
    assert_eq!((p.start, p.end), (20, 25));
}

#[test]
fn paginate_clamps_out_of_range_page_to_last() {
    let p = paginate(25, 10, 99);
    assert_eq!(p.current_page, 2);
    assert_eq!((p.start, p.end), (20, 25));
}

#[test]
fn paginate_empty_data_is_one_page() {
    let p = paginate(0, 10, 0);
    assert_eq!(p.total_pages, 1);
    assert_eq!((p.start, p.end), (0, 0));
}

#[test]
fn paginate_page_size_zero_is_floored_to_one() {
    let p = paginate(3, 0, 0);
    // page_size floored to 1 → 3 pages, first window is a single row.
    assert_eq!(p.total_pages, 3);
    assert_eq!((p.start, p.end), (0, 1));
}
