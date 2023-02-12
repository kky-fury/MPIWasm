//
// Created by Nils Krüger on 16.05.21.
//

#ifndef WASM_MPI_RS_MPI_H
#define WASM_MPI_RS_MPI_H

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C"
{
#endif
#define MPI_SUCCESS 0
#define MPI_ERR_OTHER 1

// Comparison results
enum {
    MPI_IDENT,
    MPI_CONGRUENT,
    MPI_SIMILAR,
    MPI_UNEQUAL
};

/**
 * Custom MPI implementation
 * Official MPI spec: https://www.mpi-forum.org/docs/
 * Open MPI repo: https://github.com/open-mpi/ompi
 */

#define MPI_MAX_OBJECT_NAME 128



/**
 * User-facing constants
 */
// MPI_Comms
#define MPI_COMM_WORLD 0
#define MPI_COMM_SELF 1
#define MPI_COMM_NULL 2

// Communicator types
enum
{
    MPI_COMM_TYPE_SHARED,
};

#define MPI_INT8_T 0
#define MPI_INT16_T 1
#define MPI_INT32_T 2
#define MPI_INT 3
#define MPI_INT64_T 4
#define MPI_UINT8_T 5
#define MPI_UINT16_T 6
#define MPI_UINT32_T 7
#define MPI_UINT_T 8
#define MPI_UINT64_T 9
#define MPI_LONG 10
#define MPI_LONG_LONG 11
#define MPI_LONG_LONG_INT 12
#define MPI_FLOAT 13
#define MPI_DOUBLE 14
#define MPI_DOUBLE_INT 15
#define MPI_CHAR 16
#define MPI_C_BOOL 17
#define MPI_BYTE 18
#define MPI_DATATYPE_NULL -1

// MPI flags
// These are special pointers passed in place of normal buffers to signify
// special operations (e.g. in-place manipulations). We make the pointers
// themselves equal to a specific integer so that they can be identified.
#define MPI_BOTTOM (void*) 0
#define MPI_IN_PLACE (void*) 1

// MPI_Infos
#define MPI_INFO_NULL 0

// Misc constants (compatible with OpenMPI)
#define MPI_ANY_SOURCE -1
#define MPI_ANY_TAG -1
#define MPI_UNDEFINED -32766

// Misc limits
#define MPI_MAX_PROCESSOR_NAME 256
#define MPI_CART_MAX_DIMENSIONS 2

#define MPI_MAX 0
#define MPI_MIN 1
#define MPI_SUM 2
#define MPI_PROD 3
#define MPI_LAND 4
#define MPI_LOR 5
#define MPI_BAND 6
#define MPI_BOR 7
#define MPI_MAXLOC 8
#define MPI_MINLOC 9
#define MPI_OP_NULL 10

// MPI_Statuses
#define MPI_STATUS_IGNORE ((MPI_Status*)(0))
#define MPI_STATUSES_IGNORE ((MPI_Status*)(0))

// Window attributes
#define MPI_WIN_BASE 1
#define MPI_WIN_SIZE 2
#define MPI_WIN_DISP_UNIT 3
#define MPI_WIN_CREATE_FLAVOR 4
#define MPI_WIN_MODEL 5

// MPI Threads
enum
{
    MPI_THREAD_SINGLE,
    MPI_THREAD_FUNNELED,
    MPI_THREAD_SERIALIZED,
    MPI_THREAD_MULTIPLE
};

/*
 * MPI_Status (taken from OpenMPI 4.1)
 */
struct wasi_mpi_rs_status_public_t {
    /* These fields are publicly defined in the MPI specification.
       User applications may freely read from these fields. */
    int MPI_SOURCE;
    int MPI_TAG;
    int MPI_ERROR;
    /* The following two fields are internal to the Open MPI
       implementation and should not be accessed by MPI applications.
       They are subject to change at any time.  These are not the
       droids you're looking for. */
    int _cancelled;
    int64_t _ucount;
};
typedef struct wasi_mpi_rs_status_public_t wasi_mpi_rs_status_public_t;

/*
 * User-facing types
 */
typedef int MPI_Op;
typedef int MPI_Comm;
typedef int MPI_Datatype;
typedef wasi_mpi_rs_status_public_t MPI_Status;
typedef int MPI_Message;
typedef int MPI_Info;
typedef int MPI_Request;
typedef int MPI_Group;
typedef int MPI_Win;
typedef ptrdiff_t MPI_Aint;
typedef int MPI_Fint;
typedef long MPI_Offset;

/*
 * User-defined functions
 */
typedef void(MPI_User_function)(void*, void*, int*, MPI_Datatype*);

/*
 * User-facing functions
 */
int MPI_Allgather(const void* sendbuf,
                  int sendcount,
                  MPI_Datatype sendtype,
                  void* recvbuf,
                  int recvcount,
                  MPI_Datatype recvtype,
                  MPI_Comm comm);

int MPI_Allgatherv(const void* sendbuf,
                   int sendcount,
                   MPI_Datatype sendtype,
                   void* recvbuf,
                   const int* recvcounts,
                   const int* displs,
                   MPI_Datatype recvtype,
                   MPI_Comm comm);

int MPI_Alloc_mem(MPI_Aint size, MPI_Info info, void* baseptr);

int MPI_Allreduce(const void* sendbuf,
                  void* recvbuf,
                  int count,
                  MPI_Datatype datatype,
                  MPI_Op op,
                  MPI_Comm comm);

int MPI_Alltoall(const void* sendbuf,
                 int sendcount,
                 MPI_Datatype sendtype,
                 void* recvbuf,
                 int recvcount,
                 MPI_Datatype recvtype,
                 MPI_Comm comm);

int MPI_Alltoallv(const void* sendbuf,
                  const int sendcounts[],
                  const int sdispls[],
                  MPI_Datatype sendtype,
                  void* recvbuf,
                  const int recvcounts[],
                  const int rdispls[],
                  MPI_Datatype recvtype,
                  MPI_Comm comm);

int MPI_Abort(MPI_Comm comm, int errorcode);

int MPI_Barrier(MPI_Comm comm);

int MPI_Bcast(void* buffer,
              int count,
              MPI_Datatype datatype,
              int root,
              MPI_Comm comm);

int MPI_Cart_create(MPI_Comm old_comm,
                    int ndims,
                    const int dims[],
                    const int periods[],
                    int reorder,
                    MPI_Comm* comm);

int MPI_Cart_get(MPI_Comm comm,
                 int maxdims,
                 int dims[],
                 int periods[],
                 int coords[]);

int MPI_Cart_rank(MPI_Comm comm, int coords[], int* rank);

int MPI_Cart_shift(MPI_Comm comm,
                   int direction,
                   int disp,
                   int* rank_source,
                   int* rank_dest);

MPI_Fint MPI_Comm_c2f(MPI_Comm comm);

int MPI_Comm_compare(MPI_Comm comm1, MPI_Comm comm2, int *result);

int MPI_Comm_create(MPI_Comm comm, MPI_Group group, MPI_Comm* newcomm);

int MPI_Comm_create_group(MPI_Comm comm,
                          MPI_Group group,
                          int tag,
                          MPI_Comm* newcomm);

int MPI_Comm_dup(MPI_Comm comm, MPI_Comm* newcomm);

MPI_Comm MPI_Comm_f2c(MPI_Fint comm);

int MPI_Comm_free(MPI_Comm* comm);

int MPI_Comm_group(MPI_Comm comm, MPI_Group* group);

int MPI_Comm_rank(MPI_Comm comm, int* rank);

int MPI_Comm_size(MPI_Comm comm, int* size);

int MPI_Comm_split(MPI_Comm comm, int color, int key, MPI_Comm* newcomm);

int MPI_Comm_split_type(MPI_Comm comm,
                        int split_type,
                        int key,
                        MPI_Info info,
                        MPI_Comm* newcomm);

int MPI_Finalize(void);

int MPI_Finalized(int* flag);

int MPI_Free_mem(void* base);

int MPI_Gather(const void* sendbuf,
               int sendcount,
               MPI_Datatype sendtype,
               void* recvbuf,
               int recvcount,
               MPI_Datatype recvtype,
               int root,
               MPI_Comm comm);

int MPI_Gatherv(const void* sendbuf,
                int sendcount,
                MPI_Datatype sendtype,
                void* recvbuf,
                const int* recvcounts,
                const int* displs,
                MPI_Datatype recvtype,
                int root,
                MPI_Comm comm);

int MPI_Get(void* origin_addr,
            int origin_count,
            MPI_Datatype origin_datatype,
            int target_rank,
            MPI_Aint target_disp,
            int target_count,
            MPI_Datatype target_datatype,
            MPI_Win win);

int MPI_Get_count(const MPI_Status* status,
                  MPI_Datatype datatype,
                  int* count);

int MPI_Get_processor_name(char* name, int* resultlen);

int MPI_Get_version(int* version, int* subversion);

int MPI_Group_free(MPI_Group* group);

int MPI_Group_incl(MPI_Group group,
                   int n,
                   const int ranks[],
                   MPI_Group* newgroup);

int MPI_Group_range_excl(MPI_Group group, int n, int ranges[][3], MPI_Group *newgroup);

int MPI_Group_range_incl(MPI_Group group, int n, int ranges[][3], MPI_Group *newgroup);

int MPI_Init(int* argc, char*** argv);

int MPI_Initialized(int* flag);

int MPI_Init_thread(int* argc, char*** argv, int required, int* provided);

int MPI_Irecv(void* buf,
              int count,
              MPI_Datatype datatype,
              int source,
              int tag,
              MPI_Comm comm,
              MPI_Request* request);

int MPI_Isend(const void* buf,
              int count,
              MPI_Datatype datatype,
              int dest,
              int tag,
              MPI_Comm comm,
              MPI_Request* request);

int MPI_Op_create(MPI_User_function* user_fn, int commute, MPI_Op* op);

int MPI_Op_free(MPI_Op* op);

int MPI_Probe(int source, int tag, MPI_Comm comm, MPI_Status* status);

int MPI_Put(const void* origin_addr,
            int origin_count,
            MPI_Datatype origin_datatype,
            int target_rank,
            MPI_Aint target_disp,
            int target_count,
            MPI_Datatype target_datatype,
            MPI_Win win);

int MPI_Query_thread(int* provided);

int MPI_Recv(void* buf,
             int count,
             MPI_Datatype datatype,
             int source,
             int tag,
             MPI_Comm comm,
             MPI_Status* status);

int MPI_Reduce(const void* sendbuf,
               void* recvbuf,
               int count,
               MPI_Datatype datatype,
               MPI_Op op,
               int root,
               MPI_Comm comm);

int MPI_Reduce_scatter(const void* sendbuf,
                       void* recvbuf,
                       const int* recvcounts,
                       MPI_Datatype datatype,
                       MPI_Op op,
                       MPI_Comm comm);

int MPI_Request_free(MPI_Request* request);

int MPI_Rsend(const void* buf,
              int count,
              MPI_Datatype datatype,
              int dest,
              int tag,
              MPI_Comm comm);

int MPI_Scan(const void* sendbuf,
             void* recvbuf,
             int count,
             MPI_Datatype datatype,
             MPI_Op op,
             MPI_Comm comm);

int MPI_Scatter(const void* sendbuf,
                int sendcount,
                MPI_Datatype sendtype,
                void* recvbuf,
                int recvcount,
                MPI_Datatype recvtype,
                int root,
                MPI_Comm comm);

int MPI_Send(const void* buf,
             int count,
             MPI_Datatype datatype,
             int dest,
             int tag,
             MPI_Comm comm);

int MPI_Sendrecv(const void* sendbuf,
                 int sendcount,
                 MPI_Datatype sendtype,
                 int dest,
                 int sendtag,
                 void* recvbuf,
                 int recvcount,
                 MPI_Datatype recvtype,
                 int source,
                 int recvtag,
                 MPI_Comm comm,
                 MPI_Status* status);

int MPI_Type_commit(MPI_Datatype* type);

int MPI_Type_contiguous(int count,
                        MPI_Datatype oldtype,
                        MPI_Datatype* newtype);

int MPI_Type_free(MPI_Datatype* datatype);

int MPI_Type_size(MPI_Datatype type, int* size);

int MPI_Wait(MPI_Request* request, MPI_Status* status);

int MPI_Waitall(int count,
                MPI_Request array_of_requests[],
                MPI_Status* array_of_statuses);

int MPI_Waitany(int count,
                MPI_Request array_of_requests[],
                int* index,
                MPI_Status* status);

int MPI_Win_allocate_shared(MPI_Aint size,
                            int disp_unit,
                            MPI_Info info,
                            MPI_Comm comm,
                            void* baseptr,
                            MPI_Win* win);

int MPI_Win_create(void* base,
                   MPI_Aint size,
                   int disp_unit,
                   MPI_Info info,
                   MPI_Comm comm,
                   MPI_Win* win);

int MPI_Win_fence(int assert, MPI_Win win);

int MPI_Win_free(MPI_Win* win);

int MPI_Win_get_attr(MPI_Win win,
                     int win_keyval,
                     void* attribute_val,
                     int* flag);

int MPI_Win_shared_query(MPI_Win win,
                         int rank,
                         MPI_Aint* size,
                         int* disp_unit,
                         void* baseptr);

double MPI_Wtime(void);

#ifdef __cplusplus
}
#endif

#endif //WASM_MPI_RS_MPI_H
