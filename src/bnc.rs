use serde_json::json;
use std::time::{Duration, Instant};
use tson_rust::TsonValue;

extern crate serde_json;
extern crate tson_rust;

#[allow(dead_code)]
pub fn run_simple_benchmark() {
    println!("\n=== Detailed Performance Profiling ===\n");

    // Sample data for benchmarking
    let start_data_creation = Instant::now();
    let json_data = json!([1, 2, 3, 4, 5, 6, 7,]);
    let data_creation_time = start_data_creation.elapsed();
    println!("Data creation time: {:?}", data_creation_time);

    // Convert to JSON and TSON strings
    let start_json_stringify = Instant::now();
    let json_str = serde_json::to_string(&json_data).unwrap();
    let json_stringify_time = start_json_stringify.elapsed();
    println!("JSON stringify time: {:?}", json_stringify_time);

    let start_tson_conversion = Instant::now();
    let tson_value: TsonValue = json_data.clone().into();
    let tson_conversion_time = start_tson_conversion.elapsed();
    println!("JSON to TSON conversion time: {:?}", tson_conversion_time);

    let start_tson_stringify = Instant::now();
    let tson_str = tson_rust::stringify(&tson_value, false);
    let tson_stringify_time = start_tson_stringify.elapsed();
    println!("TSON stringify time: {:?}", tson_stringify_time);

    println!("\nJSON string:");
    println!("{}", json_str);
    println!();

    println!("TSON string:");
    println!("{}", tson_str);
    println!();

    // Test parse performance for TSON v1
    let start_tson_parse = Instant::now();
    let parsed = tson_rust::parse(&tson_str);
    let tson_v1_single_parse_time = start_tson_parse.elapsed();
    println!("Single TSON v1 parse time: {:?}", tson_v1_single_parse_time);

    // Test parse performance for TSON v2
    let start_tson_v2_parse = Instant::now();
    let parsed_v2 = tson_rust::parse_new(&tson_str);
    let tson_v2_single_parse_time = start_tson_v2_parse.elapsed();
    println!("Single TSON v2 parse time: {:?}", tson_v2_single_parse_time);

    println!("Parsed TSON v1:");
    println!("{:#?}", parsed);
    println!();

    println!("Parsed TSON v2:");
    println!("{:#?}", parsed_v2);
    println!();

    // Print string sizes for comparison
    println!("JSON string size: {} bytes", json_str.len());
    println!("TSON string size: {} bytes", tson_str.len());
    println!();

    // Number of iterations for more accurate measurements
    const ITERATIONS: u32 = 1000;
    println!("\nRunning benchmark with {} iterations...", ITERATIONS);

    // Benchmark TSON v1 parsing with detailed breakdown
    let start_tson_v1_parse_bench = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = tson_rust::parse(&tson_str).unwrap();
    }
    let tson_v1_parse_time = start_tson_v1_parse_bench.elapsed();
    println!(
        "TSON v1 parsing benchmark completed in: {:?}",
        tson_v1_parse_time
    );

    // Benchmark TSON v2 parsing with detailed breakdown
    let start_tson_v2_parse_bench = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = tson_rust::parse_new(&tson_str).unwrap();
    }
    let tson_v2_parse_time = start_tson_v2_parse_bench.elapsed();
    println!(
        "TSON v2 parsing benchmark completed in: {:?}",
        tson_v2_parse_time
    );

    // Benchmark TSON v3 parsing with detailed breakdown
    let start_tson_v3_parse_bench = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = tson_rust::parse_third(&tson_str).unwrap();
    }
    let tson_v3_parse_time = start_tson_v3_parse_bench.elapsed();
    println!(
        "TSON v3 parsing benchmark completed in: {:?}",
        tson_v3_parse_time
    );

    // Benchmark JSON parsing with detailed breakdown
    let start_json_parse_bench = Instant::now();
    for _ in 0..ITERATIONS {
        let _: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    }
    let json_parse_time = start_json_parse_bench.elapsed();
    println!("JSON parsing benchmark completed in: {:?}", json_parse_time);

    // Benchmark JSON stringifying
    let start_json_stringify_bench = Instant::now();
    for i in 0..ITERATIONS {
        let _ = serde_json::to_string(&json_data).unwrap();
        // Print progress every 200 iterations
        if i > 0 && i % 200 == 0 {
            println!("  JSON stringifying: {} iterations completed", i);
        }
    }
    let json_stringify_bench_time = start_json_stringify_bench.elapsed();
    println!(
        "JSON stringifying benchmark completed in: {:?}",
        json_stringify_bench_time
    );

    // Benchmark TSON stringifying
    let start_tson_stringify_bench = Instant::now();
    for i in 0..ITERATIONS {
        let _ = tson_rust::stringify(&tson_value, false);
        // Print progress every 200 iterations
        if i > 0 && i % 200 == 0 {
            println!("  TSON stringifying: {} iterations completed", i);
        }
    }
    let tson_stringify_bench_time = start_tson_stringify_bench.elapsed();
    println!(
        "TSON stringifying benchmark completed in: {:?}",
        tson_stringify_bench_time
    );

    // Print detailed results
    println!(
        "\n=== Benchmark Results for {} iterations ===\n",
        ITERATIONS
    );

    // Parsing results
    println!("Parsing Performance:");
    println!(
        "  JSON parse time:      {:?} ({:.2} ns/iter)",
        json_parse_time,
        duration_to_ns(json_parse_time) / ITERATIONS as f64
    );
    println!(
        "  TSON v1 parse time:   {:?} ({:.2} ns/iter)",
        tson_v1_parse_time,
        duration_to_ns(tson_v1_parse_time) / ITERATIONS as f64
    );
    println!(
        "  TSON v2 parse time:   {:?} ({:.2} ns/iter)",
        tson_v2_parse_time,
        duration_to_ns(tson_v2_parse_time) / ITERATIONS as f64
    );
    println!(
        "  TSON v3 parse time:   {:?} ({:.2} ns/iter)",
        tson_v3_parse_time,
        duration_to_ns(tson_v3_parse_time) / ITERATIONS as f64
    );
    println!(
        "  Parse ratio (TSON v1/JSON): {:.2} (lower is better for TSON)",
        duration_to_ns(tson_v1_parse_time) / duration_to_ns(json_parse_time)
    );
    println!(
        "  Parse ratio (TSON v2/JSON): {:.2} (lower is better for TSON)",
        duration_to_ns(tson_v2_parse_time) / duration_to_ns(json_parse_time)
    );
    println!(
        "  Parse ratio (TSON v3/JSON): {:.2} (lower is better for TSON)",
        duration_to_ns(tson_v3_parse_time) / duration_to_ns(json_parse_time)
    );

    // Stringifying results
    println!("\nStringifying Performance:");
    println!(
        "  JSON stringify time:  {:?} ({:.2} ns/iter)",
        json_stringify_bench_time,
        duration_to_ns(json_stringify_bench_time) / ITERATIONS as f64
    );
    println!(
        "  TSON stringify time:  {:?} ({:.2} ns/iter)",
        tson_stringify_bench_time,
        duration_to_ns(tson_stringify_bench_time) / ITERATIONS as f64
    );
    println!(
        "  Stringify ratio (TSON/JSON): {:.2} (lower is better for TSON)",
        duration_to_ns(tson_stringify_bench_time) / duration_to_ns(json_stringify_bench_time)
    );

    // Overall performance summary
    println!("\nPerformance Summary:");
    println!(
        "  - TSON v1 parsing is {:.2}x {} than JSON parsing",
        if duration_to_ns(tson_v1_parse_time) > duration_to_ns(json_parse_time) {
            duration_to_ns(tson_v1_parse_time) / duration_to_ns(json_parse_time)
        } else {
            duration_to_ns(json_parse_time) / duration_to_ns(tson_v1_parse_time)
        },
        if duration_to_ns(tson_v1_parse_time) > duration_to_ns(json_parse_time) {
            "slower"
        } else {
            "faster"
        }
    );
    println!(
        "  - TSON v2 parsing is {:.2}x {} than JSON parsing",
        if duration_to_ns(tson_v2_parse_time) > duration_to_ns(json_parse_time) {
            duration_to_ns(tson_v2_parse_time) / duration_to_ns(json_parse_time)
        } else {
            duration_to_ns(json_parse_time) / duration_to_ns(tson_v2_parse_time)
        },
        if duration_to_ns(tson_v2_parse_time) > duration_to_ns(json_parse_time) {
            "slower"
        } else {
            "faster"
        }
    );

    println!(
        "  - TSON v3 parsing is {:.2}x {} than JSON parsing",
        if duration_to_ns(tson_v3_parse_time) > duration_to_ns(json_parse_time) {
            duration_to_ns(tson_v3_parse_time) / duration_to_ns(json_parse_time)
        } else {
            duration_to_ns(json_parse_time) / duration_to_ns(tson_v3_parse_time)
        },
        if duration_to_ns(tson_v3_parse_time) > duration_to_ns(json_parse_time) {
            "slower"
        } else {
            "faster"
        }
    );

    println!(
        "  - TSON stringifying is {:.2}x {} than JSON stringifying",
        if duration_to_ns(tson_stringify_bench_time) > duration_to_ns(json_stringify_bench_time) {
            duration_to_ns(tson_stringify_bench_time) / duration_to_ns(json_stringify_bench_time)
        } else {
            duration_to_ns(json_stringify_bench_time) / duration_to_ns(tson_stringify_bench_time)
        },
        if duration_to_ns(tson_stringify_bench_time) > duration_to_ns(json_stringify_bench_time) {
            "slower"
        } else {
            "faster"
        }
    );
}

// Helper function to convert Duration to nanoseconds as f64
fn duration_to_ns(duration: Duration) -> f64 {
    duration.as_secs() as f64 * 1e9 + duration.subsec_nanos() as f64
}
