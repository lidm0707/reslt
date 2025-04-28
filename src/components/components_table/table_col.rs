use core::fmt;
use dioxus::prelude::*;
use serde::Serialize;
use std::rc::Rc;

use crate::prelude::FieldAccessible;

#[derive(Clone, Props)]
pub struct Col<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible,
{
    pub head: String,
    pub index: String,
    pub class: Option<String>,
    pub action: Option<Rc<dyn Fn(T) -> Element>>,
}
impl<T> Eq for Col<T> where T: 'static + Serialize + Eq + Clone + FieldAccessible {}
impl<T> PartialEq for Col<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible,
{
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.index == other.index
    }
}

impl<T> fmt::Debug for Col<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.class)
            .field(&self.head)
            .field(&self.index)
            .finish()
    }
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PropCol<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible,
{
    pub cols: Vec<Col<T>>,
}
