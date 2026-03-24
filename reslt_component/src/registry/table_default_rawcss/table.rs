use dioxus::prelude::*;
use reslt_core::prelude::FieldAccessible;

/// Table column configuration
#[derive(Clone, PartialEq, Eq, Debug, Props)]
pub struct TableColProps {
    /// Column header text
    pub header: String,
    /// Field name to access data
    pub field: String,
    /// Whether the column is sortable
    pub sortable: bool,
    /// Custom render function for cell content
    pub render: Option<fn(String) -> String>,
}

impl Default for TableColProps {
    fn default() -> Self {
        Self {
            header: String::new(),
            field: String::new(),
            sortable: true,
            render: None,
        }
    }
}

/// Table row component
#[component]
pub fn TableRow<T>(
    /// Row data
    row: T,
    /// Column definitions
    #[props(into)]
    cols: Vec<TableColProps>,
    /// Row index
    #[props(default = 0)]
    index: usize,
) -> Element
where
    T: Clone + PartialEq + Eq + std::fmt::Debug + serde::Serialize + FieldAccessible + 'static,
{
    rsx! {
        tr {
            class: "reslt-table-row",
            for col in cols.iter() {
                td {
                    class: "reslt-table-cell",
                    {
                        let field = col.field.clone();
                        match col.render {
                            Some(render) => render(field),
                            None => row.get_field(&col.field).unwrap_or_default(),
                        }
                    }
                }
            }
        }
    }
}

