FROM sickcodes/docker-osx:monterey

# WORKDIR /app
RUN NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" 
ENV PATH="/home/linuxbrew/.linuxbrew/bin:${PATH}"
# RUN brew install libtinfo5 \
#     libtinfo-dev
WORKDIR /home/arch
# Install CLang+LLVM
ENV LLVM_VERSION=11.0.1
ARG assetName="clang+llvm-${LLVM_VERSION}-x86_64-linux-gnu-ubuntu-16.04"
RUN curl -O -J -L "https://github.com/llvm/llvm-project/releases/download/llvmorg-$LLVM_VERSION/${assetName}.tar.xz"; \ 
    tar xf ${assetName}.tar.xz; \ 
    sudo mv ${assetName} /opt/llvm

ENV PATH="/opt/llvm/bin:${PATH}"
ENV LLVM_SYS_110_PREFIX="/opt/llvm"
RUN brew install ncurses && \
    ln -s /home/linuxbrew/.linuxbrew/lib/libtinfo.so /home/linuxbrew/.linuxbrew/lib/libtinfo.so.5
ENV LD_LIBRARY_PATH="/home/linuxbrew/.linuxbrew/lib:${LD_LIBRARY_PATH}"

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/arch/.cargo/bin:${PATH}"

ENV CC=gcc
ENV CXX=g++

# Install OpenMPI
ARG OPENMPI_VERSION_MAJOR=4.0
ARG OPENMPI_VERSION_MINOR=7
ARG OPENMPI_VERSION=${OPENMPI_VERSION_MAJOR}.${OPENMPI_VERSION_MINOR}

RUN curl -O -J https://download.open-mpi.org/release/open-mpi/v${OPENMPI_VERSION_MAJOR}/openmpi-${OPENMPI_VERSION}.tar.bz2
RUN tar xf openmpi-${OPENMPI_VERSION}.tar.bz2
WORKDIR openmpi-${OPENMPI_VERSION}
RUN ./configure --prefix=/usr/local
RUN sudo make all install
RUN sudo ldconfig

RUN brew install openssl@1.1 pkg-config perl
WORKDIR /home/arch
# RUN rustup install stable
# RUN rust rustup default stable
CMD ["/bin/bash"]

