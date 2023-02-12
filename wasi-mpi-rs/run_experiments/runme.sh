#!/bin/bash

set -e

run_hpcg_benchmark_native () {
    mpirun --allow-run-as-root --oversubscribe -np $1 ../native_binaries/hpcg/xhpcg
    sleep 1
}


run_hpcg_benchmark_wasm () {
    mpirun --allow-run-as-root --oversubscribe -np $1 ../target/release/embedder ../examples/xhpcg.wasm >& $2
    sleep 1
}

run_imb_benchmark_native () {
    mpirun --allow-run-as-root --oversubscribe -np $1 ../native_binaries/IMB/IMB-MPI1 -npmin $1 Sendrecv Allreduce Reduce Allgather Gather Scatter Alltoall Bcast 1>$2 2>error
    sleep 1
}

run_imb_benchmark_wasm () {
    mpirun --allow-run-as-root --oversubscribe -np $1 ../target/release/embedder ../examples/imb.wasm -npmin $1 Sendrecv Allreduce Reduce Allgather Gather Scatter Alltoall Bcast 1>$2 2>error
    sleep 1
}

run_is_benchmark_native () {
    mpirun --allow-run-as-root --oversubscribe -np $1 ../native_binaries/NPB/IS/is.C.x 1>$2 2>error
    sleep 1
}

run_is_benchmark_wasm () {
    mpirun --allow-run-as-root --oversubscribe -np $1 ../target/release/embedder ../examples/is.C.x.wasm 1>$2 2>error
    sleep 1
}

move_hpcg_data () {
    mv $1 $2
    rm -rf hpcg*
}

move_imb_data () {
    mv $1 $2
    rm -rf error*
}


#Run small-scale experiments inside the docker container
mkdir -p experiment_data
mkdir -p experiment_data/native experiment_data/wasm
mkdir -p experiment_data/native/HPCG experiment_data/native/IS experiment_data/native/IMB
mkdir -p experiment_data/native/HPCG/4_proc experiment_data/native/HPCG/8_proc experiment_data/native/HPCG/16_proc
mkdir -p experiment_data/native/IS/4_proc experiment_data/native/IS/8_proc experiment_data/native/IS/16_proc
mkdir -p experiment_data/native/IMB/4_proc experiment_data/native/IMB/8_proc  

mkdir -p experiment_data/wasm/HPCG experiment_data/wasm/IS experiment_data/wasm/IMB
mkdir -p experiment_data/wasm/HPCG/4_proc experiment_data/wasm/HPCG/8_proc experiment_data/wasm/HPCG/16_proc
mkdir -p experiment_data/wasm/IS/4_proc experiment_data/wasm/IS/8_proc experiment_data/wasm/IS/16_proc
mkdir -p experiment_data/wasm/IMB/4_proc experiment_data/wasm/IMB/8_proc 

#1. Run HPCG, 4 processes, 8 processes, and 16 processes

#native
# mpirun --allow-run-as-root --oversubscribe -np 2 ../native_binaries/hpcg/xhpcg
echo "Running HPCG Native"

run_hpcg_benchmark_native 4
move_hpcg_data HPCG* experiment_data/native/HPCG/4_proc

run_hpcg_benchmark_native 4
move_hpcg_data HPCG* experiment_data/native/HPCG/4_proc

run_hpcg_benchmark_native 8
move_hpcg_data HPCG* experiment_data/native/HPCG/8_proc

run_hpcg_benchmark_native 8
move_hpcg_data HPCG* experiment_data/native/HPCG/8_proc

run_hpcg_benchmark_native 16
move_hpcg_data HPCG* experiment_data/native/HPCG/16_proc

run_hpcg_benchmark_native 16 
move_hpcg_data HPCG* experiment_data/native/HPCG/16_proc

#wasm
echo "Running HPCG Wasm"


run_hpcg_benchmark_wasm 4 hpcg_run_results_4_proc.out
move_hpcg_data hpcg_run_results_4_proc.out experiment_data/wasm/HPCG/4_proc

run_hpcg_benchmark_wasm 4 hpcg_run_results_4_proc_1.out
move_hpcg_data hpcg_run_results_4_proc_1.out experiment_data/wasm/HPCG/4_proc

run_hpcg_benchmark_wasm 8 hpcg_run_results_8_proc.out
move_hpcg_data hpcg_run_results_8_proc.out experiment_data/wasm/HPCG/8_proc

run_hpcg_benchmark_wasm 8 hpcg_run_results_8_proc_1.out
move_hpcg_data hpcg_run_results_8_proc_1.out experiment_data/wasm/HPCG/8_proc

run_hpcg_benchmark_wasm 16 hpcg_run_results_16_proc.out
move_hpcg_data hpcg_run_results_16_proc.out experiment_data/wasm/HPCG/16_proc

