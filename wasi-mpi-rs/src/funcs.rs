use std::ptr::null_mut;
use std::time::Instant;

use libc::c_void;
use wasmer::{WasmPtr, Array};

use crate::{Env, MpiComm, MpiDatatype, MpiGroup, MpiOp};
use crate::consts::{MpiComparisonResult, MPI_COMM_NULL, MPI_SUCCESS};
use std::ops::Deref;

#[allow(non_snake_case)]
pub fn MPI_Abort(env: &Env, comm: i32, errorcode: i32) -> i32 {
    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");

    unsafe {
        mpi_sys::MPI_Abort(host_comm, errorcode)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Allgather(env: &Env, sendbuf: WasmPtr<u8>, sendcount: i32, sendtype: i32, recvbuf: WasmPtr<u8>, recvcount: i32, recvtype: i32, comm: i32) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Allgather");
    let sendbuf_ptr = sendbuf.deref(memory).expect("out-of-bounds ptr in MPI_Allgather");
    let recvbuf_ptr = recvbuf.deref(memory).expect("out-of-bounds ptr in MPI_Alltather");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_sendtype) = *mpi_datatypes.get(sendtype).expect("invalid sendtype");
    let MpiDatatype(host_recvtype) = *mpi_datatypes.get(recvtype).expect("invalid recvtype");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");

    unsafe {
        mpi_sys::MPI_Allgather(sendbuf_ptr.as_ptr() as *mut c_void, sendcount, host_sendtype, recvbuf_ptr.as_ptr() as *mut c_void, recvcount, host_recvtype, host_comm)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Alloc_mem(env: &Env, size: i32, info: i32, baseptr: WasmPtr<WasmPtr<u8>>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Alloc_mem");
    let malloc = env.malloc.get_ref().expect("Export malloc() to use MPI_Alloc_mem()");
    let derefed_ptr = baseptr.deref(memory).expect("out-of-bounds ptr in MPI_Alloc_mem");

    let alloced = malloc.call(size).expect("error calling malloc() in MPI_Alloc_mem()");
    derefed_ptr.set(alloced);
    return MPI_SUCCESS;
}


#[allow(non_snake_case)]
pub fn MPI_Allreduce(env: &Env, sendbuf: WasmPtr<u8>, recvbuf: WasmPtr<u8>, count: i32, datatype: i32, op: i32, comm: i32) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Allreduce");
    let sendbuf_ptr = sendbuf.deref(memory).expect("out-of-bounds ptr in MPI_Allreduce");
    let recvbuf_ptr = recvbuf.deref(memory).expect("out-of-bounds ptr in MPI_Allreduce");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_datatype) = *mpi_datatypes.get(datatype).expect("invalid datatype");

    let mpi_ops = env.mpi_ops.read().unwrap();
    let MpiOp(host_op) = *mpi_ops.get(op).expect("invalid op");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");

    unsafe {
        mpi_sys::MPI_Allreduce(sendbuf_ptr.as_ptr() as *mut c_void, recvbuf_ptr.as_ptr() as *mut c_void, count, host_datatype, host_op, host_comm)
    }
  
}


