use dioxus::prelude::*;

pub fn use_modal() -> UseModal {
    UseModal {
        status: use_signal(|| false),
        title: use_signal(|| String::new()),
        content: use_signal(|| rsx! {}),
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UseModal {
    pub status: Signal<bool>,
    pub title: Signal<String>,
    pub content: Signal<Element>,
}

impl UseModal {
    pub fn open(&mut self) {
        self.status.set(true);
    }

    pub fn set_content(&mut self, content: Element) {
        self.content.set(content);
    }

    pub fn set_title(&mut self, title: &str) {
        self.title.set(title.to_string());
    }

    pub fn close(&mut self) {
        self.status.set(false);
    }

    pub fn is_open(&self) -> bool {
        *self.status.read()
    }

    pub fn get_title(&self) -> String {
        self.title.read().to_owned()
    }

    pub fn get_content(&self) -> Element {
        self.content.read().to_owned()
    }
}
