extern crate criterion;
extern crate serde_json;
extern crate tson_rust;

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use serde_json::json;

// Type-specific test cases
fn strings_only() -> serde_json::Value {
    json!({
        "name": "String Test",
        "description": "Object with strings only",
        "category": "test",
        "tag": "string-sample",
        "message": "Hello, World!"
    })
}

fn numbers_only() -> serde_json::Value {
    json!({
        "int1": 42,
        "int2": -100,
        "int3": 1000000,
        "int4": -5000
    })
}

fn floats_only() -> serde_json::Value {
    json!({
        "pi": 3.14159,
        "e": 2.71828,
        "sqrt2": 1.41421,
        "phi": 1.61803,
        "negFloat": -3.14159
    })
}

fn booleans_only() -> serde_json::Value {
    json!({
        "isTrue": true,
        "isFalse": false,
        "enabled": true,
        "visible": false,
        "active": true
    })
}

fn arrays_only() -> serde_json::Value {
    json!({
        "emptyArray": [],
        "numbers": [1, 2, 3, 4, 5],
        "strings": ["one", "two", "three", "four", "five"],
        "mixed": [1, "two", 3, "four", 5]
    })
}

fn nested_objects() -> serde_json::Value {
    json!({
        "user": {
            "profile": {
                "details": {
                    "address": {
                        "city": "Example City",
                        "zipCode": 12345
                    }
                }
            }
        }
    })
}

fn mixed_types() -> serde_json::Value {
    json!({
        "string": "Hello, World!",
        "number": 42,
        "float": 3.14159,
        "boolean": true,
        "null": null,
        "array": [1, "string", true, null, 3.14],
        "object": {
            "a": 1,
            "b": "string",
            "c": true
        }
    })
}

fn benchmark_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parsing");

    // Test cases
    let test_cases = [
        ("strings", strings_only()),
        ("numbers", numbers_only()),
        ("floats", floats_only()),
        ("booleans", booleans_only()),
        ("arrays", arrays_only()),
        ("nested", nested_objects()),
        ("mixed", mixed_types()),
    ];

    for (name, value) in test_cases.iter() {
        let json_str = serde_json::to_string(&value).unwrap();
        let tson_str = tson_rust::stringify(&value.clone().into(), false);

        // Benchmark JSON parsing
        group.bench_with_input(
            BenchmarkId::new("JSON_parse", name),
            &json_str,
            |b, json_s| {
                b.iter(|| {
                    let _: serde_json::Value = serde_json::from_str(black_box(&json_s)).unwrap();
                });
            },
        );

        // Benchmark TSON parsing with original parser
        group.bench_with_input(
            BenchmarkId::new("TSON_parse", name),
            &tson_str,
            |b, tson_s| {
                b.iter(|| {
                    let _ = tson_rust::parse(black_box(&tson_s));
                });
            },
        );

        // Benchmark TSON parsing with new parser
        group.bench_with_input(
            BenchmarkId::new("TSON_parse_new", name),
            &tson_str,
            |b, tson_s| {
                b.iter(|| {
                    let _ = tson_rust::parse_new(black_box(&tson_s));
                });
            },
        );

        // Benchmark TSON parsing with third parser
        group.bench_with_input(
            BenchmarkId::new("TSON_parse_third", name),
            &tson_str,
            |b, tson_s| {
                b.iter(|| {
                    let _ = tson_rust::parse_third(black_box(&tson_s));
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_parsing);
criterion_main!(benches);
