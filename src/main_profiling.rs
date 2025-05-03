#![feature(portable_simd)]
use serde_json::json;
use std::time::{Duration, Instant};
use tson_rust::TsonValue;
use std::fs::File;

extern crate serde_json;
extern crate tson_rust;
extern crate pprof;

use pprof::protos::Message;
use pprof::{ProfilerGuard, ProfilerGuardBuilder};

// Helper function to create a profiler guard
fn create_profiler() -> ProfilerGuard<'static> {
    ProfilerGuardBuilder::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap()
}

fn main() {
    // Start profiling
    let guard = create_profiler();
    
    println!("Starting profiling of TSON operations...");
    
    // Run the benchmark with profiling
    profile_tson_operations();
    
    // Save profiling data
    println!("\nSaving profiling data...");
    if let Err(err) = save_profile(guard, "tson_profile") {
        println!("Failed to save profile: {}", err);
    }
    
    println!("Profiling completed. Check tson_profile.svg for flamegraph visualization.");
}

fn profile_tson_operations() {
    // Create test data
    let json_data = json!({
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
    
    // Convert to TSON
    let tson_value: TsonValue = json_data.clone().into();
    
    // Stringify to TSON
    let tson_str = tson_rust::stringify(&tson_value, false);
    
    // Parse TSON
    let parsed = tson_rust::parse(&tson_str);
    
    // Run multiple iterations for better profiling data
    const ITERATIONS: u32 = 10000;
    println!("Running {} iterations of TSON operations for profiling...", ITERATIONS);
    
    // Profile TSON parsing
    println!("Profiling TSON parsing...");
    for _ in 0..ITERATIONS {
        let _ = tson_rust::parse(&tson_str);
    }
    
    // Profile JSON to TSON conversion
    println!("Profiling JSON to TSON conversion...");
    for _ in 0..ITERATIONS {
        let _: TsonValue = json_data.clone().into();
    }
    
    // Profile TSON stringification
    println!("Profiling TSON stringification...");
    for _ in 0..ITERATIONS {
        let _ = tson_rust::stringify(&tson_value, false);
    }
    
    // Profile JSON parsing for comparison
    println!("Profiling JSON parsing for comparison...");
    let json_str = serde_json::to_string(&json_data).unwrap();
    for _ in 0..ITERATIONS {
        let _: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    }
    
    // Profile JSON stringification for comparison
    println!("Profiling JSON stringification for comparison...");
    for _ in 0..ITERATIONS {
        let _ = serde_json::to_string(&json_data);
    }
    
    println!("Profiling operations completed.");
}

// Save profiling data to a file
fn save_profile(guard: ProfilerGuard<'static>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let report = guard.report().build()?;
    
    // Save raw profile data
    let file = File::create(format!("{}.pb", name))?;
    let profile = report.pprof()?;
    let mut content = Vec::new();
    profile.encode(&mut content)?;
    std::fs::write(file, content)?;
    
    // Generate flamegraph
    report.flamegraph(format!("{}.svg", name))?;
    println!("Flamegraph saved to {}.svg", name);
    
    Ok(())
}
