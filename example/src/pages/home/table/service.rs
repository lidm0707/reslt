use async_std::task::sleep;
use std::time::Duration;
use std::{future::Future, pin::Pin};

use anyhow::Result;
use reslt::prelude::PropData;

use super::{hard_data::PERSON_ARRAY, table_data::Person};

pub fn get_person_data(
    start: usize,
    end: usize,
    sort: (String, bool),
) -> Pin<Box<dyn 'static + Future<Output = (PropData<Person>, usize)>>> {
    Box::pin(async move {
        println!("get_person_data");
        sleep(Duration::from_secs(1)).await;
        // Try to acquire the lock
        let lock_result = PERSON_ARRAY.lock();
        match lock_result {
            Ok(array) => {
                // Clone the entire array to sort it without modifying the original
                let mut sorted_data = array.clone();

                // Perform sorting
                let (key, ascending) = sort;
                match key.as_str() {
                    "id" => {
                        if !ascending {
                            sorted_data.sort_by(|a, b| a.id.cmp(&b.id));
                        } else {
                            sorted_data.sort_by(|a, b| b.id.cmp(&a.id));
                        }
                    }
                    "name" => {
                        if !ascending {
                            sorted_data.sort_by(|a, b| a.name.cmp(&b.name));
                        } else {
                            sorted_data.sort_by(|a, b| b.name.cmp(&a.name));
                        }
                    }
                    "age" => {
                        if !ascending {
                            sorted_data.sort_by(|a, b| a.age.cmp(&b.age));
                        } else {
                            sorted_data.sort_by(|a, b| b.age.cmp(&a.age));
                        }
                    }
                    _ => {
                        println!("Invalid sort key: {}", key);
                    }
                }

                // Determine the length of the array
                let len = sorted_data.len();

                // Clamp the range to valid bounds
                let start_clamped = start.min(len);
                let end_clamped = end.min(len);

                // Slice the sorted data
                let sliced_data = sorted_data[start_clamped..end_clamped].to_vec();

                // Return the result
                (
                    PropData {
                        data_vec: sliced_data,
                    },
                    len,
                )
            }
            Err(_) => {
                eprintln!("Failed to acquire lock on PERSON_ARRAY");

                // Return empty data and length of zero in case of an error
                (
                    PropData {
                        data_vec: Vec::new(),
                    },
                    0,
                )
            }
        }
    })
}

pub async fn delete_row(id: u32) {
    sleep(Duration::from_secs(1)).await;
    let mut array = PERSON_ARRAY.lock().unwrap(); // เข้าถึง Mutex
    *array = array
        .clone()
        .into_iter()
        .filter(|person| person.id != id)
        .collect();
    if let Some(person) = array.get(0) {
        println!("{}", array.len());
        println!("{:?}", person);
    } else {
        println!("No data available");
    }
}

pub async fn delete_rows(vec_id: Vec<u32>) {
    sleep(Duration::from_secs(1)).await;
    let mut array = PERSON_ARRAY.lock().unwrap(); // เข้าถึง Mutex
    *array = array
        .clone()
        .into_iter()
        .filter(|person| !vec_id.contains(&person.id))
        .collect();

    println!("array {:?}", array);
    if let Some(person) = array.get(0) {
        println!("rowss{}", array.len());
        println!("rowss{:?}", person);
    } else {
        println!("No data available");
    }
}

pub async fn update_row(updated_row: Person) -> Result<(), String> {
    let mut array = PERSON_ARRAY.lock().unwrap();
    if let Some(person) = array.iter_mut().find(|person| person.id == updated_row.id) {
        *person = updated_row;
        Ok(())
    } else {
        Err("Person not found".to_string())
    }
}

pub async fn add_row(mut add_row: Person) -> Result<()> {
    let mut array = PERSON_ARRAY.lock().unwrap();
    let max_id = array.iter().map(|person| person.id).max().unwrap_or(0);
    add_row.id = max_id + 1;
    array.push(add_row);
    Ok(())
}
