use crate::components::Button;
use crate::container::Container;
use dioxus::prelude::*;

/// StatsCard component - displays a single stat card
#[component]
fn StatsCard(icon_color: String, icon_path: String, label: String, value: String) -> Element {
    rsx! {
        div {
            class: "bg-white overflow-hidden shadow rounded-lg",
            div {
                class: "p-5",
                div {
                    class: "flex items-center",
                    div {
                        class: "flex-shrink-0 {icon_color} rounded-md p-3",
                        svg {
                            class: "h-6 w-6 text-white",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "{icon_path}"
                            }
                        }
                    }
                    div {
                        class: "ml-5 w-0 flex-1",
                        dl {
                            dt {
                                class: "text-sm font-medium text-gray-500 truncate",
                                "{label}"
                            }
                            dd {
                                class: "text-lg font-semibold text-gray-900",
                                "{value}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// StatsSection component - displays a grid of stat cards
#[component]
fn StatsSection() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-3 gap-6 mb-8",

            // Total persons card
            StatsCard {
                icon_color: "bg-blue-500".to_string(),
                icon_path: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z".to_string(),
                label: "Total Persons".to_string(),
                value: "5".to_string(),
            }

            // Active persons card
            StatsCard {
                icon_color: "bg-green-500".to_string(),
                icon_path: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z".to_string(),
                label: "Active Status".to_string(),
                value: "5 Active".to_string(),
            }

            // Recent activity card
            StatsCard {
                icon_color: "bg-yellow-500".to_string(),
                icon_path: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z".to_string(),
                label: "Last Update".to_string(),
                value: "Today".to_string(),
            }
        }
    }
}

/// CreateButton component - button to create a new person
#[component]
fn CreateButton() -> Element {
    rsx! {
        Button {
            label: "Create Person".to_string(),
            variant: "primary".to_string(),
            onclick: move |_| {
                // For now, just log - would open a modal in the full version
                println!("Create Person clicked");
            }
        }
    }
}

/// Home - Main page component that displays the person management interface
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50",

            // Header section with Container
            div {
                class: "bg-white shadow",
                Container {
                    padding: "py-6".to_string(),
                    div {
                        class: "flex items-center justify-between",

                        // Page title
                        div {
                            h1 {
                                class: "text-3xl font-bold text-gray-900",
                                "Person Management"
                            }
                            p {
                                class: "mt-1 text-sm text-gray-500",
                                "Manage and view all persons in the system"
                            }
                        }

                        // Create button
                        CreateButton {}
                    }
                }
            }

            // Main content section with Container
            Container {
                padding: "py-8".to_string(),

                // Stats cards
                StatsSection {}

                // Table placeholder section
                div {
                    class: "bg-white shadow rounded-lg",
                    div {
                        class: "px-4 py-5 sm:px-6 border-b border-gray-200",
                        h3 {
                            class: "text-lg leading-6 font-medium text-gray-900",
                            "All Persons"
                        }
                        p {
                            class: "mt-1 max-w-2xl text-sm text-gray-500",
                            "View and manage all persons in the system."
                        }
                    }

                    // Simple placeholder for table
                    div {
                        class: "p-4 text-center text-gray-500",
                        "Table component will be displayed here"
                    }
                }
            }
        }
    }
}