#[allow(non_snake_case)]
pub fn MPI_Alltoall(env: &Env, sendbuf: WasmPtr<u8>, sendcount: i32, sendtype: i32, recvbuf: WasmPtr<i32>, recvcount: i32, recvtype: i32, comm: i32) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Alltoall");
    let sendbuf_ptr = sendbuf.deref(memory).expect("out-of-bounds ptr in MPI_Alltoall");
    let recvbuf_ptr = recvbuf.deref(memory).expect("out-of-bounds ptr in MPI_Alltoall");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_sendtype) = *mpi_datatypes.get(sendtype).expect("invalid sendtype");
    let MpiDatatype(host_recvtype) = *mpi_datatypes.get(recvtype).expect("invalid recvtype");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");

    unsafe {
        mpi_sys::MPI_Alltoall(sendbuf_ptr.as_ptr() as *mut c_void, sendcount, host_sendtype, recvbuf_ptr.as_ptr() as *mut c_void, recvcount, host_recvtype, host_comm)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Alltoallv(env: &Env, sendbuf: WasmPtr<u8>, sendcounts: WasmPtr<i32>, sdispls: WasmPtr<i32>, sendtype: i32, recvbuf: WasmPtr<u8>, recvcounts: WasmPtr<i32>, rdispls: WasmPtr<i32>, recvtype: i32, comm: i32) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Alltoallv");
    let sendbuf_ptr = sendbuf.deref(memory).expect("out-of-bounds ptr in MPI_Alltoallv");
    let sendcounts_ptr = sendcounts.deref(memory).expect("out-of-bounds ptr in MPI_Alltoallv");
    let sdispls_ptr = sdispls.deref(memory).expect("out-of-bounds ptr in MPI_Alltoallv");
    let recvbuf_ptr = recvbuf.deref(memory).expect("out-of-bounds ptr in MPI_Alltoallv");
    let recvcounts_ptr = recvcounts.deref(memory).expect("out-of-bounds ptr in MPI_Alltoallv");
    let rdispls_ptr = rdispls.deref(memory).expect("out-of-bounds ptr in MPI_Alltoallv");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_sendtype) = *mpi_datatypes.get(sendtype).expect("invalid sendtype");
    let MpiDatatype(host_recvtype) = *mpi_datatypes.get(recvtype).expect("invalid recvtype");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");

    unsafe {
        mpi_sys::MPI_Alltoallv(
            sendbuf_ptr.as_ptr() as *mut c_void,
            sendcounts_ptr.as_ptr(),
            sdispls_ptr.as_ptr(),
            host_sendtype,
            recvbuf_ptr.as_ptr() as *mut c_void,
            recvcounts_ptr.as_ptr(),
            rdispls_ptr.as_ptr(),
            host_recvtype,
            host_comm,
        )
    }
}


