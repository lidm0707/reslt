use core::fmt;
use std::fmt::{Debug, Display};

use serde::Serialize;

use crate::reslt_signal::table::table_trait::FieldAccessible;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropData<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug,
{
    pub data_vec: Vec<T>,
}

impl<T> Display for PropData<T>
where
    T: 'static + Serialize + Eq + Clone + FieldAccessible + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.data_vec)
    }
}
