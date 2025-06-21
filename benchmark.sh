#!/bin/bash
set -e

# Create results directory
mkdir -p /results

# Run the benchmark
echo "Running benchmarks..."
cargo bench --bench json_vs_tson

# Copy the criterion results directory
echo "Copying benchmark results to /results directory..."
cp -r target/criterion /results/

echo "Benchmark completed successfully"