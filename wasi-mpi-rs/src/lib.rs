use std::collections::HashMap;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};

use anyhow::Result;
use mpi_sys;
use sys_info;
use wasmer::{LazyInit, Memory, NativeFunc, WasmerEnv, WasmPtr, Array};

pub use consts::*;
pub use funcs::*;

pub mod consts;
pub mod funcs;

#[allow(non_snake_case)]
pub fn gethostname(env: &Env, name: WasmPtr<u8, Array>, len: i32) -> i32 {
    let memory = env.memory.get_ref().expect("uninitialized memory in gethostname");
    let name_slice = name.deref(memory, 0, len as u32).expect("out-of-bounds ptr in gethostname");

    let hostname = sys_info::hostname().expect("failed to get hostname");
    let hostname_bytes = hostname.as_bytes();
    if hostname_bytes.len() > len as usize {
        panic!("provided buffer too small for hostname")
    }

    for i in 0..hostname_bytes.len() {
        name_slice[i].set(hostname_bytes[i]);
    }

    0
}


#[allow(non_snake_case)]
pub fn wasm__cxa_allocate_exception(env: &Env, size: i32) -> i32 {
    panic!("__cxa_allocate_exception called, aborting");
}


#[allow(non_snake_case)]
pub fn wasm__cxa_throw(env: &Env, _ptr: i32, _ty: i32, _destructor: i32) {
    panic!("__cxa_throw called, aborting");
}

#[derive(Clone)]
pub struct MpiComm(mpi_sys::MPI_Comm);
impl Default for MpiComm {
    #[cfg(feature = "mvapich")]
    fn default() -> Self {
        Self(0)
    }
    #[cfg(feature = "openmpi")]
    fn default() -> Self {
        Self(null_mut())
    }
}
unsafe impl Send for MpiComm {}
unsafe impl Sync for MpiComm {}

#[derive(Clone)]
pub struct MpiDatatype(mpi_sys::MPI_Datatype);
impl Default for MpiDatatype {
    #[cfg(feature = "mvapich")]
    fn default() -> Self {
        Self(0)
    }
    #[cfg(feature = "openmpi")]
    fn default() -> Self {
        Self(null_mut())
    }
}
unsafe impl Send for MpiDatatype {}
unsafe impl Sync for MpiDatatype {}


#[derive(Clone)]
pub struct MpiGroup(mpi_sys::MPI_Group);
impl Default for MpiGroup {
    #[cfg(feature = "mvapich")]
    fn default() -> Self {
        Self(0)
    }
    #[cfg(feature = "openmpi")]
    fn default() -> Self {
        Self(null_mut())
    }
}
unsafe impl Send for MpiGroup {}
unsafe impl Sync for MpiGroup {}


#[derive(Clone)]
pub struct MpiOp(mpi_sys::MPI_Op);
impl Default for MpiOp {
    #[cfg(feature = "mvapich")]
    fn default() -> Self {
        Self(0)
    }
    #[cfg(feature = "openmpi")]
    fn default() -> Self {
        Self(null_mut())
    }
}
unsafe impl Send for MpiOp {}
unsafe impl Sync for MpiOp {}


#[derive(Clone)]
pub struct MpiRequest(mpi_sys::MPI_Request);
impl Default for MpiRequest {
    #[cfg(feature = "mvapich")]
    fn default() -> Self {
        Self(0)
    }
    #[cfg(feature = "openmpi")]
    fn default() -> Self {
        Self(null_mut())
    }
}
unsafe impl Send for MpiRequest {}
unsafe impl Sync for MpiRequest {}


#[derive(Clone)]
pub struct MpiTranslation<HostType> {
    next_id: i32,
    instances: HashMap<i32, HostType>,
}
impl<HostType> MpiTranslation<HostType> where HostType : Default {
    pub fn new() -> MpiTranslation<HostType> {
        let instances: HashMap<i32, HostType> = HashMap::new();

        MpiTranslation{
            next_id: 0,
            instances: instances,
        }
    }

    pub fn get(&self, instance_id: i32) -> Option<&HostType> {
        self.instances.get(&instance_id)
    }

    pub fn get_mut(&mut self, instance_id: i32) -> Option<&mut HostType> {
        self.instances.get_mut(&instance_id)
    }

    pub fn alloc_instance(&mut self) -> (i32, &mut HostType) {
        let instance = HostType::default();

        let instance_id = self.next_id;
        self.instances.insert(instance_id, instance);
        self.next_id += 1;

        let instance_ref = self.instances.get_mut(&instance_id).expect("missing key directly after insert");
        (instance_id, instance_ref)
    }

