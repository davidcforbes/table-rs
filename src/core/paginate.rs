//! Framework-agnostic pagination math.

/// The resolved page window for a given filtered row count, page size,
/// and requested page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Page {
    /// Total number of pages — always at least 1 (so an empty table reads
    /// "Page 1 of 1" rather than "Page 1 of 0").
    pub total_pages: usize,
    /// The requested page clamped into `0..total_pages`.
    pub current_page: usize,
    /// Index into the filtered list where this page starts (inclusive).
    pub start: usize,
    /// Index into the filtered list where this page ends (exclusive).
    pub end: usize,
}

/// Compute the [`Page`] window. `page_size` is floored at 1 to avoid
/// division by zero; `page` is clamped so an out-of-range request lands
/// on the last page instead of showing an empty slice.
pub fn paginate(filtered_len: usize, page_size: usize, page: usize) -> Page {
    let page_size_safe = page_size.max(1);
    let total_pages = ((filtered_len as f64 / page_size_safe as f64).ceil() as usize).max(1);
    let current_page = page.min(total_pages.saturating_sub(1));
    let start = current_page * page_size_safe;
    let end = ((current_page + 1) * page_size_safe).min(filtered_len);
    Page {
        total_pages,
        current_page,
        start,
        end,
    }
}