#[allow(non_snake_case)]
pub fn MPI_Barrier(env: &Env, comm: i32) -> i32 {
    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid communicator");

    unsafe {
        mpi_sys::MPI_Barrier(host_comm)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Bcast(env: &Env, buffer: WasmPtr<u8>, count: i32, datatype: i32, root: i32, comm: i32) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Bcast");
    let buffer_ptr = buffer.deref(memory).expect("out-of-bounds ptr in MPI_Bcast");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_datatype) = *mpi_datatypes.get(datatype).expect("invalid datatype");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");

    unsafe {
        mpi_sys::MPI_Bcast(buffer_ptr.as_ptr() as *mut c_void, count, host_datatype, root, host_comm)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Comm_compare(env: &Env, comm1: i32, comm2: i32, result: WasmPtr<i32>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Comm_compare");
    let result_ptr = result.deref(memory).expect("out-of-bounds ptr in MPI_Comm_compare");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm1) = *mpi_comms.get(comm1).expect("invalid comm1");
    let MpiComm(host_comm2) = *mpi_comms.get(comm2).expect("invalid comm2");

    let mut host_result = -1;
    let host_result_ref: *mut i32 = &mut host_result;
    unsafe {
        mpi_sys::MPI_Comm_compare(host_comm1, host_comm2, host_result_ref);
    }

    let result = match host_result as u32 {
        mpi_sys::MPI_IDENT => MpiComparisonResult::MPI_IDENT,
        mpi_sys::MPI_CONGRUENT => MpiComparisonResult::MPI_CONGRUENT,
        mpi_sys::MPI_SIMILAR => MpiComparisonResult::MPI_SIMILAR,
        mpi_sys::MPI_UNEQUAL => MpiComparisonResult::MPI_UNEQUAL,
        _ => panic!("invalid comparison result"),
    };

    result_ptr.set(result as i32);
    return MPI_SUCCESS;
}

#[allow(non_snake_case)]
pub fn MPI_Comm_create(env: &Env, comm: i32, group: i32, newcomm: WasmPtr<i32>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Comm_create");
    let newcomm_ptr = newcomm.deref(memory).expect("out-of-bounds.ptr in MPI_Comm_create");

    let mut mpi_comms = env.mpi_comms.write().unwrap();
    let mpi_groups = env.mpi_groups.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");
    let MpiGroup(host_group) = *mpi_groups.get(group).expect("invalid group");

    let (newcomm_id, host_newcomm) = mpi_comms.alloc_instance();
    let host_newcomm_ref: *mut mpi_sys::MPI_Comm = &mut host_newcomm.0;

    let mpi_ret = unsafe {
        mpi_sys::MPI_Comm_create(
            host_comm,
            host_group,
            host_newcomm_ref,
        )
    };
    newcomm_ptr.set(newcomm_id);
    mpi_ret
}


#[allow(non_snake_case)]
pub fn MPI_Comm_free(env: &Env, comm: WasmPtr<i32>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Comm_free");
    let comm_ptr = comm.deref(memory).expect("out-of-bounds ptr in  MPI_Comm_free");
    let comm_id = comm_ptr.get();

    let mut mpi_comms = env.mpi_comms.write().unwrap();
    let mpi_comm = mpi_comms.get_mut(comm_id).expect("invalid comm");
    let host_comm_ref: *mut mpi_sys::MPI_Comm = &mut mpi_comm.0;

    unsafe {
        mpi_sys::MPI_Comm_free(host_comm_ref);
    }

    mpi_comms.free_instance(comm_id);
    return MPI_SUCCESS;
}


#[allow(non_snake_case)]
pub fn MPI_Comm_group(env: &Env, comm: i32, group: WasmPtr<i32>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Comm_group");
    let group_ptr = group.deref(memory).expect("out-of-bounds ptr in MPI_Comm_group");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid_comm");

    let mut mpi_groups = env.mpi_groups.write().unwrap();
    let (group_id, mpi_group) = mpi_groups.alloc_instance();
    let host_group_ref: *mut mpi_sys::MPI_Group = &mut mpi_group.0;

    unsafe {
        mpi_sys::MPI_Comm_group(host_comm, host_group_ref);
    }

    group_ptr.set(group_id);
    return MPI_SUCCESS;
}


#[allow(non_snake_case)]
pub fn MPI_Comm_split(env: &Env, comm: i32, color: i32, key: i32, newcomm: WasmPtr<i32>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Comm_split");
    let newcomm_ptr = newcomm.deref(memory).expect("out-of-bounds ptr in MPI_Comm_split");

    let mut mpi_comms = env.mpi_comms.write().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid_comm");

    let (newcomm_id, mpi_newcomm) = mpi_comms.alloc_instance();
    let host_newcomm_ref: *mut mpi_sys::MPI_Comm = &mut mpi_newcomm.0;

    let newcomm_is_null: bool;
    unsafe {
        mpi_sys::MPI_Comm_split(host_comm, color, key, host_newcomm_ref);
        newcomm_is_null = mpi_newcomm.0 == mpi_sys::RSMPI_COMM_NULL;
    }

    // Handle the case where color == MPI_UNDEFINED which will lead to newcomm being set to MPI_COMM_NULL
    if newcomm_is_null  {
        mpi_comms.free_instance(newcomm_id);
        newcomm_ptr.set(MPI_COMM_NULL);
    } else {
        newcomm_ptr.set(newcomm_id);
    }

    return MPI_SUCCESS;
}


#[allow(non_snake_case)]
pub fn MPI_Comm_rank(env: &Env, comm: i32, rank: WasmPtr<i32>) -> i32 {
    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid communicator");

    let memory = env.memory.get_ref().expect("uninitialized memory in wasm_MPI_Comm_rank");
    let derefed_ptr = rank.deref(memory).expect("out-of-bounds ptr in wasm_MPI_Comm_rank");

    unsafe {
        mpi_sys::MPI_Comm_rank(host_comm, derefed_ptr.as_ptr())
    }
}


#[allow(non_snake_case)]
pub fn MPI_Comm_size(env: &Env, comm: i32, size: WasmPtr<i32>) -> i32 {
    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid communicator");

    let memory = env.memory.get_ref().expect("uninitialized memory in wasm_MPI_Comm_size");
    let derefed_ptr = size.deref(memory).expect("out-of-bounds ptr in wasm_MPI_Comm_size");

    unsafe {
        mpi_sys::MPI_Comm_size(host_comm, derefed_ptr.as_ptr())
    }
}


#[allow(non_snake_case)]
pub fn MPI_Finalize(env: &Env) -> i32 {
    unsafe {
        mpi_sys::MPI_Finalize()
    }
}


#[allow(non_snake_case)]
pub fn MPI_Free_mem(env: &Env, base: WasmPtr<u8>) -> i32 {
    let free = env.free.get_ref().expect("Export free() to use MPI_Free_mem()");

    free.call(base).expect("error calling free() in MPI_Free_mem()");
    return MPI_SUCCESS;
}


#[allow(non_snake_case)]
pub fn MPI_Gather(env:&Env, sendbuf: WasmPtr<u8>, sendcount: i32, sendtype: i32, recvbuf: WasmPtr<u8>, recvcount: i32, recvtype: i32, root: i32, comm: i32) -> i32 {
    let memory = env.memory.get_ref().expect("unintialized memory in MPI_Gather");
    let sendbuf_ptr = sendbuf.deref(memory).expect("out-of-bounds ptr in MPI_Gather");
    let recvbuf_ptr = recvbuf.deref(memory).expect("out-of-bounds ptr in MPI_Gather");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_sendtype) = *mpi_datatypes.get(sendtype).expect("invalid sendtype");
    let MpiDatatype(host_recvtype) = *mpi_datatypes.get(recvtype).expect("invalid recvtype");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");

    unsafe {
        mpi_sys::MPI_Gather(sendbuf_ptr.as_ptr() as *mut c_void, sendcount, host_sendtype, recvbuf_ptr.as_ptr() as *mut c_void, recvcount, host_recvtype, root, host_comm)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Get_count(env: &Env, status: WasmPtr<u8>, datatype: i32, count: WasmPtr<i32>) -> i32 {
    // TODO: Figure out a cleaner way to handle MPI_Status
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Get_count");
    let status_ptr = status.deref(memory).expect("out-of-bounds ptr in MPI_Get_count");
    let count_ptr = count.deref(memory).expect("out-of-bounds ptr in  MPI_Get count");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_datatype) = *mpi_datatypes.get(datatype).expect("invalid datatype");

    unsafe {
        mpi_sys::MPI_Get_count(status_ptr.as_ptr() as *mut mpi_sys::MPI_Status, host_datatype, count_ptr.as_ptr())
    }
}

#[allow(non_snake_case)]
pub fn MPI_Group_free(env: &Env, group: WasmPtr<i32>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Group_free");
    let group_ptr = group.deref(memory).expect("out-of-bounds ptr in MPI_Group_free");
    let group_id = group_ptr.get();

    let mut mpi_groups = env.mpi_groups.write().unwrap();
    let mpi_group = mpi_groups.get_mut(group_id).expect("invalid group");
    let host_group_ptr: *mut mpi_sys::MPI_Group = &mut mpi_group.0;

    let mpi_ret = unsafe {
        mpi_sys::MPI_Group_free(host_group_ptr)
    };
    mpi_groups.free_instance(group_id);
    mpi_ret
}

#[allow(non_snake_case)]
pub fn MPI_Group_range_incl(
    env: &Env,
    group: i32,
    n: i32,
    ranges: WasmPtr<i32, Array>,  // int ranges[][3]
    newgroup: WasmPtr<i32>,
) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Group_range_incl");
    let ranges_slice = ranges.deref(memory, 0, (3*n) as u32).expect("out-of-bounds ptr in MPI_Group_range_incl");
    let newgroup_ptr = newgroup.deref(memory).expect("out-of-bounds ptr in MPI_Group_range_incl");

    let mut mpi_groups = env.mpi_groups.write().unwrap();
    let MpiGroup(host_group) = *mpi_groups.get(group).expect("invalid group");

    let (newgroup_id, host_newgroup) = mpi_groups.alloc_instance();
    let host_newgroup_ptr: *mut mpi_sys::MPI_Group = &mut host_newgroup.0;

    let mpi_ret = unsafe {
        mpi_sys::MPI_Group_range_incl(
            host_group,
            n,
            ranges_slice.as_ptr() as *mut [i32; 3],
            host_newgroup_ptr,
        )
    };

    newgroup_ptr.set(newgroup_id);
    mpi_ret
}

