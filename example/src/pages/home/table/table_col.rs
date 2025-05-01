use dioxus::prelude::*;
use reslt::prelude::*;
use std::rc::Rc;

use crate::pages::home::table::service::delete_row;

use super::table_data::Person;
#[component]
fn ModalEdit(row: Person) -> Element {
    rsx! {
        div { class: "mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-green-100",
            svg {
                class: "h-6 w-6 text-green-600",
                fill: "none",
                stroke: "currentColor",
                view_box: "0 0 24 24",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M5 13l4 4L19 7",
                }
            }
        }
        label { class: "block text-sm font-medium text-gray-700 mb-1", "{row.name}" }
        input { class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500" }
        label { class: "block text-sm font-medium text-gray-700 mb-1 mt-4", "{row.age}" }
        input { class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500" }
        label { class: "block text-sm font-medium text-gray-700 mb-1 mt-4", "{row.city}" }
        input { class: "w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500" }
        div {
            button {
                onclick: move |_| {
                    use_context::<UseModal>().close();
                    let _ = delete_row(row.id);
                    use_context::<Signal<UseToast>>()().success("Save Success");
                },
                "Save"
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
                head: "".to_owned(),
                index: "check".to_owned(),
                class: Some("text-right".to_owned()),
                action: Some(Rc::new(move |row: Person| {
                    let mut checked = use_signal(|| false);
                    rsx! {input{
                        r#type:"checkbox",
                        checked:checked(),
                        onchange: move |_| {
                            if !checked() {
                                use_context::<UseCheckBox<Person>>().to_owned().push_checked_data(row.to_owned());
    
                                checked.set(true);
                            } else {
                                use_context::<UseCheckBox<Person>>().to_owned().remove(row.to_owned());
                                checked.set(false);
                            }
                            
                        }
                    }}
                })),
            },
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
