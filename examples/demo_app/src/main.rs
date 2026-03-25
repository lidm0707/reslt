use demo_app::components::toast::{ToastContainer, ToastItem};
use demo_app::data::Person;
use demo_app::pages::Home;
use dioxus::prelude::*;
use reslt_core::prelude::*;

#[component]
fn App() -> Element {
    // Provide context providers at the root level
    use_checkbox_provider::<Person>();
    use_modal_provider();
    use_toast_provider();

    // Get toast context to display toasts
    let mut toast = use_toast();
    let toasts_vec = toast.read().get_toasts();
    let toasts: Vec<Element> = toasts_vec
        .into_iter()
        .map(|t| {
            rsx! {
                ToastItem {
                    key: "{t.id}",
                    toast: t,
                    on_close: move |id| toast.write().remove(id),
                }
            }
        })
        .collect();

    rsx! {
        ToastContainer {
            toasts
        }

        // Main home page
        Home {}
    }
}

fn main() {
    dioxus::launch(App);
}
