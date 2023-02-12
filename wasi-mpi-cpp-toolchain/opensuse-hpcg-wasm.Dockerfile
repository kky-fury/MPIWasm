# ch-test-scope: full
FROM kkyfury/opensuse-15-1-modified:v2
# FROM kkyury/suseopenmpi:latest

RUN zypper install -y cmake curl python3 python3-requests

# Compile the Intel MPI benchmark
ENV OMPI_CC=clang
ENV OMPI_CXX=clang++

WORKDIR /tmp/installer
COPY ./install.py ./install.py
COPY ./include ./include
RUN ./install.py wasi-sdk-12 /opt/wasi-sdk/12

ENV WASI_SDK_PREFIX=/opt/wasi-sdk/12
ENV CC="/opt/wasi-sdk/12/bin/clang --sysroot=/opt/wasi-sdk/12/share/wasi-sysroot"
ENV CXX=$CC

WORKDIR /work
COPY /example/hpcg-benchmark /work/example/hpcg-benchmark/
COPY /example/pingpong /work/example/pingpong/

CMD ["/bin/bash"]