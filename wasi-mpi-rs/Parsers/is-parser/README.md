# IS Benchmarks Parser
This script converts the textual output from the IS MPI Benchmark to
structured CSV files that can be used programmatically (e.g. with pandas
and matplotlib).

## Usage

```bash
python3 is-parser/main.py -d <path-to-is-data-dir> -f <filename-to-save-data>
```
The IS data directory should be structured as follows:
```bash
IS
    ├── 16_proc
    │   ├── is_run_results_16_procs.out
    │   └── is_run_results_16_procs_1.out
    ├── 4_proc
    │   ├── is_run_results_4_procs.out
    │   └── is_run_results_4_procs_1.out
    ├── 8_proc
    │   ├── is_run_results_8_procs.out
    │   └── is_run_results_8_procs_1.out
```
The parser will create a file with the specified filename in the parent directory. For instance, with `-f native.csv`:
```bash
└── IS
    ├── 16_proc
    │   ├── is_run_results_16_procs.out
    │   └── is_run_results_16_procs_1.out
    ├── 4_proc
    │   ├── is_run_results_4_procs.out
    │   └── is_run_results_4_procs_1.out
    ├── 8_proc
    │   ├── is_run_results_8_procs.out
    │   └── is_run_results_8_procs_1.out
    └── native.csv
```