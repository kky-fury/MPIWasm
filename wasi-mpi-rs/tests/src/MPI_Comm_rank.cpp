#include <cassert>
#include <mpi.h>

int main(int argc, char *argv[]) {
    MPI_Init(&argc, &argv);

    int size;
    int rank;
    MPI_Comm_size(MPI_COMM_WORLD, &size);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    assert(size > 0);
    assert(rank >= 0);
    assert(rank < size);

    MPI_Finalize();
    return 0;
}
