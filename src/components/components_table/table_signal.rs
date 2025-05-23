use std::{fmt::Debug, future::Future, pin::Pin};

use dioxus::prelude::*;
use serde::Serialize;

use crate::prelude::{Col, FieldAccessible, PropCol, PropData};

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct PageState {
    pub current_page: usize,
    pub items_per_page: usize,
    pub total_items: usize,
}

#[derive(Default, Clone, PartialEq, Eq, Debug , Props)]
pub struct SortState {
    #[props(default = None)]
    pub column: Option<String>,
    #[props(default = true)]
    pub descending: bool,
}

// pub trait TableFetchFn<T>:
//     Fn(usize, usize, (String, bool)) -> Pin<Box<dyn Future<Output = (PropData<T>, usize)>>> + 'static + Clone
// where
//     T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug,
// {
// }

pub fn use_table<T>(
    fetch_fn: impl Fn(usize, usize, (String, bool)) -> Pin<Box<dyn Future<Output = (PropData<T>, usize)>>>
        + 'static
        + Clone,
    cols: PropCol<T>,
    sort_state: Option<SortState>,
) -> UseTable<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug,
{
    let mut state = UseTable {
        prop_data: use_signal(|| PropData {
            data_vec: Vec::<T>::new(),
        }),
        prop_col: use_signal(|| cols.to_owned()),
        sort_state: use_signal(|| sort_state.unwrap_or_default()),
        page_state: use_signal(|| PageState {
            current_page: 0,
            items_per_page: 10,
            total_items: 0,
        }),
        is_loading: use_signal(|| false),
    };

    let page = state.page_state.read().to_owned();
    let current_page = page.current_page;
    let items_per_page = page.items_per_page;
    let sort = state.sort_state.read().to_owned();

    let data_resource = use_resource(use_reactive!(|current_page, items_per_page, sort| {
        state.is_loading.set(true);
        let value = fetch_fn.to_owned();
        let sort = sort.to_owned();
        let start = current_page * page.items_per_page;
        let end = start + items_per_page;
        let resualt = async move {
            value.to_owned()(
                start,
                end,
                (
                    sort.column.to_owned().unwrap_or_default(),
                    sort.descending.to_owned(),
                ),
            )
            .await

            
        };


        resualt
    }));

    use_effect(use_reactive!(|(data_resource)| {
        match &*data_resource.read_unchecked() {
            Some(res) => {
                state.prop_data.set(res.0.to_owned());
                state.page_state.write().total_items = res.1;
                let total_pages = (res.1 + items_per_page.saturating_sub(1)) / items_per_page;
                if current_page >= total_pages {
                    state.page_state.write().current_page = total_pages.saturating_sub(1);
                }
                state.is_loading.set(false);
            }
            None => {
                state.prop_data.set(PropData {
                    data_vec: Vec::<T>::new(),
                });
                state.page_state.write().total_items = 0;
                state.page_state.write().current_page = 0;
                state.page_state.write().items_per_page = 10;
                state.is_loading.set(true);
            }
        }

    }));

    use_context_provider(|| data_resource);

    state
}

#[derive(Clone, PartialEq, Eq, Debug, Props)]
pub struct UseTable<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug,
{
    prop_data: Signal<PropData<T>>,
    prop_col: Signal<PropCol<T>>,
    sort_state: Signal<SortState>,
    page_state: Signal<PageState>,
    is_loading: Signal<bool>,
}

impl<T> UseTable<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug,
{
    pub fn is_loading(&self) -> bool {
        *self.is_loading.read()
    }

    pub fn sort_by_field(&mut self, field_name: &str) {
        let sort_col = self.get_sort_col();
        let sort_desc = self.get_sort_state();

        if sort_col == field_name.to_owned() {
            self.sort_state.set(SortState {
                column: Some(field_name.to_owned()),
                descending: !sort_desc,
            });
        } else {
            self.sort_state.set(SortState {
                column: Some(field_name.to_owned()),
                descending: false,
            });
        }
    }

    pub fn get_rows(&self) -> Vec<T> {
        self.prop_data.read().data_vec.to_owned()
    }

    pub fn get_cols(&self) -> Vec<Col<T>> {
        self.prop_col.read().cols.to_owned()
    }

    pub fn get_sort_state(&self) -> bool {
        self.sort_state.read().descending
    }

    pub fn get_sort_col(&self) -> String {
        self.sort_state.read().column.to_owned().unwrap_or_default()
    }

    pub fn get_page_state(&self) -> PageState {
        self.page_state.read().to_owned()
    }

    pub fn set_page(&mut self, page: usize) {
        self.page_state.write().current_page = page;
    }

    pub fn set_items_per_page(&mut self, items: usize) {
        self.page_state.write().items_per_page = items;
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading.set(loading);
    }


}
