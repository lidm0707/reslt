use crate::prelude::*;
use dioxus::prelude::*;
use serde::Serialize;
use std::fmt::Debug;

#[component]
pub fn CheckBox<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    checkbox: UseCheckBox<T>,
    method: Element,
    children: Element,
) -> Element {
    provide_context(checkbox);
    let is_visible = if use_context::<UseCheckBox<T>>().get_checked_data().len() > 0 {
        "visible"
    } else {
        "invisible"
    };
    rsx! {
        div { class: format!("fixed bottom-0 w-full flex justify-center justify-items-center bg-white p-4 gap-4 border border-gray-200 shadow-lg rounded-lg w-auto z-20 {}", is_visible),
            div { class:  " p-4 w-auto", {method} }
        
        }
        {children}
    }
}
