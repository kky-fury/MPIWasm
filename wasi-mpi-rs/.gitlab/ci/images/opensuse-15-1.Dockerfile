# Same OpenSUSE version as used on SuperMUC
FROM opensuse/leap:15.1

RUN zypper -n update
RUN zypper -n install -t pattern devel_C_C++
RUN zypper -n install \
    bzip2 \
    curl \
    gcc10 \
    gcc10-c++ \
    libffi-devel \
    libstdc++-devel \
    linux-glibc-devel \
    glibc-devel-static \
    zlib-devel-static \
    libxml2-devel \
    ncurses5-devel \
    openssh \
    openssl-devel \
    git-core



# Install CLang+LLVM
ENV LLVM_VERSION=11.0.1
RUN set -eux; \
    case "$(arch)" in \
      x86_64) assetName="clang+llvm-$LLVM_VERSION-x86_64-linux-gnu-ubuntu-16.04";; \
      aarch64) assetName="clang+llvm-$LLVM_VERSION-aarch64-linux-gnu";; \
      *) exit 1;; \
    esac; \
    curl -O -J -L "https://github.com/llvm/llvm-project/releases/download/llvmorg-$LLVM_VERSION/$assetName.tar.xz"; \
    tar xf $assetName.tar.xz; \
    mv $assetName /opt/llvm
ENV PATH="/opt/llvm/bin:${PATH}"
ENV LLVM_SYS_110_PREFIX="/opt/llvm"

ENV CC=clang
ENV CXX=clang++

# Install OpenMPI
ARG OPENMPI_VERSION_MAJOR=4.0
ARG OPENMPI_VERSION_MINOR=7
ARG OPENMPI_VERSION=${OPENMPI_VERSION_MAJOR}.${OPENMPI_VERSION_MINOR}

RUN curl -O -J https://download.open-mpi.org/release/open-mpi/v${OPENMPI_VERSION_MAJOR}/openmpi-${OPENMPI_VERSION}.tar.bz2
RUN tar xf openmpi-${OPENMPI_VERSION}.tar.bz2
WORKDIR openmpi-${OPENMPI_VERSION}
RUN ./configure --enable-static --prefix=/usr/local
RUN make all install
WORKDIR /
RUN ldconfig





# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
