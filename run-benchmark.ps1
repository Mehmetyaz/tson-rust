# PowerShell script to run TSON benchmarks in Docker

# Create benchmark_results directory if it doesn't exist
if (!(Test-Path -Path "benchmark_results")) {
    New-Item -ItemType Directory -Path "benchmark_results" | Out-Null
}

# Run the Docker container
Write-Host "Building and running the benchmark container..." -ForegroundColor Yellow
docker-compose up --build

# Copy results to Documents folder
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$destinationFolder = "$env:USERPROFILE\Documents\tson-benchmarks\benchmark_$timestamp"

if (!(Test-Path -Path "$env:USERPROFILE\Documents\tson-benchmarks")) {
    New-Item -ItemType Directory -Path "$env:USERPROFILE\Documents\tson-benchmarks" | Out-Null
}

New-Item -ItemType Directory -Path $destinationFolder | Out-Null
Copy-Item -Path ".\benchmark_results\criterion" -Destination $destinationFolder -Recurse

Write-Host "Benchmark completed successfully!" -ForegroundColor Green
Write-Host "Results are available at: $destinationFolder" -ForegroundColor Green

# Open the results folder
Invoke-Item $destinationFolder 