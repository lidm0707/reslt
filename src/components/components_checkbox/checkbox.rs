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
        div { class: format!("fixed   bottom-10 left-[calc(50%-10rem)] w-xs flex flex-col justify-center justify-items-center bg-white p-4 gap-4 border border-gray-200 shadow-lg rounded-lg  p-4 w-auto  z-40 {}", is_visible),
            {method}

        }
        {children}
    }
}
