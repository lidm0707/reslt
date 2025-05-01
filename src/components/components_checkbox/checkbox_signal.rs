use std::fmt::Debug;
use serde::Serialize;
use dioxus::prelude::*;
use crate::prelude::*;





pub fn use_checkbox<T>() -> UseCheckBox<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug,
{
    UseCheckBox {
        data_checked: use_signal(|| Vec::<T>::new()),
        is_all_checked: use_signal(|| false),
    }

}



#[derive(Clone, PartialEq, Eq, Debug, Props)]
pub struct UseCheckBox<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug
{
    data_checked: Signal<Vec<T>>,
    is_all_checked: Signal<bool>,
}

impl<T> UseCheckBox<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug,
{
    pub fn get_checked_data(&self) -> Vec<T> {
        self.data_checked.read().to_vec()
    }

    pub fn set_checked_data(&mut self, data: Vec<T>) {
        self.data_checked.set(data);
    }

    pub fn push_checked_data(&mut self, data: T) {
        self.data_checked.write().push(data);
    }

    pub fn is_all_checked(&self) -> bool {
        *self.is_all_checked.read()
    }

    pub fn set_all_checked(&mut self, checked: bool) {
        self.is_all_checked.set(checked);
    }


    pub fn remove(&mut self, predicate: T) {
        let mut data = self.data_checked.write();
        data.retain(|item| item != &predicate);
    }

}