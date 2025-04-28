use chrono::{DateTime, Utc};
use reslt::prelude::*;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Clone, FieldAccessible, Debug)]
//
pub struct Person {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub city: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
