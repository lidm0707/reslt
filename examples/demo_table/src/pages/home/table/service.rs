use super::hard_data::get_hardcoded_data;
use super::table_data::Person;
use anyhow::Result;
use dioxus::prelude::*;
use reslt_core::prelude::*;
use std::pin::Pin;

// Define ServerFnError as an alias for anyhow::Error
pub type ServerFnError = anyhow::Error;

/// Get person data for the table
pub async fn get_person_data() -> Result<Vec<Person>> {
    // In a real application, this would fetch from a database or API
    // For now, we'll use the hardcoded data
    let data = get_hardcoded_data();

    // Simulate network delay
    async_std::task::sleep(std::time::Duration::from_millis(100)).await;

    Ok(data)
}

/// Wrapper function for use_table that matches the required signature
pub fn get_person_data_wrapper(
) -> impl Fn(usize, usize, (String, bool)) -> Pin<Box<dyn Future<Output = (PropData<Person>, usize)>>>
{
    move |start: usize, end: usize, sort: (String, bool)| {
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
                    let prop_data = PropData {
                        data_vec: sorted_data,
                    };
                    (prop_data, total)
                }
                Err(_) => (
                    PropData {
                        data_vec: Vec::new(),
                    },
                    0,
                ),
            }
        })
    }
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
pub async fn delete_rows(ids: Vec<u32>) -> Result<()> {
    // In a real application, this would delete from a database
    // For now, we'll just simulate the operation

    println!("Deleting rows with IDs: {:?}", ids);

    // Simulate network delay
    async_std::task::sleep(std::time::Duration::from_millis(500)).await;

    Ok(())
}

/// Create a new person
pub async fn create_person(person: Person) -> Result<Person> {
    // In a real application, this would insert into a database
    // For now, we'll just return the person

    println!("Creating person: {:?}", person);

    // Simulate network delay
    async_std::task::sleep(std::time::Duration::from_millis(500)).await;

    Ok(person)
}

/// Update an existing person
pub async fn update_person(id: u32, person: Person) -> Result<Person> {
    // In a real application, this would update in a database
    // For now, we'll just return the person

    println!("Updating person with ID {}: {:?}", id, person);

    // Simulate network delay
    async_std::task::sleep(std::time::Duration::from_millis(500)).await;

    Ok(person)
}
