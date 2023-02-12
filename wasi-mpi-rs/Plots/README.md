# Plotting helper
This script plots the results obtained from the execution of the differenet benchmarks using the generated structured csv files from the parsers.

# Usage

```bash
python3 Plots/main.py -d <path-to-data-dir> 
```
The data directory should be structured as follows:
```bash
─ experiment_data
│   ├── native
│   │   ├── HPCG
│   │   ├── IMB
│   │   └── IS
│   └── wasm
│       ├── HPCG
│       ├── IMB
│       └── IS
```
The plotter will create plots for the different benchmarks.
```bash
Plots
│   ├── HPCG
│   │   ├── hpcg_bandwidth.png
│   │   └── hpcg_flops.png
│   ├── IMB
│   │   ├── 4_proc
│   │   │   ├── Allgather
│   │   │   │   └── Allgather.png
│   │   │   ├── Allreduce
│   │   │   │   └── Allreduce.png
│   │   │   ├── Alltoall
│   │   │   │   └── Alltoall.png
│   │   │   ├── Bcast
│   │   │   │   └── Bcast.png
│   │   │   ├── Gather
│   │   │   │   └── Gather.png
│   │   │   ├── Reduce
│   │   │   │   └── Reduce.png
│   │   │   ├── Scatter
│   │   │   │   └── Scatter.png
│   │   │   └── Sendrecv
│   │   │       └── Sendrecv.png
│   │   └── 8_proc
│   │       ├── Allgather
│   │       │   └── Allgather.png
│   │       ├── Allreduce
│   │       │   └── Allreduce.png
│   │       ├── Alltoall
│   │       │   └── Alltoall.png
│   │       ├── Bcast
│   │       │   └── Bcast.png
│   │       ├── Gather
│   │       │   └── Gather.png
│   │       ├── Reduce
│   │       │   └── Reduce.png
│   │       ├── Scatter
│   │       │   └── Scatter.png
│   │       └── Sendrecv
│   │           └── Sendrecv.png
│   └── IS
│       └── is_mops.png
```