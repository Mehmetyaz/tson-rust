# TSON for Rust

TSON (Typed JSON) parser implementation in Rust.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tson-rust = "0.1.0"
```

## Usage

```rust
use tson_rust::{parse, stringify, TSON};
use serde_json::json;

fn main() {
    // Parse TSON string to Rust value
    let tson_str = r#"{
        name "TSON Example"
        value #42
        pi =3.14159
        enabled ?true
        items [
            #1
            #2
            #3
        ]
    }"#;

    let parsed = parse(tson_str, None);
    println!("{:#?}", parsed);

    // Create a Rust value and stringify to TSON
    let value = json!({
        "name": "Rust Example",
        "numbers": [1, 2, 3],
        "nested": {
            "a": true,
            "b": 3.14
        }
    });

    let tson_string = stringify(&value, true); // Pretty print
    println!("{}", tson_string);
}
```

## Features

- Parse TSON strings into Rust values
- Stringify Rust values to TSON format
- Support for all TSON data types:
  - Objects `{}`
  - Arrays `[]`
  - Strings `"text"`
  - Integers `#123`
  - Floats `=3.14`
  - Booleans `?true`, `?false`
  - Array type specifiers `<type>`

## License

This project is licensed under the MIT License - see the LICENSE file for details.