run_hpcg_benchmark_wasm 16 hpcg_run_results_16_proc_1.out
move_hpcg_data hpcg_run_results_16_proc_1.out experiment_data/wasm/HPCG/16_proc

#2. Run IMB, 4 proceeses, and 8 processes
#Native
echo "Running IMB Native"


run_imb_benchmark_native 4 imb_run_results_4_procs.out
move_imb_data imb_run_results_4_procs.out experiment_data/native/IMB/4_proc 

run_imb_benchmark_native 8 imb_run_results_8_procs.out
move_imb_data imb_run_results_8_procs.out experiment_data/native/IMB/8_proc 


#Wasm
echo "Running IMB Wasm"


run_imb_benchmark_wasm 4 imb_run_results_4_procs.out
move_imb_data imb_run_results_4_procs.out experiment_data/wasm/IMB/4_proc

run_imb_benchmark_wasm 8 imb_run_results_8_procs.out
move_imb_data imb_run_results_8_procs.out experiment_data/wasm/IMB/8_proc



#3 Run IS, 2 processes, 4 processes, and 8 processes

#Native 
echo "Running IS Native"


run_is_benchmark_native 4 is_run_results_4_procs.out
move_imb_data is_run_results_4_procs.out experiment_data/native/IS/4_proc

run_is_benchmark_native 4 is_run_results_4_procs_1.out
move_imb_data is_run_results_4_procs_1.out experiment_data/native/IS/4_proc

run_is_benchmark_native 8 is_run_results_8_procs.out
move_imb_data is_run_results_8_procs.out experiment_data/native/IS/8_proc

run_is_benchmark_native 8 is_run_results_8_procs_1.out
move_imb_data is_run_results_8_procs_1.out experiment_data/native/IS/8_proc

run_is_benchmark_native 16 is_run_results_16_procs.out
move_imb_data is_run_results_16_procs.out experiment_data/native/IS/16_proc

run_is_benchmark_native 16 is_run_results_16_procs_1.out
move_imb_data is_run_results_16_procs_1.out experiment_data/native/IS/16_proc

#Wasm
echo "Running IS Wasm"


run_is_benchmark_wasm 4 is_run_results_4_procs_wasm.out
move_imb_data is_run_results_4_procs_wasm.out experiment_data/wasm/IS/4_proc


run_is_benchmark_wasm 4 is_run_results_4_procs_wasm_1.out
move_imb_data is_run_results_4_procs_wasm_1.out experiment_data/wasm/IS/4_proc

run_is_benchmark_wasm 8 is_run_results_8_procs_wasm.out
move_imb_data is_run_results_8_procs_wasm.out experiment_data/wasm/IS/8_proc

run_is_benchmark_wasm 8 is_run_results_8_procs_wasm_1.out
move_imb_data is_run_results_8_procs_wasm_1.out experiment_data/wasm/IS/8_proc


run_is_benchmark_wasm 16 is_run_results_16_procs_wasm.out
move_imb_data is_run_results_16_procs_wasm.out experiment_data/wasm/IS/16_proc

run_is_benchmark_wasm 16 is_run_results_16_procs_wasm_1.out
move_imb_data is_run_results_16_procs_wasm_1.out experiment_data/wasm/IS/16_proc

#Parse data
python3 ../Parsers/hpcg-parser/main.py -d experiment_data/native/HPCG/ -f native.csv
python3 ../Parsers/hpcg-parser/main.py -d experiment_data/wasm/HPCG -f wasm.csv

python3 ../Parsers/is-parser/main.py -d experiment_data/native/IS/ -f native.csv
python3 ../Parsers/is-parser/main.py -d experiment_data/wasm/IS/ -f wasm.csv


python3 ../Parsers/imb-parser/impi_parser/main.py ./experiment_data/native/IMB/4_proc/imb_run_results_4_procs.out
python3 ../Parsers/imb-parser/impi_parser/main.py ./experiment_data/native/IMB/8_proc/imb_run_results_8_procs.out
# python3 ../Parsers/imb-parser/impi_parser/main.py ./experiment_data/native/IMB/16_proc/imb_run_results_16_procs.out

python3 ../Parsers/imb-parser/impi_parser/main.py ./experiment_data/wasm/IMB/4_proc/imb_run_results_4_procs.out
python3 ../Parsers/imb-parser/impi_parser/main.py ./experiment_data/wasm/IMB/8_proc/imb_run_results_8_procs.out
# python3 ../Parsers/imb-parser/impi_parser/main.py ./experiment_data/wasm/IMB/16_proc/imb_run_results_16_procs_wasm.out

#Plot data
python3 ../Plots/main.py -d experiment_data