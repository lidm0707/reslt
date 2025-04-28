use core::fmt;

use dioxus::prelude::*;

#[derive(Clone)]
pub struct ModalState {
    pub is_open: bool,
    pub title: String,
    pub content: Element,
}

impl Eq for ModalState {}
impl PartialEq for ModalState {
    fn eq(&self, other: &Self) -> bool {
        self.is_open == other.is_open && self.title == other.title
    }
}

impl fmt::Debug for ModalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.is_open)
            .field(&self.title)
            .finish()
    }
}

pub fn use_modal() -> UseModal {
    UseModal {
        is_open: use_signal(|| false),
        title: use_signal(|| String::new()),
        content: use_signal(|| rsx! {}),
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UseModal {
    pub is_open: Signal<bool>,
    pub title: Signal<String>,
    pub content: Signal<Element>,
}

impl UseModal {
    pub fn open(&mut self) {
        self.is_open.set(true);
    }

    pub fn set_content(&mut self, content: Element) {
        self.content.set(content);
    }

    pub fn set_title(&mut self, title: &str) {
        self.title.set(title.to_string());
    }

    pub fn close(&mut self) {
        self.is_open.set(false);
    }

    pub fn is_open(&self) -> bool {
        *self.is_open.read()
    }

    pub fn get_title(&self) -> String {
        self.title.read().to_owned()
    }

    pub fn get_content(&self) -> Element {
        self.content.read().to_owned()
    }
}
