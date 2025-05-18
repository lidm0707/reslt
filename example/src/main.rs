use dioxus::prelude::*;
use example::pages::home::table::{
    service::{delete_rows, get_person_data},
    table_col::create_col,
    table_data::Person,
};
use reslt::prelude::*;

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
                        use_context::<Resource<(PropData<Person>, usize)>>().restart();
                        use_context::<Signal<UseToast>>()().error("Deleted");
                        use_context::<UseCheckBox<Person>>().set_all_checked(Vec::new());
                    }
                },
                "Delete Selected"
            }
        }
    }
}

#[component]
fn App() -> Element {
    let cols = create_col();
    let table = use_table(get_person_data, cols.to_owned());
    let checkbox = use_checkbox::<Person>();
    println!("{:?}", checkbox);
    rsx! {
        document::Stylesheet { href: asset!("/assets/output.css") }

        ToastContainer {
            Modal {
                DefaultTable {
                    table,
                    checkbox_method: rsx! {
                        CheckMethod {}
                    },
                    column_check: true,
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