    pub fn free_instance(&mut self, instance_id: i32) -> () {
        self.instances.remove(&instance_id);
    }
}
impl Default for MpiTranslation<MpiComm> {
    fn default() -> Self {
        let mut instances: HashMap<i32, MpiComm> = HashMap::new();

        unsafe {
            instances.insert(self::MPI_COMM_WORLD, MpiComm(mpi_sys::RSMPI_COMM_WORLD));
            instances.insert(self::MPI_COMM_SELF, MpiComm(mpi_sys::RSMPI_COMM_SELF));
            instances.insert(self::MPI_COMM_NULL, MpiComm(mpi_sys::RSMPI_COMM_NULL));
        }

        Self{
            next_id: instances.len() as i32,
            instances: instances,
        }
    }
}
impl Default for MpiTranslation<MpiDatatype> {
    fn default() -> Self {
        let mut instances: HashMap<i32, MpiDatatype> = HashMap::new();

        unsafe {
            instances.insert(self::MPI_INT8_T, MpiDatatype(mpi_sys::RSMPI_INT8_T));
            instances.insert(self::MPI_INT16_T, MpiDatatype(mpi_sys::RSMPI_INT16_T));
            instances.insert(self::MPI_INT32_T, MpiDatatype(mpi_sys::RSMPI_INT32_T));
            instances.insert(self::MPI_INT, MpiDatatype(mpi_sys::RSMPI_INT32_T));
            instances.insert(self::MPI_INT64_T, MpiDatatype(mpi_sys::RSMPI_INT64_T));
            instances.insert(self::MPI_UINT8_T, MpiDatatype(mpi_sys::RSMPI_UINT8_T));
            instances.insert(self::MPI_UINT16_T, MpiDatatype(mpi_sys::RSMPI_UINT16_T));
            instances.insert(self::MPI_UINT32_T, MpiDatatype(mpi_sys::RSMPI_UINT32_T));
            instances.insert(self::MPI_UINT_T, MpiDatatype(mpi_sys::RSMPI_UINT32_T));
            instances.insert(self::MPI_UINT64_T, MpiDatatype(mpi_sys::RSMPI_UINT64_T));
            instances.insert(self::MPI_LONG, MpiDatatype(mpi_sys::RSMPI_INT64_T));
            instances.insert(self::MPI_LONG_LONG, MpiDatatype(mpi_sys::RSMPI_INT64_T));
            instances.insert(self::MPI_LONG_LONG_INT, MpiDatatype(mpi_sys::RSMPI_INT64_T));
            instances.insert(self::MPI_FLOAT, MpiDatatype(mpi_sys::RSMPI_FLOAT));
            instances.insert(self::MPI_DOUBLE, MpiDatatype(mpi_sys::RSMPI_DOUBLE));
            instances.insert(self::MPI_DOUBLE_INT, MpiDatatype(mpi_sys::RSMPI_DOUBLE));
            instances.insert(self::MPI_CHAR, MpiDatatype(mpi_sys::RSMPI_UINT8_T));
            instances.insert(self::MPI_C_BOOL, MpiDatatype(mpi_sys::RSMPI_UINT8_T));
            instances.insert(self::MPI_BYTE, MpiDatatype(mpi_sys::RSMPI_UINT8_T));
            instances.insert(self::MPI_DATATYPE_NULL, MpiDatatype(mpi_sys::RSMPI_DATATYPE_NULL));
        }

        Self {
            next_id: (instances.len() - 1) as i32,
            instances,
        }
    }
}
impl Default for MpiTranslation<MpiOp> {
    fn default() -> Self {
        let mut instances: HashMap<i32, MpiOp> = HashMap::new();

        unsafe {
            instances.insert(self::MPI_MAX, MpiOp(mpi_sys::RSMPI_MAX));
            instances.insert(self::MPI_MIN, MpiOp(mpi_sys::RSMPI_MIN));
            instances.insert(self::MPI_SUM, MpiOp(mpi_sys::RSMPI_SUM));
            instances.insert(self::MPI_PROD, MpiOp(mpi_sys::RSMPI_PROD));
            instances.insert(self::MPI_LAND, MpiOp(mpi_sys::RSMPI_LAND));
            instances.insert(self::MPI_LOR, MpiOp(mpi_sys::RSMPI_LOR));
            instances.insert(self::MPI_BAND, MpiOp(mpi_sys::RSMPI_BAND));
            instances.insert(self::MPI_BOR, MpiOp(mpi_sys::RSMPI_BOR));
            // instances.insert(self::MPI_MAXLOC, MpiOp(mpi_sys::MPI_MAXLOC));
            // instances.insert(self::MPI_MINLOC, MpiOp(mpi_sys::MPI_MINLOC));
            // instances.insert(self::MPI_OP_NULL, MpiOp(mpi_sys::RSMPI_OP_NULL));
        }

        Self {
            next_id: (instances.len() + 2) as i32,
            instances,
        }
    }
}


#[derive(Clone, WasmerEnv)]
pub struct Env {
    #[wasmer(export)]
    pub memory: LazyInit<Memory>,
    #[wasmer(export(optional = true))]
    pub malloc: LazyInit<NativeFunc<i32, WasmPtr<u8>>>,
    #[wasmer(export(optional = true))]
    pub free: LazyInit<NativeFunc<WasmPtr<u8>, ()>>,
    pub mpi_comms: Arc<RwLock<MpiTranslation<MpiComm>>>,
    pub mpi_datatypes: Arc<RwLock<MpiTranslation<MpiDatatype>>>,
    pub mpi_groups: Arc<RwLock<MpiTranslation<MpiGroup>>>,
    pub mpi_ops: Arc<RwLock<MpiTranslation<MpiOp>>>,
    pub mpi_requests: Arc<RwLock<MpiTranslation<MpiRequest>>>,
}
impl Env {
    pub fn new() -> Env {
        Env{
            memory: LazyInit::new(),
            malloc: LazyInit::new(),
            free: LazyInit::new(),
            mpi_comms: Arc::new(RwLock::new(MpiTranslation::default())),
            mpi_datatypes: Arc::new(RwLock::new(MpiTranslation::default())),
            mpi_groups: Arc::new(RwLock::new(MpiTranslation::new())),
            mpi_ops: Arc::new(RwLock::new(MpiTranslation::default())),
            mpi_requests: Arc::new(RwLock::new(MpiTranslation::new())),
        }
    }
}
