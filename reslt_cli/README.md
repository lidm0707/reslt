# reslt-cli

A CLI tool for adding Reslt components from a registry to your Dioxus 0.7 projects.

## Overview

`reslt-cli` is a command-line interface that makes it easy to discover, download, and integrate pre-built UI components from the Reslt component registry into your Dioxus applications. All components follow Dioxus 0.7 best practices and are pure UI components without internal state management.

## Installation

### Install via Cargo

```sh
cargo install reslt-cli
```

This installs the `reslt` binary to your system.

### Install from Source

```sh
git clone https://github.com/lidm0707/reslt.git
cd reslt/reslt_cli
cargo install --path .
```

## Quick Start

```sh
# List all available components
reslt list

# Add a component interactively
reslt add

# Add a specific component
reslt add toast_default_tailwind

# Search for components
reslt search "notification"
```

## Commands

### `reslt list`

Lists all available components from the registry with their details.

```sh
reslt list
```

Output:
```
📦 Fetching component registry...

Available Components (2)

┌──────────────────────────────┬────────────┬──────────────────────┬──────────────────────────────────────────┐
│ Name                         │ Version    │ Category             │ Description                              │
├──────────────────────────────┼────────────┼──────────────────────┼──────────────────────────────────────────┤
│ toast_default_tailwind       │ 0.1.0      │ ui                   │ Toast notification component             │
├──────────────────────────────┼────────────┼──────────────────────┼──────────────────────────────────────────┤
│ toast_default_rawcss         │ 0.1.0      │ ui                   │ Toast notification component (raw CSS)   │
└──────────────────────────────┴────────────┴──────────────────────┴──────────────────────────────────────────┘
```

### `reslt add`

Downloads and installs a component into your project.

```sh
reslt add [OPTIONS] [NAME]
```

**Options:**
- `-o, --output <DIR>` - Output directory for component files (default: `./src/components`)
- `-s, --style <STYLE>` - Style choice: `raw-css`, `tailwind`, or `none`
- `--no-mod` - Don't add to `mod.rs` automatically

**Examples:**

```sh
# Interactive mode (select from list)
reslt add

# Add specific component with default options
reslt add toast_default_tailwind

# Add component to custom directory
reslt add toast_default_tailwind -o ./src/ui

# Add component with explicit style choice
reslt add toast_default_tailwind -s tailwind

# Add component without updating mod.rs
reslt add toast_default_tailwind --no-mod
```

**Installation Process:**
1. Fetches component registry
2. Displays component information
3. Confirms installation
4. Copies component files to output directory
5. Optionally updates `mod.rs` with module declaration

### `reslt search`

Searches for components by name, category, or description.

```sh
reslt search <QUERY>
```

**Example:**
```sh
reslt search "toast"
```

Output:
```
🔍 Searching for 'toast'...

Found 2 matching component(s):

📦 toast_default_tailwind (v0.1.0)
   Category: ui
   Description: Toast notification component

📦 toast_default_rawcss (v0.1.0)
   Category: ui
   Description: Toast notification component (raw CSS)
```

## Registry Configuration

### Local Registry

By default, `reslt-cli` looks for a local registry at `../reslt-component/registry`. You can specify a custom path:

```sh
reslt --registry-url /path/to/registry list
```

### Remote Registry

Remote registry support is planned but not yet implemented. For now, use a local registry.

## Component Structure and Naming

### Directory Naming Convention

Components in the registry follow this strict naming convention:

```
<name>-<variant>-<style>/
```

**Components:**
- `<name>` - The component name in kebab-case (e.g., `toast`, `button`, `modal`)
- `<variant>` - The style variant (e.g., `default`, `minimal`, `filled`)
- `<style>` - The styling approach (`tailwind` or `rawcss`)

**Examples:**
- `toast_default_tailwind/` - Toast component with Tailwind CSS
- `toast_default_rawcss/` - Toast component with raw CSS
- `button_filled_tailwind/` - Button component with filled variant and Tailwind
- `modal_minimal_rawcss/` - Modal component with minimal variant and raw CSS

### Component Directory Structure

Each component directory contains:

```
toast_default_tailwind/
├── metadata.json       # Component metadata and configuration
├── component.rs        # Main component entry point with documentation
├── toast.rs          # Component implementation
└── (optional) toast.css  # CSS styles (only for rawcss style)
```

### File Naming Conventions

#### 1. Component Files

- **Implementation file**: `<name>.rs` - Uses kebab-case matching directory name
  - Example: `toast.rs`, `button.rs`, `modal.rs`
  
- **Entry point file**: `component.rs` - Always named `component.rs`
  - Contains module exports and documentation
  - Imports the main implementation from `<name>.rs`

#### 2. CSS Files

- **Raw CSS**: `<name>.css` - Uses kebab-case matching component name
  - Example: `toast.css`, `button.css`
  - Only present for `rawcss` style variants

- **Tailwind**: No CSS file - Uses Tailwind utility classes directly

#### 3. Metadata File

- **Metadata**: `metadata.json` - Always named `metadata.json`
  - Contains component information, dependencies, and configuration

### Metadata Format

```json
{
  "name": "toast",
  "variant": "default",
  "style": "tailwind",
  "version": "0.1.0",
  "description": "Toast notification component",
  "category": "ui",
  "files": [
    "component.rs",
    "toast.rs"
  ],
  "dependencies": {
    "dioxus": "0.7.1"
  },
  "requires_tailwind": true
}
```

### Rust Naming Conventions

Within the Rust code:

- **Components**: PascalCase
  - Example: `Toast`, `ToastItem`, `ToastContainer`
  
- **Enums**: PascalCase
  - Example: `ToastType`, `ToastPosition`

- **Props**: snake_case
  - Example: `on_close`, `toast_type`, `auto_hide`

- **Modules**: snake_case (derived from component name)
  - Example: `mod toast;` (from `toast_default_tailwind`)

### Installing to Your Project

When you install a component, the CLI:

1. **Creates output directory** (e.g., `./src/components`)
2. **Copies component files** maintaining original structure
3. **Creates/updates `mod.rs`** with module declaration:
   ```rust
   pub mod toast;
   ```
4. **For rawcss**: Copies the CSS file alongside Rust files

### Naming in Your Project

After installation, you import and use components like this:

```rust
// Import the component module
use your_crate::components::toast;

// Use the component in RSX
rsx! {
    ToastContainer {
        toasts: vec![]
    }
}
```

The module name in `mod.rs` is the **snake_case** version of the component name:
- `toast_default_tailwind` → `mod toast;`
- `button_filled_rawcss` → `mod button;`
- `modal_minimal_tailwind` → `mod modal;`

## Configuration Options

### Registry URL

Specify a custom registry location:

```sh
reslt --registry-url /custom/path list
reslt --registry-url /custom/path add toast_default_tailwind
```

The registry path resolution works in this order:
1. If it's a URL (`http://` or `https://`), use as-is
2. If the path exists, use it directly
3. Try resolving relative to the CLI binary location
4. Fall back to the path as-is (may fail)

### Output Directory

Control where components are installed:

```sh
# Default: ./src/components
reslt add toast_default_tailwind

# Custom directory
reslt add toast_default_tailwind -o ./src/ui/components

# Relative to project root
reslt add toast_default_tailwind -o ./components
```

### Style Choice

Choose how styles are applied:

```sh
# Tailwind CSS (default)
reslt add toast_default_tailwind -s tailwind

# Raw CSS
reslt add toast_default_tailwind -s raw-css

# No styling (unstyled component)
reslt add toast_default_tailwind -s none
```

## Creating Components for the Registry

To add a new component to the registry:

1. **Create directory** following naming convention:
   ```sh
   mkdir mycomponent_default_tailwind
   ```

2. **Create `metadata.json`**:
   ```json
   {
     "name": "mycomponent",
     "variant": "default",
     "style": "tailwind",
     "version": "0.1.0",
     "description": "Brief description",
     "category": "ui",
     "files": ["component.rs", "mycomponent.rs"],
     "dependencies": {"dioxus": "0.7.1"},
     "requires_tailwind": true
   }
   ```

3. **Create implementation file** `mycomponent.rs`:
```rust
use dioxus::prelude::*;

#[component]
pub fn MyComponent() -> Element {
    rsx! {
        div { "My Component" }
    }
}
```

4. **Create entry point** `component.rs`:
   ```rust
   //! MyComponent - Brief description
   //!
   //! Usage:
   //! ```rust,no_run
   //! use dioxus::prelude::*;
   //!
   //! rsx! {
   //!     MyComponent {}
   //! }
   //! ```

   pub use mycomponent::MyComponent;
   ```

5. **For rawcss**: Create `mycomponent.css` and update metadata

6. **Test** the component before adding to registry

## Component Guidelines

### ✅ Do

- Create pure UI components with no internal state
- Use owned values for props (String, not &str)
- Provide proper documentation comments
- Include example usage in `component.rs`
- Use Dioxus 0.7 APIs (no `cx`, `Scope`, `use_state`)
- Make components composable and reusable
- Follow the naming conventions strictly

### ❌ Don't

- Use `use_signal` inside components (state is managed by consumer)
- Create context providers (leave that to consumer)
- Hardcode complex business logic
- Use old Dioxus APIs
- Create components that can't be customized
- Deviate from the directory/file naming convention

## Dependencies

- **Rust**: 1.70 or later
- **Dioxus**: 0.7.1 (for components)
- **CLI Dependencies**:
  - `clap` 4.5 - Command-line argument parsing
  - `tokio` 1.43 - Async runtime
  - `reqwest` 0.12 - HTTP client (for future remote registry)
  - `serde` 1.0 - Serialization
  - `dialoguer` 0.11 - Interactive prompts

## Troubleshooting

### Registry Not Found

```
Error: Registry path '../reslt-component/registry' does not exist
```

**Solution**: Ensure the component registry exists at the expected path, or specify a custom path:
```sh
reslt --registry-url /correct/path list
```

### Component Already Exists

```
Error: File 'toast.rs' already exists in output directory
```

**Solution**: Remove the existing file or use a different output directory:
```sh
reslt add toast_default_tailwind -o ./src/new_components
```

### Permission Denied

```
Error: Failed to create output directory
```

**Solution**: Ensure you have write permissions to the output directory.

## License

MIT

## Contributing

Contributions are welcome! Please ensure:
1. All components follow the naming conventions
2. Metadata is complete and accurate
3. Components are tested before submission
4. Documentation is clear and helpful

## Support

For issues, questions, or contributions, visit: https://github.com/lidm0707/reslt

## Related Projects

- [reslt-component](https://github.com/lidm0707/reslt/tree/main/reslt_component) - Component registry
- [reslt-core](https://github.com/lidm0707/reslt/tree/main/reslt_core) - Core utilities and hooks
- [Dioxus](https://dioxuslabs.com/) - The React-like framework for Rust