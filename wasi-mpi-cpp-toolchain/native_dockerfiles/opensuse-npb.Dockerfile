FROM kkyfury/opensuse-15-1-modified:v2


ENV OMPI_CC=clang
ENV OMPI_CXX=clang++



WORKDIR /npb
RUN curl -O -J -L -k "https://www.nas.nasa.gov/assets/npb/NPB3.4.2.tar.gz" && tar -xvzf NPB3.4.2.tar.gz && \
        cd NPB3.4.2/ && cd NPB3.4-MPI && cd config && cp make.def.template make.def && cd .. \
        && make IS CLASS=C && make IS CLASS=D && make DT CLASS=C && make DT CLASS=D && make DT CLASS=B