#[allow(non_snake_case)]
pub fn MPI_Group_translate_ranks(env: &Env, group1: i32, n: i32, ranks1: WasmPtr<i32>, group2: i32, ranks2: WasmPtr<i32>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Group_translate_ranks");
    let ranks1_ptr = ranks1.deref(memory).expect("out-of-bounds ptr in MPI_Group_translate_ranks");
    let ranks2_ptr = ranks2.deref(memory).expect("out-of-bounds ptr in MPI_Group_translate_ranks");

    let mpi_groups = env.mpi_groups.read().unwrap();
    let MpiGroup(host_group1) = *mpi_groups.get(group1).expect("invalid group1");
    let MpiGroup(host_group2) = *mpi_groups.get(group2).expect("invalid group2");

    unsafe {
        mpi_sys::MPI_Group_translate_ranks(host_group1, n, ranks1_ptr.as_ptr(), host_group2, ranks2_ptr.as_ptr())
    }
}


#[allow(non_snake_case)]
pub fn MPI_Init(env: &Env, _argc: i32, _argv: i32) -> i32 {
    unsafe {
        // OpenMPI does not use argc and argv so just pass nullptr to them
        mpi_sys::MPI_Init(null_mut(), null_mut())
    }
}


#[allow(non_snake_case)]
pub fn MPI_Initialized(env: &Env, flag: WasmPtr<i32>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Initialized");
    let flag_ptr = flag.deref(memory).expect("out-of-bounds ptr in MPI_Initialized");

    unsafe {
        mpi_sys::MPI_Initialized(flag_ptr.as_ptr())
    }
}


