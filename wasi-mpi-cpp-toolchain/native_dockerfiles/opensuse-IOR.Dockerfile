# ch-test-scope: full
FROM  kkyfury/opensuse-15-1-modified:v2
# FROM kkyury/suseopenmpi:latest

# Compile the Intel MPI benchmark
ENV OMPI_CC=clang
ENV OMPI_CXX=clang++

WORKDIR /ior
RUN curl -O -J -L "https://github.com/hpc/ior/releases/download/3.3.0/ior-3.3.0.tar.gz" && tar -xvzf ior-3.3.0.tar.gz && cd ior-3.3.0 \
    && ls && ./configure && make -j 4
CMD ["/bin/bash"]