FROM ubuntu:20.04

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

RUN apt-get update
RUN apt-get install -y \
    build-essential \
    curl \
    libffi-dev \
    libssl-dev \
    libtinfo5 \
    libtinfo-dev \
    libxml2-dev \
    openssh-client \
    pkg-config \
    zlib1g-dev

# Install OpenMPI
ARG OPENMPI_VERSION_MAJOR=4.1
ARG OPENMPI_VERSION_MINOR=1
ARG OPENMPI_VERSION=${OPENMPI_VERSION_MAJOR}.${OPENMPI_VERSION_MINOR}

RUN curl -O -J https://download.open-mpi.org/release/open-mpi/v${OPENMPI_VERSION_MAJOR}/openmpi-${OPENMPI_VERSION}.tar.bz2
RUN tar xf openmpi-${OPENMPI_VERSION}.tar.bz2
WORKDIR openmpi-${OPENMPI_VERSION}
RUN ./configure --prefix=/usr/local
RUN make all install
WORKDIR /
RUN ldconfig


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


# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
