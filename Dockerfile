FROM rust:latest

WORKDIR /app

# Copy project files
COPY . .

# Make the benchmark script executable
RUN chmod +x benchmark.sh

# Create a directory for the benchmark results
RUN mkdir -p /results

# Run the benchmark script
CMD ["./benchmark.sh"] 