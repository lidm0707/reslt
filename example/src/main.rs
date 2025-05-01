use dioxus::prelude::*;
use example::pages::home::table::{
    service::get_person_data, table_col::create_col, table_data::Person,
};
use reslt::prelude::*;

#[component]
fn CheckMethod() -> Element {
    rsx! {
        {
            use_context::<UseCheckBox<Person>>()
                .to_owned()
                .get_checked_data()
                .into_iter()
                .map(|row| {
                    rsx! {
                        div { "ID: {row.id}, Name: {row.name}, Age: {row.age}, City: {row.city}" }
                    }
                })
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
                CheckBox {
                    checkbox,
                    method: rsx! {
                        CheckMethod {}
                    },
                    DefaultTable { table }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
