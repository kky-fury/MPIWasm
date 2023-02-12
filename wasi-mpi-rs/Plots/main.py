import pandas as pd
import matplotlib.pyplot as plt
import os
import logging
import subprocess
import argparse

def shell(cmd, quiet=False):
    if not quiet:
        logging.debug(f'  shell: {cmd}')
    result = subprocess.run(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT).stdout
    try:
        result = result.decode().strip()
    except:
        pass
    if result and not quiet:
        logging.debug(f'    result: {result}')
    return result

def generate_plot_imb(datapath_native, datapath_wasm, path_to_save):
    for _, dirnames, filenames in os.walk(datapath_native):
        for dir in dirnames:
            for _, benchname, filenames in os.walk(os.path.join(datapath_native, dir)):
                for bench in benchname:
                    # for file in filenames:      
                    logging.info(f'Benchmark: {bench}')
                    for _, _, datafiles in os.walk(os.path.join(datapath_native, dir, bench)):
                        # print(datafiles[0])
                        df_imb_native = pd.read_csv(os.path.join(datapath_native, dir, bench, datafiles[0]), index_col=False)
                        df_imb_wasm = pd.read_csv(os.path.join(datapath_wasm, dir, bench, datafiles[0]), index_col=False)
                        create_dir_path = path_to_save + "/" + bench
                        # print(create_dir_path)
                        shell(f'mkdir -p {create_dir_path}')
                        # print(os.path.join(datapath_native, dir, bench, datafiles[0]))
                        # print(os.path.join(datapath_wasm, dir, bench, datafiles[0]))
                        
                        fig, ax = plt.subplots()
                        ax.plot(df_imb_native['bytes'], df_imb_native['t_avg_us'], label='Native')
                        ax.plot(df_imb_wasm['bytes'], df_imb_wasm['t_avg_us'], label='Wasm')
                        plt.xlabel('Bytes')
                        plt.ylabel('Iteration Time')
                        plt.title(f'{bench}')
                        plt.legend()
                        save_path = create_dir_path + "/" + bench + ".png"
                        # print(save_path)
                        plt.savefig(save_path, dpi=300) 


#Create plots for HPCG, IS, and IMB
def create_plots(dirpath):
    
    shell("mkdir -p Plots")
    shell("mkdir -p Plots/HPCG")
    shell("mkdir -p Plots/IS")
    shell("mkdir -p Plots/IMB")
    shell("mkdir -p Plots/IMB/4_proc")
    shell("mkdir -p Plots/IMB/8_proc")

    data_path_native = os.path.join(dirpath, "native")
    data_path_wasm= os.path.join(dirpath, "wasm")
    logging.info(f'Generating plots for HPCG')
   
    df_hpcg_native = pd.read_csv(os.path.join(data_path_native, "HPCG/native.csv"), index_col=False)
    df_hpcg_wasm = pd.read_csv(os.path.join(data_path_wasm, "HPCG/wasm.csv"), index_col=False)

    fig, ax = plt.subplots()
    ax.plot(df_hpcg_native['nproc'], df_hpcg_native['gflop_per_s'], label='Native')
    ax.plot(df_hpcg_wasm['nproc'], df_hpcg_wasm['gflop_per_s'], label='Wasm')
    plt.xlabel('Number of MPI Processes')
    plt.ylabel('Gflop/s')
    plt.legend()
    plt.savefig("Plots/HPCG/hpcg_flops.png", dpi=300)   
    
    fig, ax = plt.subplots()
    ax.plot(df_hpcg_native['nproc'], df_hpcg_native['gb_per_s'], label='Native')
    ax.plot(df_hpcg_wasm['nproc'], df_hpcg_wasm['gb_per_s'], label='Wasm')
    plt.xlabel('Number of MPI Processes')
    plt.ylabel('Bandwidth (GB/s)')
    plt.legend()
    plt.savefig("Plots/HPCG/hpcg_bandwidth.png", dpi=300)   

    logging.info(f'Generating plots for IS')

    df_is_native = pd.read_csv(os.path.join(data_path_native, "IS/native.csv"), index_col=False)
    df_is_wasm = pd.read_csv(os.path.join(data_path_wasm, "IS/wasm.csv"), index_col=False)

    # print(df_is_native)
    fig, ax = plt.subplots()

    ax.plot(df_is_native['nproc'], df_is_native['mop_per_s'], label='Native')
    ax.plot(df_is_wasm['nproc'], df_is_wasm['mop_per_s'], label='Wasm')
    plt.xlabel('Number of MPI Processes')
    plt.ylabel('Mop/s')
    plt.legend()
    plt.savefig("Plots/IS/is_mops.png", dpi=300)   
    
    logging.info(f'Generating plots for IMB')
    generate_plot_imb(os.path.join(data_path_native, "IMB/4_proc"), os.path.join(data_path_wasm, "IMB/4_proc"), "Plots/IMB/4_proc")
    generate_plot_imb(os.path.join(data_path_native, "IMB/8_proc"), os.path.join(data_path_wasm, "IMB/8_proc"), "Plots/IMB/8_proc")


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    parser = argparse.ArgumentParser()
    parser.add_argument('-d', '--dir', type=str)
    args = parser.parse_args()
    create_plots(args.dir)
    