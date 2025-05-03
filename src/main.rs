use serde_json::json;
use tson_rust::TsonValue;

mod bnc;

fn main() {
    let tson_str = r#"{name"TSON Example" value#42 pi=3.14159 enabled?true items[#1 #2 #3]}"#;

    println!("Original TSON string:");
    println!("{}", tson_str);
    println!();

    // Parse TSON to a Rust value
    let parsed = tson_rust::parse_third(tson_str);
    println!("Parsed as Rust value:");
    println!("{:#?}", parsed);
    println!();

    // Create a Rust value and stringify to TSON
    let value = json!({
        "string": "Hello, World!",
        "number": 42,
        "float": std::f64::consts::PI,
        "boolean": true,
        "null": null,
        "array": [1, "string", true, null, 3.14],
        "object": {
            "a": 1,
            "b": "string",
            "c": true
        }
    });

    // Convert value to TsonValue
    let tson_value: TsonValue = value.clone().into();

    println!("Stringified TSON (compact):");
    println!("{}", tson_rust::stringify(&tson_value, false));
    println!();

    println!("Stringified TSON (pretty):");
    println!("{}", tson_rust::stringify(&tson_value, true));
    println!();

    // Using the TSON struct
    let parsed_via_struct = tson_rust::parse(&tson_rust::stringify(&tson_value, false));
    println!("Parsed via TSON struct:");
    println!("{:#?}", parsed_via_struct);

    // Simple benchmark comparison between JSON and TSON
    println!("\n=== Simple Performance Comparison ===");
    bnc::run_simple_benchmark();
}
