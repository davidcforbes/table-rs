//! Shared data types for the Leptos backend.
//!
//! Mirrors `crate::yew::types` / `crate::dioxus::types` but without the
//! framework-specific prop derives — Leptos components take these as
//! plain values. `SortOrder` is re-exported from the framework-agnostic
//! [`crate::core`].

pub use crate::core::SortOrder;

/// A column definition.
#[derive(Clone, PartialEq)]
pub struct Column {
    /// Unique identifier; also the key into each row's `HashMap`.
    pub id: &'static str,
    /// Header text shown at the top of the column.
    pub header: &'static str,
    /// Whether the column is sortable (clickable header).
    pub sortable: bool,
    /// Minimum width in pixels (kept for API parity with the other backends).
    pub min_width: u32,
    /// Optional inline style for the header cell.
    pub style: Option<&'static str>,
    /// Optional extra class(es) for the header cell.
    pub class: Option<&'static str>,
}

impl Default for Column {
    fn default() -> Self {
        Self {
            id: "",
            header: "",
            sortable: false,
            min_width: 100,
            style: Some("padding: 8px; font-weight: 600; text-align: left;"),
            class: Some("table-header-cell"),
        }
    }
}

/// Class names for each part of the table.
#[derive(Clone, PartialEq)]
pub struct TableClasses {
    pub container: &'static str,
    pub table: &'static str,
    pub thead: &'static str,
    pub tbody: &'static str,
    pub pagination: &'static str,
    pub search_input: &'static str,
    pub header_cell: &'static str,
    pub body_cell: &'static str,
    pub row: &'static str,
    pub loading_row: &'static str,
    pub empty_row: &'static str,
    pub pagination_button: &'static str,
}

impl Default for TableClasses {
    fn default() -> Self {
        Self {
            container: "table-container",
            table: "table",
            thead: "thead",
            tbody: "tbody",
            pagination: "pagination-controls",
            search_input: "search-input",
            header_cell: "th",
            body_cell: "td",
            row: "tr",
            loading_row: "loading-row",
            empty_row: "empty-row",
            pagination_button: "pagination-button",
        }
    }
}

impl TableClasses {
    /// `TableClasses` variant that opts the table into the motion system.
    ///
    /// Mirrors the Yew/Dioxus `with_motion()`. Mount
    /// [`crate::leptos::motion::MotionPreamble`] at app root for the
    /// `trs-*` classes to take visible effect.
    pub fn with_motion() -> Self {
        Self {
            pagination_button: "pagination-button trs-eased trs-pressable trs-focus-ring",
            search_input: "search-input trs-eased trs-focus-ring",
            row: "tr trs-eased",
            loading_row: "loading-row trs-fade-in",
            empty_row: "empty-row trs-fade-in",
            ..Self::default()
        }
    }
}

/// UI text strings.
#[derive(Clone, PartialEq)]
pub struct TableTexts {
    pub loading: &'static str,
    pub empty: &'static str,
    pub search_placeholder: &'static str,
    pub previous_button: &'static str,
    pub next_button: &'static str,
    /// Format string for the page indicator (`{current}` / `{total}`).
    pub page_indicator: &'static str,
}

impl Default for TableTexts {
    fn default() -> Self {
        Self {
            loading: "Loading...",
            empty: "No results found",
            search_placeholder: "Search...",
            previous_button: "Previous",
            next_button: "Next",
            page_indicator: "Page {current} of {total}",
        }
    }
}
