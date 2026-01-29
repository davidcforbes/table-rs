# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**table-rs** is a highly customizable data table component library for WASM frontends (Yew, Dioxus, Leptos). It provides sorting, pagination, search (with URL sync), and is optimized for performance with large datasets.

## Architecture

### Feature-Based Module System

The codebase uses Cargo features to enable framework-specific implementations:

- `yew` feature → `src/yew/` module
- `dio` feature → `src/dioxus/` module
- `lep` feature → `src/leptos/` module (TODO)

Each framework module contains:
- `table.rs` - Main table component with state management and URL hydration
- `header.rs` - Table header with sorting controls
- `body.rs` - Table body with pagination and filtering
- `controls.rs` - Pagination controls
- `types.rs` - Framework-specific prop types and structs

### Key Design Patterns

1. **URL-based state hydration**: Search queries sync with URL query params (`?search=`)
2. **Debounced search**: Yew uses `gloo-timers` for debouncing; Dioxus has built-in debouncing
3. **Component composition**: Table splits into Header/Body/Controls subcomponents for maintainability
4. **Props-based customization**: All styling via `TableClasses`, all text via `TableTexts`

### Important Types

- `Column` - Defines column metadata (id, header, sortable, custom cell rendering)
- `TableProps` - Main component props (data, columns, page_size, loading, paginate, search)
- `TableClasses` - CSS class overrides for all table elements
- `TableTexts` - UI text customization (loading, empty state, pagination labels)
- `SortOrder` - Enum for Asc/Desc sorting

## Development Commands

### Building and Testing

```bash
# Build library with all features
cargo build --all-features

# Run tests (CI uses this)
cargo test --all-features

# Check code without building
cargo check --all-features

# Format code
cargo fmt

# Lint code
cargo clippy --all-features
```

### Working with Examples

```bash
# Yew example (uses Trunk)
cd examples/yew
trunk serve

# Dioxus example (uses dx CLI)
cd examples/dioxus
dx serve --platform web

# Build Yew example for production
cd examples/yew
trunk build --release

# Build Dioxus example for production
cd examples/dioxus
dx build --platform web --release
```

### Version Management

```bash
# Bump version (uses bump2version)
bump2version patch  # 0.0.5 -> 0.0.6
bump2version minor  # 0.0.5 -> 0.1.0
bump2version major  # 0.0.5 -> 1.0.0
```

## Framework-Specific Considerations

### Yew
- Uses `gloo-timers` for debounced search input
- State management via `use_state` hooks
- Requires `features = ["yew"]` in dependencies

### Dioxus
- Built-in signal/hook system handles debouncing
- State management via `use_signal`
- Requires `features = ["dio"]` in dependencies
- Custom cell rendering via `cell: Option<Callback<String, Element>>`

### Adding Support for New Frameworks

1. Add feature in `Cargo.toml`: `my-framework = ["dep:my-framework"]`
2. Create `src/my_framework/` module with table/header/body/controls/types
3. Add conditional compilation in `src/lib.rs`: `#[cfg(feature = "my-framework")]`
4. Create example in `examples/my_framework/`
5. Update README.md with usage guide

## Common Patterns

### Custom Cell Rendering (Dioxus)
```rust
Column {
    id: "status",
    header: "Status",
    cell: Some(Callback::new(|value: String| {
        rsx! { span { class: "badge", "{value}" } }
    })),
    ..Default::default()
}
```

### Custom Styling
```rust
let classes = TableClasses {
    table: "custom-table",
    header_cell: "custom-header",
    ..Default::default()
};
```

### URL Search Hydration
The table automatically reads `?search=` from URL on mount and updates the URL when search input changes. This uses `web_sys::UrlSearchParams` for parsing.

## Release Process

1. Update version with `bump2version`
2. CI runs on push to main (build + test with all features)
3. Manually publish to crates.io: `cargo publish`
4. Examples are deployed to Netlify (see live demos in README)

## Important Notes

- The library requires `web-sys` features: `["Window", "UrlSearchParams", "Url", "Location", "History"]`
- Release profile is heavily optimized for WASM size: `opt-level = "z"`, `lto = "thin"`, `strip = "symbols"`
- Leptos support is planned but not yet implemented
- All examples use `maplit::hashmap!` for convenient HashMap creation in demo data
