# HPCG Benchmarks Parser
This script converts the textual output from the HPCG MPI Benchmark to
structured CSV files that can be used programmatically (e.g. with pandas
and matplotlib).

## Usage

```bash
python3 hpcg-parser/main.py -d <path-to-hpcg-data-dir> -f <filename-to-save-data>
```
The HPCG data directory should be structured as follows:
```bash
├── HPCG
│   ├── 16_proc
│   │   ├── HPCG-Benchmark_3.1_2022-12-05_22-17-38.txt
│   │   └── HPCG-Benchmark_3.1_2022-12-05_22-17-41.txt
│   ├── 4_proc
│   │   ├── HPCG-Benchmark_3.1_2022-12-05_22-17-31.txt
│   │   └── HPCG-Benchmark_3.1_2022-12-05_22-17-32.txt
│   ├── 8_proc
│   │   ├── HPCG-Benchmark_3.1_2022-12-05_22-17-34.txt
│   │   └── HPCG-Benchmark_3.1_2022-12-05_22-17-35.txt
```
The parser will create a file with the specified filename in the parent directory. For instance, with `-f native.csv`:
```bash
├── HPCG
│   ├── 16_proc
│   │   ├── HPCG-Benchmark_3.1_2022-12-05_22-17-38.txt
│   │   └── HPCG-Benchmark_3.1_2022-12-05_22-17-41.txt
│   ├── 4_proc
│   │   ├── HPCG-Benchmark_3.1_2022-12-05_22-17-31.txt
│   │   └── HPCG-Benchmark_3.1_2022-12-05_22-17-32.txt
│   ├── 8_proc
│   │   ├── HPCG-Benchmark_3.1_2022-12-05_22-17-34.txt
│   │   └── HPCG-Benchmark_3.1_2022-12-05_22-17-35.txt
│   └── native.csv
```