#include <array>
#include <cassert>
#include <mpi.h>

const int SEND_RANK = 0;
const int RECV_RANK = 1;
const size_t BUF_SIZE = 8;

int main(int argc, char *argv[]) {
    MPI_Init(&argc, &argv);

    int size;
    int rank;
    MPI_Comm_size(MPI_COMM_WORLD, &size);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    if (rank == SEND_RANK) {
        std::array<int, BUF_SIZE> send_buf {0, 1, 2, 3, 4, 5, 6, 7};
        MPI_Send(send_buf.begin(), send_buf.size(), MPI_INT, RECV_RANK, 0, MPI_COMM_WORLD);
    } else if (rank == RECV_RANK) {
        std::array<int, BUF_SIZE> expected{0, 1, 2, 3, 4, 5, 6, 7};
        std::array<int, BUF_SIZE> recv_buf{};
        MPI_Status status;

        MPI_Recv(recv_buf.begin(), recv_buf.size(), MPI_INT, SEND_RANK, 0, MPI_COMM_WORLD, &status);
        assert(recv_buf == expected);
    }

    MPI_Finalize();
    return 0;
}
