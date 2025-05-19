use std::fmt::Debug;

use dioxus::prelude::*;
use serde::Serialize;
use crate::prelude::*;

#[component]
pub fn TableHeader(class: Option<String>, children: Element) -> Element {
    rsx! {
        thead { class: class.unwrap_or_default(), {children} }
    }
}

#[component]
pub fn TableHead(class: Option<String>, children: Element) -> Element {
    rsx! {
        th { class: class.unwrap_or_default(), scope: "col", {children} }
    }
}

#[component]
pub fn TableBody(class: Option<String>, children: Element) -> Element {
    rsx! {
        tbody { class: class.unwrap_or_default(), {children} }
    }
}

#[component]
pub fn TableRow(class: Option<String>, children: Element) -> Element {
    rsx! {
        tr { class: class.unwrap_or_default(), {children} }
    }
}

#[component]
pub fn TableCell(class: Option<String>, children: Element) -> Element {
    rsx! {
        td { class: class.unwrap_or_default(), {children} }
    }
}

#[component]
pub fn TableMain(class: Option<String>, children: Element) -> Element {
    rsx! {

        div {
            table { class: class.unwrap_or_default(), {children} }
        }
    }
}



#[component]
pub fn DefaultChildren<T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug>(
    row: T,
    col: Col<T>,
) -> Element {
    rsx! {
        {
            if let Some(action) = col.action {
                action(row.to_owned())
            } else {
                rsx! {
                    {row.get_field(&col.index).unwrap_or_default()}
                }
            }
        }
    }
}

#[component]
pub fn ContainerTable(
    class: Option<String>, children: Element
) -> Element {
    rsx! {
        div { class: "overflow-y-scroll h-100 w-screen ", {children} }
    }
}







