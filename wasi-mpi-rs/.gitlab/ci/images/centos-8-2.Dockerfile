# Same CentOS version as used on a64fx testbed
FROM centos:8.2.2004

RUN cd /etc/yum.repos.d/
RUN sed -i 's/mirrorlist/#mirrorlist/g' /etc/yum.repos.d/CentOS-*
RUN sed -i 's|#baseurl=http://mirror.centos.org|baseurl=http://vault.centos.org|g' /etc/yum.repos.d/CentOS-* 
RUN yum -y group install "Development Tools"
RUN yum -y install \
    libffi-devel \
    libxml2-devel \
    ncurses-compat-libs \
    ncurses-devel \
    openssl-devel

# Install OpenMPI
ARG OPENMPI_VERSION_MAJOR=4.0
ARG OPENMPI_VERSION_MINOR=4
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
