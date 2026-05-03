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
    #[props(default = true)]
    pub sortable: bool,
    /// Custom render function for cell content
    #[props(default = None)]
    pub render: Option<fn(String) -> String>,
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
            class: "border-b hover:bg-gray-50 transition-colors",
            for col in cols.iter() {
                td {
                    class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                    if let Some(render) = col.render {
                        {render(col.field.clone())}
                    } else {
                        {row.get_field(&col.field)}
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
            class: "bg-gray-50",
            tr {
                for col in cols.into_iter() {
                    th {
                        class: if col.sortable {
                            "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100 select-none"
                        } else {
                            "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                        },
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
                                        class: "ml-1",
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
            class: "divide-y divide-gray-200",
            for _ in 0..rows {
                tr {
                    for _ in 0..cols {
                        td {
                            class: "px-6 py-4",
                            div {
                                class: "h-4 bg-gray-200 rounded animate-pulse"
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
            class: "flex items-center justify-between px-6 py-3 bg-white border-t",

            // Items per page selector
            div {
                class: "flex items-center space-x-2",
                label {
                    class: "text-sm text-gray-700",
                    "Rows per page:"
                }
                select {
                    class: "border border-gray-300 rounded-md px-3 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500",
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
                class: "flex items-center space-x-2",
                button {
                    class: "px-3 py-1 border rounded-md text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-100",
                    disabled: current_page == 0,
                    onclick: move |_| on_page_change.call(current_page.saturating_sub(1)),
                    "Previous"
                }

                span {
                    class: "text-sm text-gray-700",
                    "{current_page + 1} of {total_pages}"
                }

                button {
                    class: "px-3 py-1 border rounded-md text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-100",
                    disabled: current_page >= total_pages.saturating_sub(1),
                    onclick: move |_| on_page_change.call(current_page + 1),
                    "Next"
                }
            }

            // Total items info
            div {
                class: "text-sm text-gray-700",
                "Total: {total_items} items"
            }
        }
    }
}

/// Main table component with Tailwind CSS
///
/// # Example
/// ```ignore
/// // Import from your project structure, e.g.:
/// // use components::table::*;
///
/// #[component]
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
            class: "overflow-x-auto {class}",
            div {
                class: "min-w-full align-middle",
                div {
                    class: "overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg",

                    table {
                        class: "min-w-full divide-y divide-gray-300",

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
                                class: "divide-y divide-gray-200",
                                tr {
                                    td {
                                        class: "px-6 py-12 text-center text-gray-500",
                                        colspan: "{cols.len()}",
                                        "No data available"
                                    }
                                }
                            }
                        } else {
                            tbody {
                                class: "divide-y divide-gray-200 bg-white",
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
