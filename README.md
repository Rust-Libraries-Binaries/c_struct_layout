# c_struct_layout

A library for enforcing stable, C-compatible data layouts in Rust.

## Features

- Ensure consistent, C-compatible memory layouts for structs using `#[repr(C)]`.
- Provides a `check_layout` method to verify the layout at runtime (basic implementation).

## Installation

To use `c_struct_layout`, add the following to your `Cargo.toml`:

```toml
[dependencies]
c_struct_layout = "0.1.0"
```
## Usage
```rust
use c_struct_layout::CStructLayout;

#[derive(CStructLayout)]
#[repr(C)]
struct MyData {
    x: u32,
    y: f64,
}

fn main() {
    MyData::check_layout();
}
```
The check_layout method is a placeholder for verifying struct layouts. You can expand it with more detailed checks if needed.

## License
This project is licensed under the MIT License

## Author
Ben Santora <bensatlantik@gmail.com>

## Donate
https://bensatlantik.github.io/
