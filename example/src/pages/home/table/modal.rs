use chrono::Utc;
use dioxus::prelude::*;
use reslt::prelude::*;
use std::rc::Rc;

use crate::pages::home::table::service::add_row;

use super::table_data::Person;
#[component]
pub fn ModalCreate() -> Element {
    let row = Person {
        id: 0,
        name: "".to_string(),
        age: 0,
        city: "".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    // Manage state for editable fields
    let mut name = use_signal(|| row.name.clone());
    let mut age = use_signal(|| row.age.to_string());
    let mut city = use_signal(|| row.city.clone());

    let save_changes = move || async move {
        let row = Person {
            id: row.id,
            name: name.read().clone(),
            age: age.read().parse().unwrap_or(row.age), // Fallback to original value on parse failure
            city: city.read().clone(),
            created_at: row.created_at, // Keep the original creation time
            updated_at: Utc::now(),     // Set the current time as the updated timestamp
        };

        // Simulate an async save operation
        // Call your save or update function
        let result = add_row(row).await;
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