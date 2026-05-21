<div align="center">

# 📋 Table RS

[![Crates.io](https://img.shields.io/crates/v/table-rs)](https://crates.io/crates/table-rs)
[![Crates.io Downloads](https://img.shields.io/crates/d/table-rs)](https://crates.io/crates/table-rs)
![Crates.io License](https://img.shields.io/crates/l/table-rs)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)

[![Join our Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

<!-- absolute url for docs.rs cause assets is excluded from crate -->
![logo](https://raw.githubusercontent.com/opensass/table-rs/refs/heads/main/assets/logo.webp)

</div>

## 🎬 Demo

| Framework | Live Demo                                                                                                                |
| --------- | ------------------------------------------------------------------------------------------------------------------------ |
| Yew       | [![Netlify Status](https://api.netlify.com/api/v1/badges/4e1494d6-c19a-4a4c-b2d3-47d964214a71/deploy-status)](https://table-rs.netlify.app) |
| Dioxus    | [![Netlify Status](https://api.netlify.com/api/v1/badges/4e1494d6-c19a-4a4c-b2d3-47d964214a71/deploy-status)](https://table-dio.netlify.app) |
| Leptos    | TODO                                                                                                                     |

## 📜 Intro

**Table RS** is a **powerful**, **lightweight**, and **accessible** data table component designed specifically for **WASM frontends** like **Yew**, **Dioxus**, and **Leptos**. It supports sorting, pagination, searching (with URL sync), and is highly customizable via props.

## 🤔 Why Use Table RS?

The following are some of the reasons why **Table RS** should be your go-to table component:

1. **🔍 Built-in Search**: Real-time filtering with URL query parameter synchronization.
1. **🧹 Debounced Inputs**: Smooth user experience while searching.
1. **⬆️ Sorting Support**: Column-based sorting with accessible `aria-sort` attributes.
1. **📄 Pagination**: Built-in pagination controls for large datasets.
1. **🎨 Custom Styling**: Easily customize classes and styles.
1. **🧩 Accessibility First**: Semantic roles and ARIA attributes out of the box.

## Yew Usage

<!-- absolute url for docs.rs cause YEW.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/table-rs/blob/main/YEW.md) to integrate this component into your Yew app.

## 🧬 Dioxus Usage

<!-- absolute url for docs.rs cause DIOXUS.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/table-rs/blob/main/DIOXUS.md) to integrate this component into your Dioxus app.

## 🌱 Leptos Usage

The `lep` feature ships a full `Table` component (`table_rs::leptos::table::Table`)
with search, sorting, and pagination — sharing the same framework-agnostic
core as the Yew and Dioxus backends. Pass the same `data: Vec<HashMap<&'static str, String>>`
and `columns: Vec<Column>` you would for the other frameworks:

```rust,ignore
use leptos::prelude::*;
use table_rs::leptos::table::Table;
use table_rs::leptos::types::{Column, TableClasses};

#[component]
fn App() -> impl IntoView {
    let columns = vec![
        Column { id: "name", header: "Name", sortable: true, ..Default::default() },
        Column { id: "email", header: "Email", ..Default::default() },
    ];
    let data = vec![/* HashMap rows keyed by column id */];
    view! {
        <Table data=data columns=columns page_size=10 paginate=true search=true />
    }
}
```

Opt into motion with `classes=TableClasses::with_motion()` and mount
`table_rs::leptos::motion::MotionPreamble` once at the app root.

## ✨ Optional motion system

The `crate::motion` module exposes a Fluent 2-inspired set of CSS primitives
(eased state transitions, press depression, animated focus rings, and a
`prefers-reduced-motion` guard). Opting in is two steps:

1. Mount `MotionPreamble` once at app root — the framework-specific
   component is at `table_rs::yew::motion::MotionPreamble`,
   `table_rs::dioxus::motion::MotionPreamble`, or
   `table_rs::leptos::motion::MotionPreamble`. It emits two `<style>`
   elements: a `:root { --trs-* }` token block and a class-definition
   block.
2. Pass `TableClasses::with_motion()` instead of `TableClasses::default()`
   to the `Table`. Pagination buttons, the search input, rows, loading
   and empty placeholders pick up the eased + pressable + focus-ring +
   fade-in behavior. Sortable column headers also gain a `▲` indicator
   that rotates 180° between asc and desc (always rendered for sortable
   columns; the rotation lights up only when the motion preamble is
   present).

Existing consumers see no visual change without both steps; the motion API
surface is purely additive. See `examples/yew` and `examples/dioxus` for
end-to-end setups.

## 🤝 Contributions

Contributions are welcome! Whether it's bug fixes, feature requests, or examples, we would love your help to make **Table RS** even better.

1. Fork the repository.
1. Create a new branch for your feature/bugfix.
1. Submit a pull request for review.

## 📜 License

**Table RS** is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this library in your projects.
