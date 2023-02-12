FROM kkyfury/opensuse-15-1-modified:v2


WORKDIR app

COPY ./Parsers/ /app/Parsers/
COPY ./native_binaries /app/native_binaries


COPY ./.cargo /app/cargo
COPY ./config /app/config
COPY ./examples/ /app/examples
COPY ./src/ /app/src
COPY ./tests/src /app/tests/src 
COPY ./tests/CMakeLists.txt /app/tests/CMakeLists.txt 
COPY ./tests/wasi-cmake.sh /app/tests/wasi-cmake.sh 
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml 

RUN cargo build --release 
COPY ./run_experiments/ /app/run_experiments/
COPY ./Plots/ /app/Plots/

CMD ["/bin/bash"]
