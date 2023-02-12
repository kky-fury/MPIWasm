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
    mops_values = []
    mopsps_values = []
    for _, _, filenames in os.walk(dirpath):
        for file in filenames:
            print(file)
            with open(os.path.join(dirpath, file)) as f:
                data = f.readlines()
                for line in data:
                    print(line)
                    if "Total number of processes" in line:
                        print(line)
                        parsed_line = int(line.split(":")[-1].strip(" "))
                        # print(parsed_line)
                        num_processes = parsed_line
                    elif "Mop/s total" in line:
                        print(line)
                        parsed_line = float(line.split("=")[-1].strip(" "))
                        # print(parsed_line)
                        mops_values.append(parsed_line)
                    elif " Mop/s/process" in line:
                        print(line)
                        parsed_line = float(line.split("=")[-1].strip(" "))
                        # print(parsed_line)
                        mopsps_values.append(parsed_line)
    mops_mean = statistics.fmean(mops_values)
    mopsps_mean = statistics.fmean(mopsps_values)
    print(mops_mean)
    print(mopsps_mean)
    df = pd.DataFrame(
        {
            "nproc":[num_processes],
            "mop_per_s":[mops_mean],
            "mop_per_s_per_proc":[mopsps_mean]
        }
    )
    return df

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('-d', '--dir', type=str)
    parser.add_argument('-f', '--filename', type=str)
    args = parser.parse_args()
    main(args.dir, args.filename)