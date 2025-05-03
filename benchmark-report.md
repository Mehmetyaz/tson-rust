# JSON vs TSON Benchmark Results

This benchmark compares the performance and size characteristics of JSON and TSON formats across different data structures and operations.

## Size Comparison

| Data Structure | JSON Size (bytes) | TSON Size (bytes) | Size Ratio (TSON/JSON) |
| -------------- | ----------------- | ----------------- | ---------------------- |
| Simple Object  | 67                | 61                | 0.91                   |
| Nested Object  | 152               | 136               | 0.89                   |
| Array Heavy    | 122               | 165               | 1.35                   |
| Deep Nesting   | 100               | 85                | 0.85                   |
| Large Dataset  | 5368              | 4861              | 0.91                   |
| Mixed Types    | 156               | 146               | 0.94                   |

TSON generally produces smaller output sizes compared to JSON, with an average reduction of approximately 9-15% for most structures. The notable exception is array-heavy data, where TSON's size is about 35% larger than JSON.

## Parsing Performance

| Data Structure | JSON Parse Time | TSON Parse Time | Performance Ratio (TSON/JSON) |
| -------------- | --------------- | --------------- | ----------------------------- |
| Simple Object  | 660.30 ns       | 879.62 ns       | 1.33                          |
| Nested Object  | 1.8502 µs       | 2.1750 µs       | 1.18                          |
| Array Heavy    | 2.4964 µs       | 2.1307 µs       | 0.85                          |
| Deep Nesting   | 1.3754 µs       | 1.6456 µs       | 1.20                          |
| Large Dataset  | 68.019 µs       | 74.482 µs       | 1.10                          |
| Mixed Types    | 1.9672 µs       | 2.3613 µs       | 1.20                          |

In most cases, JSON parsing is 10-33% faster than TSON parsing. However, for array-heavy data structures, TSON parsing outperforms JSON by approximately 15%.

## Stringifying Performance

| Data Structure | JSON Stringify Time | TSON Stringify Time | Performance Ratio (TSON/JSON) |
| -------------- | ------------------- | ------------------- | ----------------------------- |
| Simple Object  | 190.24 ns           | 1.5349 µs           | 8.07                          |
| Nested Object  | 384.01 ns           | 7.2189 µs           | 18.80                         |
| Array Heavy    | 407.99 ns           | 8.5040 µs           | 20.85                         |
| Deep Nesting   | 270.09 ns           | 3.2245 µs           | 11.94                         |
| Large Dataset  | 10.915 µs           | 153.56 µs           | 14.07                         |
| Mixed Types    | 466.31 ns           | 4.0997 µs           | 8.79                          |

JSON stringification is significantly faster than TSON across all data structures, with TSON being 8-21 times slower depending on the data structure.

## Summary

- **Size**: TSON generally produces more compact representations (5-15% smaller) except for array-heavy data.
- **Parsing**: JSON parsing is typically 10-33% faster than TSON, except for array-heavy structures where TSON is ~15% faster.
- **Stringifying**: JSON stringification is considerably faster than TSON (8-21 times faster).

These benchmarks used Criterion.rs with multiple samples to ensure statistical validity.

## Test Data Structures

1. **Simple Object**: Basic object with primitive values
2. **Nested Object**: Object with nested objects and arrays
3. **Array Heavy**: Data with many nested arrays
4. **Deep Nesting**: Deeply nested object structure
5. **Large Dataset**: Large collection with 100 items
6. **Mixed Types**: Object with various data types

## Conclusions

TSON offers size advantages over JSON in most cases, which could be beneficial for data transmission and storage. However, the current implementation has performance trade-offs, particularly in stringification. The choice between JSON and TSON should be based on specific application requirements:

- If minimizing data size is critical and parsing/stringifying performance is less important, TSON may be appropriate.
- If stringification performance is crucial, JSON is clearly the better choice with current implementations.
- For array-heavy data structures, the trade-offs are more nuanced, with TSON offering better parsing performance but larger data size.
