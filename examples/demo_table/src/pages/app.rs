use crate::pages::home::{delete_rows, get_person_data_wrapper, ModalCreate, Person};
use crate::pages::{create_person_columns, DefaultTable, ToastContainer};
use dioxus::prelude::*;

use reslt_core::prelude::*;

#[component]
fn CheckMethod() -> Element {
    let context = use_context::<UseCheckBox<Person>>().to_owned();

    let checked_data = context.get_checked_data();
    let count = checked_data.len();
    let ids_to_delete: Vec<u32> = context
        .get_checked_data()
        .into_iter()
        .map(|row| row.id)
        .collect();

    rsx! {
        div {
            div { "Selected Count: {count}" }
            button {
                class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition",
                onclick: move |_| {
                    let ids_to_delete = ids_to_delete.clone();
                    async move {
                        delete_rows(ids_to_delete).await;
                    }
                },
                "Delete Selected"
            }
        }
    }
}

#[component]
fn FancyButton() -> Element {
    let modal = use_context::<UseModal>();

    rsx! {
        button {
            onclick: move |_| {
                modal.set_title("Create");
                modal.set_content(rsx! {
                    ModalCreate {}
                });
                modal.open();
            },
            "Create"
        }
    }
}

#[component]
pub fn App() -> Element {
    let cols = create_person_columns();

    // Provide context providers
    use_checkbox_provider::<Person>();
    use_context_provider(|| UseModal {
        title: use_signal(|| String::new()),
        content: use_signal(|| None),
        is_open: use_signal(|| false),
    });
    use_context_provider(|| UseToast {
        messages: use_signal(|| Vec::new()),
    });

    // Create table using the wrapper function
    let table = use_table(
        get_person_data_wrapper(),
        cols.to_owned(),
        Some(SortState {
            column: Some("id".to_owned()),
            descending: true,
        }),
    );

    // Create a signal for the table
    let mut table_signal = use_signal(|| table);

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/output.css"),
        }

        ToastContainer {}

        div {
            FancyButton {}
        }

        DefaultTable {
            table: table_signal,
            checkbox_method: rsx! {
                CheckMethod {}
            },
            column_check: true,
        }
    }
}
