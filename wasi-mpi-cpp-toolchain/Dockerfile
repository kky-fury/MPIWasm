FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y cmake curl python3 python3-requests

WORKDIR /tmp/installer
COPY ./install.py ./install.py
COPY ./include ./include
RUN ./install.py wasi-sdk-12 /opt/wasi-sdk/12

ENV WASI_SDK_PREFIX=/opt/wasi-sdk/12
ENV CC="/opt/wasi-sdk/12/bin/clang --sysroot=/opt/wasi-sdk/12/share/wasi-sysroot"
ENV CXX=$CC

WORKDIR /work
COPY /example/intel-mpi-benchmarks /work/example/intel-mpi-benchmarks/
COPY /example/hpcg-benchmark /work/example/hpcg-benchmark/
COPY /example/NPB3.4.2 /work/example/NPB3.4.2

CMD ["/bin/bash"]