#[allow(non_snake_case)]
pub fn MPI_Irecv(env: &Env, buf: WasmPtr<u8>, count: i32, datatype: i32, source: i32, tag: i32, comm: i32, request: WasmPtr<i32>) -> i32 {
    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid communicator");
    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_datatype) = *mpi_datatypes.get(datatype).expect("invalid datatype");

    let memory = env.memory.get_ref().expect("uninitialized memory in wasm_MPI_Irecv");
    let buf_ptr = buf.deref(memory).expect("out-of-bounds ptr in wasm_MPI_Irecv");
    let request_ptr = request.deref(memory).expect("out-of-bounds ptr in wasm_MPI_Irecv");

    let mut mpi_requests = env.mpi_requests.write().unwrap();
    let (request_id, mpi_request) = mpi_requests.alloc_instance();
    let host_request_ptr: *mut mpi_sys::MPI_Request = &mut mpi_request.0;

    request_ptr.set(request_id);
    unsafe {
        mpi_sys::MPI_Irecv(buf_ptr.as_ptr() as *mut c_void, count, host_datatype, source, tag, host_comm, host_request_ptr)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Isend(env: &Env, buf: WasmPtr<u8>, count: i32, datatype: i32, dest: i32, tag: i32, comm: i32, request: WasmPtr<i32>) -> i32 {
    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid communicator");
    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_datatype) = *mpi_datatypes.get(datatype).expect("invalid datatype");

    let memory = env.memory.get_ref().expect("uninitialized memory in wasm_MPI_Isend");
    let buf_ptr = buf.deref(memory).expect("out-of-bounds ptr in wasm_MPI_Isend");
    let request_ptr = request.deref(memory).expect("out-of-bounds ptr in wasm_MPI_Isend");

    let mut mpi_requests = env.mpi_requests.write().unwrap();
    let (request_id, mpi_request) = mpi_requests.alloc_instance();
    let host_request_ptr: *mut mpi_sys::MPI_Request = &mut mpi_request.0;

    request_ptr.set(request_id);
    unsafe {
        mpi_sys::MPI_Isend(buf_ptr.as_ptr() as *mut c_void, count, host_datatype, dest, tag, host_comm, host_request_ptr)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Op_create(env: &Env, function: i32, commute: i32, op: WasmPtr<i32>) -> i32 {
    panic!("MPI_Op_create is not supported yet");
}


#[allow(non_snake_case)]
pub fn MPI_Recv(env: &Env, buf: WasmPtr<u8>, count: i32, datatype: i32, source: i32, tag: i32, comm: i32, status: WasmPtr<i32>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Recv");
    let buf_ptr = buf.deref(memory).expect("out-of-bounds ptr in MPI_Recv");
    let status_ptr = status.deref(memory).expect("out-of-bounds ptr in MPI_Recv");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_datatype) = *mpi_datatypes.get(datatype).expect("invalid datatype");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");

    unsafe {
        mpi_sys::MPI_Recv(buf_ptr.as_ptr() as *mut c_void, count, host_datatype, source, tag, host_comm, status_ptr.as_ptr() as *mut mpi_sys::MPI_Status)
    }

}


#[allow(non_snake_case)]
pub fn MPI_Reduce(env: &Env, sendbuf: WasmPtr<u8>, recvbuf: WasmPtr<u8>, count: i32, datatype: i32, op: i32, root: i32, comm: i32) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Reduce");
    let sendbuf_ptr = sendbuf.deref(memory).expect("out-of-bounds ptr in MPI_Reduce");
    let recvbuf_ptr = recvbuf.deref(memory).expect("out-of-bounds ptr in MPI_Reduce");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_datatype) = *mpi_datatypes.get(datatype).expect("invalid datatype");

    let mpi_ops = env.mpi_ops.read().unwrap();
    let MpiOp(host_op) = *mpi_ops.get(op).expect("invalid op");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");

    unsafe {
        mpi_sys::MPI_Reduce(sendbuf_ptr.as_ptr() as *mut c_void, recvbuf_ptr.as_ptr() as *mut c_void, count, host_datatype, host_op, root, host_comm)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Scatter(env: &Env, sendbuf: WasmPtr<u8>, sendcount: i32, sendtype: i32, recvbuf: WasmPtr<u8>, recvcount: i32, recvtype: i32, root: i32, comm: i32) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Scatter");
    let sendbuf_ptr = sendbuf.deref(memory).expect("out-of-bounds ptr in MPI_Scatter");
    let recvbuf_ptr = recvbuf.deref(memory).expect("out-of-bounds ptr in MPI_Scatter");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_sendtype) = *mpi_datatypes.get(sendtype).expect("invalid sendtype");
    let MpiDatatype(host_recvtype) = *mpi_datatypes.get(recvtype).expect("invalid recvtype");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");

    unsafe {
        mpi_sys::MPI_Scatter(sendbuf_ptr.as_ptr() as *mut c_void, sendcount, host_sendtype, recvbuf_ptr.as_ptr() as *mut c_void, recvcount, host_recvtype, root, host_comm)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Send(env: &Env, buf: WasmPtr<u8>, count: i32, datatype: i32, dest: i32, tag: i32, comm: i32) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Send");
    let buf_ptr = buf.deref(memory).expect("out-of-bounds ptr in MPI_Send");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_datatype) = *mpi_datatypes.get(datatype).expect("invalid datatype");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(host_comm) = *mpi_comms.get(comm).expect("invalid comm");
    
    unsafe {
        mpi_sys::MPI_Send(buf_ptr.as_ptr() as *mut c_void, count, host_datatype, dest, tag, host_comm)
    }    
}


