cmake_minimum_required(VERSION 3.14)
project(wasi_mpi_tests)
set(CMAKE_CXX_STANDARD 20)

if (CMAKE_SYSTEM_NAME STREQUAL "WASI")
    SET(CMAKE_EXECUTABLE_SUFFIX ".wasm")
    SET(TEST_RUNNER mpirun ../../target/release/embedder)
else()
    SET(TEST_RUNNER mpirun)
endif()

function(add_test_file TEST_NAME TEST_FILE)
    add_executable(${TEST_NAME} ${TEST_FILE})

    if (CMAKE_SYSTEM_NAME STREQUAL "WASI")
        target_compile_options(${TEST_NAME} PUBLIC -fno-exceptions)
        target_link_options(${TEST_NAME} PUBLIC -Xlinker --allow-undefined)
    else()
        find_package(MPI)
        target_link_libraries(${TEST_NAME} MPI::MPI_C)
    endif()

    add_test(NAME ${TEST_NAME} COMMAND ${TEST_RUNNER} $<TARGET_FILE:${TEST_NAME}>)
endfunction()

add_test_file(MPI_Comm_rank src/MPI_Comm_rank.cpp)
add_test_file(MPI_Comm_send src/MPI_Comm_send.cpp)
enable_testing()
