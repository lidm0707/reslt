use dioxus::prelude::*;
use example::pages::home::table::{service::get_person_data, table_col::create_col};
use reslt::prelude::*;

#[component]
fn App() -> Element {
    let check = use_signal(|| Vec::new());
    let cols = create_col(check);
    let table = use_table(get_person_data, cols.to_owned());

    rsx! {
        document::Stylesheet { href: asset!("/assets/output.css") }

        ToastContainer {
            Modal {
                DefaultTable { table }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
