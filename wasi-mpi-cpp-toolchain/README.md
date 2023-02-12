# WASI MPI CPP Toolchain
This directory contains the `mpi.h` header file for use with WASI MPI and a Python script that downloads the correct
version of WASI SDK for your system and places the `mpi.h` header in the sysroot of the WASI SDK installation to
simplify the compilation process.

## Installation
Install WASI-SDK with the `mpi.h` header by executing
```bash
python3 ./install.py
```
The script will prompt you for the version of WASI-SDK you want to install and the directory to install it to.

## Compiling Make-based or plain C projects
You can set the `CC` and `CXX` environment variables to refer to the CLang compiler that ships with WASI-SDK and
specify that it should use the WASI-SDK sysroot (the following examples use the default installation directory
of `~/opt/wasi-sdk/12`):
```bash
export CC="~/opt/wasi-sdk/12/bin/clang --sysroot=~/opt/wasi-sdk/12/share/wasi-sysroot"
export CXX=$CC
```
Some projects may require other variables such as `MPICC` to be set.

## Compiling CMake-based projects
CMake projects are a little more involved compared to plain or Make-based ones. Copy `wasi-cmake.sh` to the root of
the project you want to build and add the following snippet to `CMakeLists.txt`:
```
if (CMAKE_SYSTEM_NAME STREQUAL "WASI")
    SET(CMAKE_EXECUTABLE_SUFFIX ".wasm")
    add_compile_options(-fno-exceptions)
    add_link_options( -Wl,--allow-undefined,--export=malloc,--export=free)
endif()
```

If the CMake build for the application makes use of commands like `find_package(MPI)` you need to feature gate them
to only be executed when the system is not `WASI`.

Then you can setup a build by executing `./wasi-cmake.sh`.
