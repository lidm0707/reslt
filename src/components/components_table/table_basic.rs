use std::fmt::Debug;

use crate::prelude::*;
use dioxus::prelude::*;
use serde::Serialize;

#[component]
pub fn TableHeader(style: String, children: Element) -> Element {
    rsx! {
        thead { style:style, {children} }
    }
}

#[component]
pub fn TableHead(style: String, children: Element) -> Element {
    rsx! {
        th { style:style, scope: "col", {children} }
    }
}

#[component]
pub fn TableBody(style: String, children: Element) -> Element {
    rsx! {
        tbody { style:style, {children} }
    }
}

#[component]
pub fn TableRow(style: String, children: Element) -> Element {
    rsx! {
        tr { style:style, {children} }
    }
}

#[component]
pub fn TableCell(style: String, children: Element) -> Element {
    rsx! {
        td { style:style, {children} }
    }
}

#[component]
pub fn TableMain(style: String, children: Element) -> Element {
    rsx! {
        div {
            table { style:style, {children} }
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
pub fn ContainerTable(style: String, children: Element) -> Element {
    rsx! {
        div { style:style, {children} }
    }
}