/// Table header component with sorting support
#[component]
pub fn TableHeader(
    /// Column definitions
    #[props(into)]
    cols: Vec<TableColProps>,
    /// Current sort column
    #[props(default = None)]
    sort_column: Option<String>,
    /// Current sort direction (true = descending)
    #[props(default = true)]
    sort_descending: bool,
    /// Callback when a column header is clicked
    on_sort: EventHandler<String>,
) -> Element {
    rsx! {
        thead {
            tr {
                for col in cols.into_iter() {
                    th {
                        class: if col.sortable { "reslt-table-head reslt-sortable" } else { "reslt-table-head" },
                        onclick: move |_| {
                            if col.sortable {
                                on_sort.call(col.field.clone());
                            }
                        },

                        {col.header.clone()}

                        if col.sortable {
                            if let Some(ref sort_col) = sort_column {
                                if sort_col == &col.field {
                                    span {
                                        class: "reslt-sort-indicator",
                                        {if sort_descending { "▼" } else { "▲" }}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Loading skeleton for table
#[component]
pub fn TableSkeleton(
    /// Number of columns
    #[props(default = 5)]
    cols: usize,
    /// Number of rows to show
    #[props(default = 5)]
    rows: usize,
) -> Element {
    rsx! {
        tbody {
            class: "reslt-table-body",
            for _ in 0..rows {
                tr {
                    class: "reslt-skeleton-row",
                    for _ in 0..cols {
                        td {
                            class: "reslt-skeleton-cell",
                            div {
                                class: "reslt-skeleton-bar"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Pagination component
#[component]
pub fn Pagination(
    /// Current page number
    #[props(default = 0)]
    current_page: usize,
    /// Total number of pages
    #[props(default = 1)]
    total_pages: usize,
    /// Items per page
    #[props(default = 10)]
    items_per_page: usize,
    /// Total items
    #[props(default = 0)]
    total_items: usize,
    /// Callback when page changes
    on_page_change: EventHandler<usize>,
    /// Callback when items per page changes
    on_items_per_page_change: EventHandler<usize>,
) -> Element {
    let pages = vec![5, 10, 20, 50];

    rsx! {
        div {
            class: "reslt-pagination",

            // Items per page selector
            div {
                class: "reslt-pagination-info",
                label {
                    class: "reslt-pagination-label",
                    "Rows per page:"
                }
                select {
                    class: "reslt-pagination-select",
                    value: "{items_per_page}",
                    onchange: move |e| {
                        let value: usize = e.value().parse().unwrap_or(10);
                        on_items_per_page_change.call(value);
                    },
                    for page_size in pages {
                        option {
                            value: "{page_size}",
                            selected: page_size == items_per_page,
                            "{page_size}"
                        }
                    }
                }
            }

            // Page navigation
            div {
                class: "reslt-pagination-nav",
                button {
                    class: "reslt-pagination-button",
                    disabled: current_page == 0,
                    onclick: move |_| on_page_change.call(current_page.saturating_sub(1)),
                    "Previous"
                }

                span {
                    class: "reslt-pagination-current",
                    "{current_page + 1} of {total_pages}"
                }

                button {
                    class: "reslt-pagination-button",
                    disabled: current_page >= total_pages.saturating_sub(1),
                    onclick: move |_| on_page_change.call(current_page + 1),
                    "Next"
                }
            }

            // Total items info
            div {
                class: "reslt-pagination-total",
                "Total: {total_items} items"
            }
        }
    }
}

/// Main table component with raw CSS
///
/// # Example
/// ```rust
/// // Import from your project structure, e.g.:
/// // use components::table::*;
///
/// #[derive(Clone, Serialize, Eq, PartialEq, Debug, FieldAccessible)]
/// pub struct User {
///     pub id: i32,
///     pub name: String,
///     pub email: String,
///     pub age: i32,
/// }
///
/// fn UserTable() -> Element {
///     let cols = vec![
///         TableColProps { header: "ID".to_string(), field: "id".to_string(), sortable: true, render: None },
///         TableColProps { header: "Name".to_string(), field: "name".to_string(), sortable: true, render: None },
///         TableColProps { header: "Email".to_string(), field: "email".to_string(), sortable: true, render: None },
///         TableColProps { header: "Age".to_string(), field: "age".to_string(), sortable: true, render: None },
///     ];
///
///     let table = use_table(|start, end, sort| async move {
///         // Fetch data logic
///         let data = vec![
///             User { id: 1, name: "John".to_string(), email: "john@example.com".to_string(), age: 30 },
///             User { id: 2, name: "Jane".to_string(), email: "jane@example.com".to_string(), age: 25 },
///         ];
///         (PropData { data_vec: data }, 100)
///     }, cols.clone(), None);
///
///     let sort_state = table.sort_state.read().clone();
///     let page_state = table.page_state.read().clone();
///     let total_pages = (page_state.total_items + page_state.items_per_page - 1) / page_state.items_per_page;
///
///     rsx! {
///         TableStylesheet {}
///
///         Table {
///             cols: cols.clone(),
///             rows: table.get_rows(),
///             is_loading: table.is_loading(),
///             sort_column: sort_state.column,
///             sort_descending: sort_state.descending,
///             current_page: page_state.current_page,
///             total_pages,
///             items_per_page: page_state.items_per_page,
///             total_items: page_state.total_items,
///             on_sort: move |field| table.sort_by_field(&field),
///             on_page_change: move |page| table.set_page(page),
///             on_items_per_page_change: move |items| table.set_items_per_page(items),
///         }
///     }
/// }
/// ```
#[component]
pub fn Table<T>(
    /// Column definitions
    #[props(into)]
    cols: Vec<TableColProps>,
    /// Row data to display
    #[props(into)]
    rows: Vec<T>,
    /// Whether the table is in loading state
    #[props(default = false)]
    is_loading: bool,
    /// Current sort column
    #[props(default = None)]
    sort_column: Option<String>,
    /// Current sort direction (true = descending)
    #[props(default = true)]
    sort_descending: bool,
    /// Current page number
    #[props(default = 0)]
    current_page: usize,
    /// Total number of pages
    #[props(default = 1)]
    total_pages: usize,
    /// Items per page
    #[props(default = 10)]
    items_per_page: usize,
    /// Total items
    #[props(default = 0)]
    total_items: usize,
    /// Callback when a column is clicked for sorting
    on_sort: EventHandler<String>,
    /// Callback when page changes
    on_page_change: EventHandler<usize>,
    /// Callback when items per page changes
    on_items_per_page_change: EventHandler<usize>,
    /// Additional CSS classes for the table container
    #[props(default = String::new())]
    class: String,
) -> Element
where
    T: Clone + PartialEq + Eq + std::fmt::Debug + serde::Serialize + FieldAccessible + 'static,
{
    rsx! {
        div {
            class: "reslt-table-container {class}",
            div {
                class: "reslt-table-wrapper",
                div {
                    class: "reslt-table-card",

                    table {
                        class: "reslt-table",

                        TableHeader {
                            cols: cols.clone(),
                            sort_column: sort_column.clone(),
                            sort_descending,
                            on_sort,
                        }

                        if is_loading {
                            TableSkeleton {
                                cols: cols.len(),
                                rows: 5,
                            }
                        } else if rows.is_empty() {
                            tbody {
                                class: "reslt-table-body",
                                tr {
                                    td {
                                        class: "reslt-table-cell reslt-table-empty",
                                        colspan: "{cols.len()}",
                                        "No data available"
                                    }
                                }
                            }
                        } else {
                            tbody {
                                class: "reslt-table-body",
                                for (index, row) in rows.iter().enumerate() {
                                    TableRow {
                                        key: "{index}",
                                        row: row.clone(),
                                        cols: cols.clone(),
                                        index,
                                    }
                                }
                            }
                        }
                    }

                    Pagination {
                        current_page,
                        total_pages,
                        items_per_page,
                        total_items,
                        on_page_change,
                        on_items_per_page_change,
                    }
                }
            }
        }
    }
}
