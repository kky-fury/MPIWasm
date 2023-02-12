FROM kkyfury/opensuse-15-1-modified:v2

ENV OMPI_CC=clang
ENV OMPI_CXX=clang++

WORKDIR /intel_mpi_benchmarks
ARG IMB_VERSION=2019.6
RUN curl -O -J -L "https://github.com/intel/mpi-benchmarks/archive/refs/tags/IMB-v2019.6.tar.gz"
RUN tar -xvzf mpi-benchmarks-IMB-v2019.6.tar.gz
RUN mpicc --version
RUN cd mpi-benchmarks-IMB-v2019.6/src_c \
 && make clean && make CC=mpicc -j$(getconf _NPROCESSORS_ONLN) 

CMD ["/bin/bash"]