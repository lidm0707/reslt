<div align="center">
    <img src=".\crabby.png" alt="นี่คือตัวอย่างรูป" width="200" height="200">
</div>
# Reslt - Reactive Table Library for Dioxus

A powerful, customizable table component library for Dioxus applications with built-in sorting, pagination, and filtering capabilities.


## on YouTube 🎥  
[![Dioxus Table Tutorial](https://img.youtube.com/vi/EDeWMxA82Mg/0.jpg)](https://www.youtube.com/watch?v=EDeWMxA82Mg)  
👉 **Click the thumbnail or [here](https://www.youtube.com/watch?v=EDeWMxA82Mg) to watch the video.**



## Features

- 📊 Fully reactive data tables
- 🔄 Sorting functionality
- 📑 Pagination support
- 🔍 Filtering capabilities
- 🎨 Customizable styling with Tailwind CSS
- 📱 Responsive design
- 🧩 Modular components

## Installation

Add the library to your Cargo.toml:

```toml
[dependencies]
reslt = { path = "../" }  # Reference the dioxus_table library as a local dependency
```

## Quick Start

```rust
use dioxus::prelude::*;
use reslt::prelude::*;

// Define your data structure
#[derive(Clone, Debug, PartialEq, Eq, Serialize , FieldAccessible)]
struct Person {
    id: u32,
    name: String,
    age: u32,
    city: String,
}

// Create a function to fetch data
fn get_person_data(start: usize, end: usize, sort: (String, bool)) -> Pin<Box<dyn Future<Output = (PropData<Person>, usize)>>> {
    Box::pin(async move {
        // Your data fetching logic here
        // ...
    })
}

// Create table columns
fn create_columns() -> PropCol<Person> {
    PropCol {
        cols: vec![
            Col {
                head: "Name".to_owned(),
                index: "name".to_owned(),
                class: None,
                action: None,
            },
            Col {
                head: "Age".to_owned(),
                index: "age".to_owned(),
                class: Some("text-right".to_owned()),
                action: None,
            },
            // Add more columns as needed
        ],
    }
}

#[component]
fn App() -> Element {
    let cols = create_columns();
    let table = use_table(get_person_data, cols);

    rsx! {
        DefaultTable { table }
    }
}
```

## Documentation

For detailed documentation and examples, visit the [documentation site](https://github.com/yourusername/reslt).

## Development

### Prerequisites

- Rust and Cargo
- Node.js and npm (for Tailwind CSS)

### Setup

1. Clone the repository
2. Install dependencies:
   ```bash
   npm install
   ```
3. Run Tailwind CSS compiler:
   ```bash
   npx tailwindcss -i ./assets/input.css -o ./assets/output.css --watch
   ```
4. Run the example:
   ```bash
   cd example
   dx serve
   ```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