#[allow(non_snake_case)]
pub fn MPI_Sendrecv(env: &Env, sendbuf: WasmPtr<u8>, sendcount: i32, sendtype: i32, dest: i32, sendtag: i32, recvbuf: WasmPtr<u8>, recvcount: i32, recvtype: i32, source: i32, recvtag: i32, comm: i32, status: WasmPtr<u8>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Sendrecv");
    let sendbuf_ptr = sendbuf.deref(memory).expect("out-of-bounds ptr in MPI_Sendrecv");
    let recvbuf_ptr = recvbuf.deref(memory).expect("out-of-bounds ptr in MPI_Sendrecv");
    let status_ptr = status.deref(memory).expect("out-of-bounds ptr in MPI_Sendrecv");

    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(sendtype_host) = *mpi_datatypes.get(sendtype).expect("invalid sendtype");
    let MpiDatatype(recvtype_host) = *mpi_datatypes.get(recvtype).expect("invalid recvtype");

    let mpi_comms = env.mpi_comms.read().unwrap();
    let MpiComm(comm_host) = *mpi_comms.get(comm).expect("invalid_comm");

    unsafe {
        mpi_sys::MPI_Sendrecv(sendbuf_ptr.as_ptr() as *mut c_void, sendcount, sendtype_host, dest, sendtag, recvbuf_ptr.as_ptr() as *mut c_void, recvcount, recvtype_host, source, recvtag, comm_host, status_ptr.as_ptr() as *mut mpi_sys::MPI_Status)
    }
}


