use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Toast {
    pub id: u32,
    pub message: String,
    pub toast_type: ToastType,
}

#[derive(Clone)]
pub struct UseToast {
    pub toasts: Signal<Vec<Toast>>,
    pub counter: Signal<u32>,
}

impl UseToast {
    pub fn success(&mut self, message: &str) {
        self.show(message, ToastType::Success);
    }

    pub fn error(&mut self, message: &str) {
        self.show(message, ToastType::Error);
    }

    pub fn warning(&mut self, message: &str) {
        self.show(message, ToastType::Warning);
    }

    pub fn info(&mut self, message: &str) {
        self.show(message, ToastType::Info);
    }

    fn show(&mut self, message: &str, toast_type: ToastType) {
        let mut counter = self.counter;
        let mut toasts = self.toasts;

        *counter.write() += 1;
        let id = counter();

        toasts.write().push(Toast {
            id,
            message: message.to_owned(),
            toast_type: toast_type.to_owned(),
        });
    }

    pub fn get_toasts(&self) -> Vec<Toast> {
        self.toasts.read().to_vec()
    }

    pub fn remove(&mut self, id: u32) {
        let mut toasts = self.toasts.to_owned();
        toasts.write().retain(|t| t.id != id);
        println!("Remove toast ID {id}");
    }
}

/// Provides toast context to child components
pub fn use_toast_provider() {
    let toasts = use_signal(|| Vec::<Toast>::new());
    let counter = use_signal(|| 0);

    use_context_provider(|| UseToast { toasts, counter });
}

/// Returns the toast context for showing toasts
pub fn use_toast() -> UseToast {
    use_context::<UseToast>()
}
