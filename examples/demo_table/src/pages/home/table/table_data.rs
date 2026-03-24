use reslt_core::prelude::FieldAccessible;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub city: String,
    pub created_at: String,
    pub updated_at: String,
}

impl FieldAccessible for Person {
    fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "id" => Some(self.id.to_string()),
            "name" => Some(self.name.clone()),
            "age" => Some(self.age.to_string()),
            "city" => Some(self.city.clone()),
            "created_at" => Some(self.created_at.clone()),
            "updated_at" => Some(self.updated_at.clone()),
            _ => None,
        }
    }
}