#[allow(non_snake_case)]
pub fn MPI_Type_free(env: &Env, datatype: WasmPtr<i32>) -> i32 {
    panic!("MPI_Type_free is not supported yet");
}


#[allow(non_snake_case)]
pub fn MPI_Type_size(env: &Env, datatype: i32, size: WasmPtr<i32>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in MPI_Type_size");
    let size_ptr = size.deref(memory).expect("out-of-bounds ptr in MPI_Type_size");
    let mpi_datatypes = env.mpi_datatypes.read().unwrap();
    let MpiDatatype(host_datatype) = *mpi_datatypes.get(datatype).expect("invalid datatype");

    unsafe {
        mpi_sys::MPI_Type_size(host_datatype, size_ptr.as_ptr())
    }
}


#[allow(non_snake_case)]
pub fn MPI_Wait(env: &Env, request: WasmPtr<i32>, status: WasmPtr<u64>) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in wasm_MPI_Wait");
    let request_ptr = request.deref(memory).expect("out-of-bounds ptr in wasm_MPI_Wait");
    let status_ptr = status.deref(memory).expect("out-of-bounds ptr in wasm_MPI_Wait");

    let request_id = request_ptr.get();
    let mut mpi_requests = env.mpi_requests.write().unwrap();
    let mpi_request = mpi_requests.get_mut(request_id).expect("invalid request");
    let host_request_ptr: *mut mpi_sys::MPI_Request = &mut mpi_request.0;

    let mpi_retval = unsafe {
        mpi_sys::MPI_Wait(host_request_ptr, status_ptr.as_ptr() as *mut mpi_sys::MPI_Status)
    };

    mpi_requests.free_instance(request_id);
    mpi_retval
}


#[allow(non_snake_case)]
pub fn MPI_Waitall(env: &Env, count: i32, array_of_requests: WasmPtr<i32>, array_of_statuses: WasmPtr<i32>) -> i32 {
    panic!("MPI_Waitall is not supported yet");
}


#[allow(non_snake_case)]
pub fn MPI_Wtime(env: &Env) -> f64 {
    unsafe {
        mpi_sys::MPI_Wtime()
    }
}
