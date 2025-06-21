# TSON Docker Benchmarks

Docker ile TSON benchmark çalıştırma aracı.

## Kullanım

1. Docker ve Docker Compose kurun

2. Benchmark'ları çalıştırmak için:

```bash
docker-compose up --build
```

veya PowerShell script ile:

```powershell
.\run-benchmark.ps1
```

3. Benchmark sonuçları:
   - `./benchmark_results/criterion` - Docker konteynerinden alınan Criterion sonuçları
   - `%USERPROFILE%\Documents\tson-benchmarks\benchmark_YYYY-MM-DD_HH-MM-SS` - Windows'ta kopyalanan sonuçlar
