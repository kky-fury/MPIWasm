# ch-test-scope: full
FROM kkyfury/opensuse-15-1-modified:v2
# FROM kkyury/suseopenmpi:latest

# Compile the Intel MPI benchmark
ENV OMPI_CC=clang
ENV OMPI_CXX=clang++




WORKDIR /hpcg_benchmark
RUN mpicxx --version
RUN git clone https://github.com/kky-fury/hpcg.git
RUN cd hpcg && mkdir build && cd build && ls .. && ../configure Linux && make -j$(getconf _NPROCESSORS_ONLN) 
CMD ["/bin/bash"]