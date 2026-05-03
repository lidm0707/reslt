use reslt_core::prelude::FieldAccessible;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;

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

/// Get hardcoded person data for demo purposes
pub fn get_hardcoded_data() -> Vec<Person> {
    vec![
        Person {
            id: 1,
            name: "John Doe".to_string(),
            age: 30,
            city: "New York".to_string(),
            created_at: "2024-01-15 10:30:00".to_string(),
            updated_at: "2024-01-20 14:45:00".to_string(),
        },
        Person {
            id: 2,
            name: "Jane Smith".to_string(),
            age: 25,
            city: "Los Angeles".to_string(),
            created_at: "2024-02-01 09:15:00".to_string(),
            updated_at: "2024-02-05 16:20:00".to_string(),
        },
        Person {
            id: 3,
            name: "Bob Johnson".to_string(),
            age: 35,
            city: "Chicago".to_string(),
            created_at: "2024-02-10 11:00:00".to_string(),
            updated_at: "2024-02-12 13:30:00".to_string(),
        },
        Person {
            id: 4,
            name: "Alice Williams".to_string(),
            age: 28,
            city: "Houston".to_string(),
            created_at: "2024-03-01 08:45:00".to_string(),
            updated_at: "2024-03-05 15:10:00".to_string(),
        },
        Person {
            id: 5,
            name: "Charlie Brown".to_string(),
            age: 42,
            city: "Phoenix".to_string(),
            created_at: "2024-03-15 12:00:00".to_string(),
            updated_at: "2024-03-18 10:30:00".to_string(),
        },
    ]
}

/// Get person data for the table
pub async fn get_person_data() -> anyhow::Result<Vec<Person>> {
    // In a real application, this would fetch from a database or API
    // For now, we'll use the hardcoded data
    let data = get_hardcoded_data();

    // Simulate network delay
    async_std::task::sleep(std::time::Duration::from_millis(100)).await;

    Ok(data)
}

/// Wrapper function for use_table that matches the required signature
pub fn get_person_data_wrapper() -> Box<
    dyn Fn(
        usize,
        usize,
        (String, bool),
    ) -> Pin<Box<dyn Future<Output = (reslt_core::prelude::PropData<Person>, usize)>>>,
> {
    Box::new(move |start: usize, end: usize, sort: (String, bool)| {
        Box::pin(async move {
            match get_person_data().await {
                Ok(mut data) => {
                    // Filter data based on start and end
                    let filtered_data: Vec<Person> =
                        data.into_iter().skip(start).take(end - start).collect();

                    // Sort data if sort column is provided
                    let mut sorted_data = filtered_data;
                    if !sort.0.is_empty() {
                        sort_by_field(&mut sorted_data, &sort.0, sort.1);
                    }

                    // Return PropData and total count
                    let total = sorted_data.len() + start;
                    let prop_data = reslt_core::prelude::PropData {
                        data_vec: sorted_data,
                    };
                    (prop_data, total)
                }
                Err(_) => (
                    reslt_core::prelude::PropData {
                        data_vec: Vec::new(),
                    },
                    0,
                ),
            }
        })
    })
}

/// Sort a vector of Person by a field
fn sort_by_field(data: &mut Vec<Person>, field: &str, descending: bool) {
    data.sort_by(|a, b| {
        let cmp = match field {
            "id" => a.id.cmp(&b.id),
            "name" => a.name.cmp(&b.name),
            "age" => a.age.cmp(&b.age),
            "city" => a.city.cmp(&b.city),
            "created_at" => a.created_at.cmp(&b.created_at),
            "updated_at" => a.updated_at.cmp(&b.updated_at),
            _ => std::cmp::Ordering::Equal,
        };

        if descending {
            cmp.reverse()
        } else {
            cmp
        }
    });
}

/// Delete multiple rows by their IDs
pub async fn delete_rows(ids: Vec<u32>) -> anyhow::Result<()> {
    // In a real application, this would delete from a database
    // For now, we'll just simulate the operation

    println!("Deleting rows with IDs: {:?}", ids);

    // Simulate network delay
    async_std::task::sleep(std::time::Duration::from_millis(500)).await;

    Ok(())
}

/// Create a new person
pub async fn create_person(person: Person) -> anyhow::Result<Person> {
    // In a real application, this would insert into a database
    // For now, we'll just return the person

    println!("Creating person: {:?}", person);

    // Simulate network delay
    async_std::task::sleep(std::time::Duration::from_millis(500)).await;

    Ok(person)
}

/// Update an existing person
pub async fn update_person(id: u32, person: Person) -> anyhow::Result<Person> {
    // In a real application, this would update in a database
    // For now, we'll just return the person

    println!("Updating person with ID {}: {:?}", id, person);

    // Simulate network delay
    async_std::task::sleep(std::time::Duration::from_millis(500)).await;

    Ok(person)
}
