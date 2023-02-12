FROM kkyfury/macosmodified:v1

WORKDIR /home/arch/app

COPY ./Parsers/ /home/arch/app/Parsers/
COPY ./native_binaries /home/arch/app/native_binaries

COPY ./.cargo /home/arch/app/cargo
COPY ./config /home/arch/app/config
COPY ./examples/ /home/arch/app/examples
COPY ./src/ /home/arch/app/src
COPY ./tests/src /home/arch/app/tests/src 
COPY ./tests/CMakeLists.txt /home/arch/app/tests/CMakeLists.txt 
COPY ./tests/wasi-cmake.sh /home/arch/app/tests/wasi-cmake.sh 
COPY ./Cargo.lock /home/arch/app/Cargo.lock
COPY ./Cargo.toml /home/arch/app/Cargo.toml 

ENV OPENSSL_INCLUDE_DIR="/home/linuxbrew/.linuxbrew/opt/openssl@1.1/include/"
ENV OPENSSL_LIB_DIR="/home/linuxbrew/.linuxbrew/opt/openssl@1.1/lib/"
RUN cargo build --release 

ENV LD_LIBRARY_PATH="/usr/local/lib:${LD_LIBRARY_PATH}"

CMD ["/bin/bash"]
