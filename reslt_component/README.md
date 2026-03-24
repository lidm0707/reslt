# reslt-component Registry

A registry of UI components for Dioxus 0.7 that can be downloaded and integrated into your projects via `reslt-cli`.

## Overview

The reslt-component registry provides pre-built, styled UI components that follow Dioxus 0.7 best practices. All components are:

- **Pure UI components** - No internal state management
- **Composable** - Easy to mix and match
- **Dioxus 0.7 compatible** - Uses latest APIs and conventions
- **Style-variant separated** - Each component has Tailwind and raw CSS variants

## Component Structure

Each component in the registry follows this naming convention:

```
<name>-<variant>-<style>/
├── metadata.json          # Component metadata and configuration
├── component.rs          # Main component with stylesheet import (for raw CSS)
├── <name>.rs            # Component implementation
└── <name>.css           # CSS styles (only for raw CSS variants)
```

### Naming Convention

- **name**: The component name (e.g., `toast`)
- **variant**: The style variant (e.g., `default`)
- **style**: The styling approach - either `tailwind` or `rawcss`

Examples:
- `toast-default-tailwind/` - Toast component with Tailwind CSS classes
- `toast-default-rawcss/` - Toast component with raw CSS styles

## Using Components

### 1. Install reslt-cli

```sh
cargo install reslt-cli
```

### 2. List Available Components

```sh
reslt list
```

### 3. Download a Component

```bash
reslt add toast-default-tailwind
```

This will download the component files into your project.

### 4. Integrate the Component

For **Tailwind** variants, just use the component directly:

```rust
use dioxus::prelude::*;

// In your app
rsx! {
    ToastContainer {
        toasts: vec![]
    }
}
```

For **raw CSS** variants, include the stylesheet in your app's root component:

```rust
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        ToastStylesheet {}  // Import the component's CSS
        // Rest of your app
    }
}
```

## State Management

Components in this registry are **pure UI components** with no internal state. State management should be handled by your application using:

1. **Dioxus signals** (`use_signal`, `use_memo`) for local state
2. **reslt-core hooks** for complex state management

Example with signals:

```rust
#[component]
fn MyComponent() -> Element {
    let mut is_checked = use_signal(|| false);
    
    rsx! {
        Checkbox {
            checked: is_checked(),
            onchange: move |value| is_checked.set(value),
            label: "Accept terms".to_string(),
        }
    }
}
```

Example with reslt-core:

```rust
use reslt_core::prelude::*;

#[component]
fn MyComponent() -> Element {
    let toast = use_toast();
    
    rsx! {
        button {
            onclick: move |_| toast.success("Operation completed!"),
            "Show Toast"
        }
    }
}
```

## Available Components

### Toast

Toast notification component with success, error, warning, and info variants.

**Tailwind Variant**: `toast-default-tailwind`  
**Raw CSS Variant**: `toast-default-rawcss`

Components:
- `ToastContainer` - Container for multiple toast notifications
- `ToastItem` - Individual toast notification
- `ToastType` - Enum for toast types (Success, Error, Warning, Info)
- `Toast` - Data structure for toast notifications

Example:

```rust
rsx! {
    ToastContainer {
        toasts: vec![
            rsx! {
                ToastItem {
                    toast: Toast {
                        id: 1,
                        message: "Success!".to_string(),
                        toast_type: ToastType::Success,
                    },
                    on_close: |_| {},
                }
            }
        ]
    }
}
```

## Adding New Components

When adding a new component to the registry:

1. **Create directory** following naming convention: `<name>-<variant>-<style>/`
2. **Create `metadata.json`** with component information:

```json
{
  "name": "mycomponent",
  "variant": "default",
  "style": "tailwind",
  "version": "0.1.0",
  "description": "A brief description",
  "category": "ui",
  "files": [
    "component.rs",
    "mycomponent.rs"
  ],
  "dependencies": {
    "dioxus": "0.7.1"
  },
  "requires_tailwind": true
}
```

3. **Create component implementation** in `<name>.rs`
4. **If raw CSS variant**: Create `<name>.css` and a `component.rs` with stylesheet import
5. **Test** the component before committing

## Component Guidelines

### Do:
- ✅ Create pure UI components with no internal state
- ✅ Use owned values for props (String, not &str)
- ✅ Provide proper documentation comments
- ✅ Include example usage in component.rs
- ✅ Use Dioxus 0.7 APIs (no `cx`, `Scope`, `use_state`)
- ✅ Make components composable and reusable

### Don't:
- ❌ Use `use_signal` inside components (state is managed by consumer)
- ❌ Create context providers (leave that to consumer)
- ❌ Hardcode complex business logic
- ❌ Use old Dioxus APIs
- ❌ Create components that can't be customized

## Dependencies

- **Dioxus**: 0.7.1
- **reslt-core**: For state management hooks (optional, for advanced features)

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please follow the component guidelines when adding new components.

## Support

For issues or questions, please visit: https://github.com/lidm0707/reslt