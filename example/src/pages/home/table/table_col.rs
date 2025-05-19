use chrono::Utc;
use dioxus::prelude::*;
use reslt::prelude::*;
use std::rc::Rc;

use crate::pages::home::table::service::{delete_row, update_row};

use super::table_data::Person;
#[component]
fn ModalEdit(row: Person) -> Element {
    // Manage state for editable fields
    let mut name = use_signal(|| row.name.clone());
    let mut age = use_signal(|| row.age.to_string());
    let mut city = use_signal(|| row.city.clone());

    let save_changes = move || async move {
        let updated_row = Person {
            id: row.id,
            name: name.read().clone(),
            age: age.read().parse().unwrap_or(row.age), // Fallback to original value on parse failure
            city: city.read().clone(),
            created_at: row.created_at, // Keep the original creation time
            updated_at: Utc::now(),     // Set the current time as the updated timestamp
        };

        // Simulate an async save operation
        // Call your save or update function
        let result = update_row(updated_row).await;
        if result.is_ok() {
            use_context::<Signal<UseToast>>()().success("Save Success");
        } else {
            use_context::<Signal<UseToast>>()().error("Save Failed");
        }

        use_context::<UseModal>().close();
    };

    let handle_submit = move |event: FormEvent| async move {
        event.prevent_default(); // Prevent page reload
        save_changes().await;
        use_context::<Resource<(PropData<Person>, usize)>>().restart();
    };

    rsx! {
        div {
                form {
                    onsubmit: handle_submit,
                    label { class: "flex text-sm font-medium text-gray-700 mb-1", "Name" }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                        value: "{name}",
                        oninput: move |evt| name.set(evt.value()),
                        name: "name",
                    }
                    label { class: "flex text-sm font-medium text-gray-700 mb-1 mt-4", "Age" }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                        value: "{age}",
                        oninput: move |evt| age.set(evt.value()),
                        name: "age",
                    }
                    label { class: "flex text-sm font-medium text-gray-700 mb-1 mt-4", "City" }
                    input {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500",
                        value: "{city}",
                        oninput: move |evt| city.set(evt.value()),
                        name: "city",
                    }
                    label { class: "flex text-sm font-medium text-gray-700 mb-1 mt-4", "Created At" }
                    p { class: "text-gray-600", "{row.created_at}" }
                    label { class: "flex text-sm font-medium text-gray-700 mb-1 mt-4", "Updated At" }
                    p { class: "text-gray-600", "{row.updated_at}" }
                    div { class: "mt-6 flex justify-end",
                        button {
                            class: "btn-save",
                            r#type: "submit",
                            "Save"
                        }
                    }
                }
            
        }
    }
}

#[component]
fn ModalDelete(row: Person) -> Element {
    rsx! {
        div {
            button {
                onclick: move |_| async move {
                    use_context::<UseModal>().close();
                    delete_row(row.id).await;
                    use_context::<Resource<(PropData<Person>, usize)>>().restart();
                    use_context::<Signal<UseToast>>()().error("Deleted");
                },
                "Delete"
            }
        }
    }
}

fn dorpdown() -> Option<Rc<dyn Fn(Person) -> Element>> {
    Some(Rc::new({
        move |row: Person| {
            rsx! {
                {
                    let row_edit = row.to_owned();
                    let row_delete = row.to_owned();
                    rsx! {
                        ActionDropdown {
                            item: rsx! {
                                ItemDropdown {
                                    title: "Edit".to_string(),
                                    action: EventHandler::new(move |_| {
                                        use_context::<UseModal>().set_title("Edit Row");
                                        use_context::<UseModal>()
                                            .set_content({
                                                let row = row_edit.to_owned();
                                                rsx! {
                                                    ModalEdit { row }
                                                }
                                            });
                                        use_context::<UseModal>().open();
                                    }),
                                }
                                ItemDropdown {
                                    title: "Delete".to_string(),
                                    action: EventHandler::new(move |_| {
                                        use_context::<UseModal>().set_title("Delete");
                                        use_context::<UseModal>()
                                            .set_content({
                                                let row = row_delete.to_owned();
                                                rsx! {
                                                    ModalDelete { row }
                                                }
                                            });
                                        use_context::<UseModal>().open();
                                    }),
                                }
                            },
                        }
                    }
                }
            }
        }
    }))
}

pub fn create_col() -> PropCol<Person> {
    PropCol {
        cols: vec![
            Col {
                head: "Age".to_owned(),
                index: "age".to_owned(),
                class: Some("text-right".to_owned()),
                action: None,
            },
            Col {
                head: "Name".to_owned(),
                index: "name".to_owned(),
                class: None,
                action: None,
            },
            Col {
                head: "City".to_owned(),
                index: "city".to_owned(),
                class: None,
                action: Some(Rc::new(move |row: Person| {
                    rsx! {{row.get_field("city").unwrap_or_default()}}
                })),
            },
            Col {
                head: "Click".to_owned(),
                index: "click".to_owned(),
                class: Some(
                    "font-medium text-blue-600 dark:text-blue-500 hover:underline".to_owned(),
                ),
                action: Some(Rc::new(move |row: Person| {
                    rsx! {
                        button {
                            onclick: move |_| {
                                println!("Clicked Row: {:?}", row);
                                },
                            "click me"
                        }
                    }
                })),
            },
            Col {
                head: "Input".to_owned(),
                index: "input".to_owned(),
                class: None,
                action: Some(Rc::new(move |_row: Person| {
                    rsx! {
                        input {
                            onchange: move |evt| {
                                println!("{:?}",evt);

                            }
                        }
                    }
                })),
            },
            Col {
                head: "DropDown".to_owned(),
                index: "DropDown".to_owned(),
                class: None,
                action: dorpdown(),
            },
        ],
    }
}
