# MPIWasm
MPIWasm is a WebAssembly Runtime (embedder) based on Wasmer that enables the high-performance execution of MPI applications compiled to Wasm.

You can find more details about MPIWasm in our ACM SIGPLAN PPoPP'23 [Paper](https://arxiv.org/abs/2301.03982).

# Getting Started:
Running our webassembly embedder (MPIWasm) for executing MPI applications compiled to WebAssembly (Wasm). 

## What is MPIWasm?
MPIWasm is an embedder for MPI based HPC applications based on [Wasmer](https://wasmer.io/). It enables the high performance execution of MPI-based HPC applications compiled to Wasm. It servers two purposes:

1. Delivering close to native application performance, i.e., when applications are executed directly on a host machine without using Wasm. 
2. Enabling the distribution of MPI-based HPC applications as Wasm binaries.


## Requirements
- Docker
- Currently the docker image is only built for the linux/amd64 platform. For building images for other platforms, please see [here](#support-for-arm64).

## Steps:
    sudo docker run -it kkyfury/ppoppae:v2 /bin/bash
    
    #Executing the HPCG benchmark compiled to Wasm inside the docker container
    mpirun --allow-run-as-root -np 4 ./target/release/embedder examples/xhpcg.wasm 
    #Wait 2~3 mins and you can see the execution
### What should the output look like?
MPIWasm should succcessfully execute the HPCG benchmark which has been compiled to Wasm. On successful exeuction, you should see something similar to [this](./wasi-mpi-rs/sample_outputs/HPCG/hpcg_run_results_4_proc.out).


    #Executing the IntelMPI benchmarks compiled to Wasm inside the docker container, redirecting nonaffecting errors to a file
    mpirun --allow-run-as-root -np 4 ./target/release/embedder examples/imb.wasm 2>error
    #Wait 2~3 mins and you can see the execution

### What should the output look like?
MPIWasm should succcessfully execute the IntelMPI benchmarks which have been compiled to Wasm. On successful exeuction, you should see something similar to [this](./wasi-mpi-rs/sample_outputs/IMB/imb_run_results_4_procs.out).

### Note:
The number of processes for execution (-np) can be increased or decreased. However, depending on your machine you might need to provide `-oversubscribe` flag to `mpirun`. 

# Running Experiments with MPIWasm
This section describes how to run experiments with our embedder to obtain plots simlar to the ones in our paper.

## Running small-scale experiments inside the docker container.
To run small-scale experiments inside the docker container, we provide an end-to-end [script](./wasi-mpi-rs/run_experiments/runme.sh). This script: 

1. Executes the HPCG, IS, and IntelMPI benchmarks for their native execution and when they are executed using MPIWasm after compilation to Wasm. 
2. Parses the obtained results and generates the relevant plots.

### Script execution:
    sudo docker run -it kkyfury/ppoppae:v2 /bin/bash
    cd run_experiments
    ./runme.sh

The script can take around 10-15 minutes to finish execution. After completion, you can see the generated data in the `run_experiments/experiment_data` folder. The generated plots can be found in the `run_experiments/Plots` folder. For copying the plots to your local filesystem, please use the [docker-cp](https://docs.docker.com/engine/reference/commandline/cp/) command.


## Running large-scale experiments on an HPC system.
This section describes running MPIWasm for executing MPI applications on multiple nodes of an HPC system. For this, a user needs to do the following:

1. Build a version of the embedder for your HPC system depending on the particular architecture, operating system, and the MPI library on the system ([reference](#embedder-mpiwasm-section-3)).  MPIWasm currently supports the OpenMPI library. 
2. For building MPIWasm for different linux distributions and OpenMPI versions please see [here](#support-for-multiple-operating-systems). It is important to ensure that the MPI library version on the HPC system matches the one with which the embedder is built with. For building MPIWasm for different architectures, please see [here](#building-images-for-arm64).
3. Execute the MPI applications using the built embedder on the HPC system. This can be done via submitting jobs to a RJMS software on an HPC system such as [SLURM](https://slurm.schedmd.com/). We provide sample job scripts for our HPC system, i.e., SuperMUC-NG that uses SLURM [here](./wasi-mpi-rs/sample_outputs/sample_job_scripts/).
4. After executing the applications, the user can use the different [Parsers](./wasi-mpi-rs/Parsers/) to parse the benchmark data. Following this, the results can be visualized using out Plotting helper [script](./wasi-mpi-rs/Plots/main.py).

# Step by Step Instructions:

## Compiling C/C++ applications to Wasm (Section 3.2)
We have setup a docker container with the required dependencies for compiling different MPI applications conformant to the MPI-2.2 standard to Wasm. We have also included the HPCG benchmark, the Intel MPI benchmarks, and the IS benchmark as examples.

### Steps:
    sudo docker run -it kkyfury/wasitoolchain:v1 /bin/bash
    
    #Compiling HPCG
    cd /work/example/hpcg-benchmark
    ./wasi-cmake.sh
    cd cmake-build-wasi
    make 
    # Following this, you can see the generated wasm binary which can be executed with our embedder.

    #Compiling IntelMPI Benchmarks
    cd /work/example/intel-mpi-benchmarks
    ./wasi-cmake.sh
    cd cmake-build-wasi
    make 
    # Ignore the warnings during compilation, you can see the generated wasm binaries in cmake-build-wasi/src_cpp directory

    #Compiling IS Benchmark
    cd NPB3.4.2/NPB3.4-MPI/
    make IS CLASS=C
    #Following this, you can see the generated wasm binary in the bin folder.


For compiling different MPI applications to Wasm, please refer to this [Readme file](./wasi-mpi-cpp-toolchain/README.md).
All the different applications compiled to Wasm that we used in our paper are present [here](./wasi-mpi-rs/examples/) (Section 4). More details about them can be found [here](./wasi-mpi-cpp-toolchain/README.md).

## Compiling C/C++ applications natively
We provide [Dockerfiles](./wasi-mpi-cpp-toolchain/native_dockerfiles/) for natively compiling the different benchmarks used in our experiments. All pre-compiled native binaries can be found [here](./wasi-mpi-rs/native_binaries/).


## Embedder (MPIWasm) (Section 3)

### Building from source
<!-- To enable users to modify and change our embedder easily, we provide a docker-compose file that faciliatese
To enable ease-of-development for our embedder, we provide a docker-compose file that enables users to easily make changes to the embedder and generate new versions for different operating systems. The docker-compose file currently supports `opensuse-15.1` -->
For detailed instructions, please look at this [Readme file](./wasi-mpi-rs/README.md). 

#### Note
We recommend users to use our provided [docker-compose](./wasi-mpi-rs/docker-compose.yml) file for building the embedder from source for different distributions. This prevents the unnecessary installation of software on a user's local system.

### Sample Usage for Ubuntu:20.04

    cd wasi-mpi-rs
    docker-compose run ubuntu-20-04
    cargo build --release
    #After the build process, you can see the built embedder in the /target/release/ folder.

### Support for multiple operating systems.
We provide support for building our embedder for the following distributions:
1. `centos-8-2`
2. `opensuse-15-1`
3. `ubuntu-20-04`
4. `macos-monterey` based on [Docker-OSX](https://github.com/sickcodes/Docker-OSX#quick-start-docker-osx). Note, that the generated embedder might not be directly compatible for darwin distributions.

For all of these distributions, the embedder can be built by using the provided [docker-compose](./wasi-mpi-rs/docker-compose.yml) file.

#### Sample usage for centos-8.2
    cd wasi-mpi-rs
    docker compose run centos-8-2
    cargo build --release
    #After the build process, you can see the built embedder in the /target/release/ folder.
Following this, the user can copy the embedder for usage of their HPC system using the [docker-cp](https://docs.docker.com/engine/reference/commandline/cp/) command.

#### Example:
    docker cp <container-id>:/s/target/release/embedder <destination-path-user-filesystem>



The base images used for the different operating systems can be found [here](./wasi-mpi-rs/.gitlab/ci/images/). These example Dockerfiles can be easily extended to support other different linux distributions. We provide pre-built versions of our embedder for the different distributions [here](./wasi-mpi-rs/pre-built-embedders/x86_64/). For specific OpenMPI versions, please see the individual [Dockerfiles](./wasi-mpi-rs/.gitlab/ci/images/).

#### Note:
The path for the generated embedder for `macos-monterey` is `/home/arch/s/`.

<br />

### Usage
For detailed instructions, please look at this [Readme file](./wasi-mpi-rs/README.md).

<br />

### Modifying the embedder
For modifying our embedder, we recommend using our provided [docker-compose](./wasi-mpi-rs/docker-compose.yml) file for any of the supported operating systems. This docker-compose file mounts the volume with the embedder's source code inside the container. As a result, any changes to it's source code will be reflected inside it.

All modifications to the embedder need to be done [here](./wasi-mpi-rs/src/). Following this, the embedder needs to be recompiled.

#### Sample workflow
    cd wasi-mpi-rs
    docker-compose run ubuntu-20-04
    #Make any relevant change to the embedder's source code inside the /wasi-mpi-rs/src/ directory. These changes are automatically reflected inside the container. 
    cargo build --release
After the build process, the new embedder can be copied to the user's local filesystem using the [docker-cp](https://docs.docker.com/engine/reference/commandline/cp/) command.

<br />

### Support for arm64
Our embedder also supports execution on linux/arm64 platforms. We provide pre-built versions of our embedder for arm64 for the different linux distributions [here](./wasi-mpi-rs/pre-built-embedders/arm64/). For specific, OpenMPI versions, please see the [Dockerfiles](./wasi-mpi-rs/.gitlab/ci/images/).

<br />

### Building images for arm64

Requirements:

If you are building the docker image on an x86_64 system then you require [docker-buidlx](https://github.com/docker/buildx). Note that, building the image might take around 12 hours. 

If you are using an arm64 machine then follow the [normal instructions](#building-from-source).


Example for building the embedder for ubuntu:20.04 for arm64 on an x86_64 machine. Please follow the steps below:

    sudo docker buildx create --name mybuilder --use --bootstrap
    cd wasi-mpi-rs/.gitlab/ci/images/
    sudo docker buildx build --push -f ubuntu-20-04.Dockerfile --platform linux/arm64 -t kkyfury/ubuntumodifiedbase:v1 .
    cd ../../../
    sudo docker buildx build --push -f Dockerfile --platform linux/arm64 -t kkyfury/embedderarm:v1 .

Please change the docker image tags according to your docker registry account, i.e., replace kkyfury with your registry username. Following this, the image name in the FROM keyword in the [Dockerfile](./wasi-mpi-rs/Dockerfile) needs to changed accordingly.