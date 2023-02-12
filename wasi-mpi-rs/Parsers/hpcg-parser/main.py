import pandas as pd
import os
import argparse
import statistics

def main(dirpath, filename):
    list_dfs = []
    for _, dirnames, _ in os.walk(dirpath):
        for dir in dirnames:
            print(dir)
            df = getdf(os.path.join(dirpath, dir))
            list_dfs.append(df)
    df = pd.concat(list_dfs)
    df = df.sort_values("nproc")
    print(df)
    df.to_csv(os.path.join(dirpath, filename), index=False)


def getdf(dirpath):
    num_processes = ''
    gflops_values = []
    bandwidth_values = []
    for _, _, filenames in os.walk(dirpath):
        for file in filenames:
            print(file)
            with open(os.path.join(dirpath, file)) as f:
                data = f.readlines()
                for line in data:
                    if "Distributed Processes" in line:
                        # print(line)
                        parsed_line = int(line.split("=")[-1].strip("\n"))
                        # print(parsed_line)
                        num_processes = parsed_line
                    elif "GFLOP/s Summary" and "VALID" in line:
                        # print(line)
                        parsed_line = float(line.split("=")[-1].strip("\n"))
                        # print(parsed_line)
                        gflops_values.append(parsed_line)
                    elif "GB/s Summary::Total" in line:
                        # print(line)
                        parsed_line = float(line.split("=")[-1].strip("\n"))
                        # print(parsed_line)
                        bandwidth_values.append(parsed_line)
    gflop_mean = statistics.fmean(gflops_values)
    bandwidth_mean = statistics.fmean(bandwidth_values)
    print(gflop_mean)
    print(bandwidth_mean)
    df = pd.DataFrame(
        {
            "nproc":[num_processes],
            "gflop_per_s":[gflop_mean],
            "gb_per_s":[bandwidth_mean]
        }
    )
    return df

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('-d', '--dir', type=str)
    parser.add_argument('-f', '--filename', type=str)
    args = parser.parse_args()
    main(args.dir, args.filename)