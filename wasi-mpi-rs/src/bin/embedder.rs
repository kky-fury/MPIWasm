use std::fs;
use std::time::Instant;

use anyhow::{bail, Result};
use reqwest;
use serde::{Deserialize, Serialize};
use structopt::{StructOpt};
use wasmer::{ExportType, Function, ImportType, Instance, Module, namespace, Store};
use wasmer_cache::{Cache, FileSystemCache, Hash};
use wasmer_compiler_llvm::LLVM;
use wasmer_engine_dylib::Dylib;
use wasmer_wasi::WasiState;

use wasm_mpi_rs;
use wasm_mpi_rs::Env;


#[derive(StructOpt)]
#[structopt(name = "embedder", about = "Run WASI-MPI modules")]
#[structopt(setting = structopt::clap::AppSettings::AllowLeadingHyphen)]
struct Opt {
    #[structopt(short, long)]
    pub timings: bool,
    #[structopt(short, long)]
    #[structopt(number_of_values = 1)]
    pub dir: Vec<String>,
    #[structopt(short, long)]
    pub callback: Option<String>,
    #[structopt(name = "MODULE_PATH")]
    pub module_path: String,
    #[structopt(name = "MODULE_ARGS")]
    pub module_args: Vec<String>,
}


#[derive(Copy, Clone, Deserialize, PartialEq, Serialize)]
enum JobState {
    Submitted,
    Running,
    Failed,
    Completed,
}


#[derive(Clone, Deserialize, Serialize)]
struct JobCallback {
    pub state: JobState,
}


fn main() -> Result<()> {
    let opt = Opt::from_args();
    let http_client = reqwest::blocking::Client::builder().no_proxy().build()?;

    // Create a new file system cache.
    let mut fs_cache = FileSystemCache::new("./cache")?;

    let engine = Dylib::new(LLVM::new()).engine();
    let store = Store::new(&engine);

    let wasm_bytes = fs::read(&opt.module_path)?;
    let hash = Hash::generate(&wasm_bytes);

    let module = unsafe {
        fs_cache.load(&store, hash).unwrap_or_else(|_| {
            let time_before = Instant::now();
            let module = Module::from_binary(&store, &wasm_bytes)
                .expect("Could not compile .wasm binary");
            let time_after = Instant::now();
            if opt.timings {
                println!("Compile took {}ms", (time_after - time_before).as_millis())
            }

            fs_cache.store(hash, &module);
            module
        })
    };

    let imports: Vec<ImportType> = module.imports().collect();
    let exports: Vec<ExportType> = module.exports().collect();

    let mut wasi_state = WasiState::new(&opt.module_path);
    wasi_state.args(&opt.module_args);
    for dir in &opt.dir {
        wasi_state.preopen_dir(dir)?;
    }

    let mut wasi_env = wasi_state.finalize()?;
    let mut import_object = wasi_env.import_object(&module)?;

    let env = Env::new();
    let mpi_imports = namespace! {
        "__cxa_allocate_exception" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::wasm__cxa_allocate_exception),
        "__cxa_throw" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::wasm__cxa_throw),
        "gethostname" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::gethostname),
        "MPI_Abort" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Abort),
        "MPI_Allgather" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Allgather),
        "MPI_Alloc_mem" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Alloc_mem),
        "MPI_Allreduce" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Allreduce),
        "MPI_Alltoall" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Alltoall),
        "MPI_Alltoallv" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Alltoallv),
        "MPI_Barrier" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Barrier),
        "MPI_Bcast" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Bcast),
        "MPI_Comm_compare" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Comm_compare),
        "MPI_Comm_create" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Comm_create),
        "MPI_Comm_free" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Comm_free),
        "MPI_Comm_group" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Comm_group),
        "MPI_Comm_split" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Comm_split),
        "MPI_Comm_rank" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Comm_rank),
        "MPI_Comm_size" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Comm_size),
        "MPI_Finalize" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Finalize),
        "MPI_Free_mem" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Free_mem),
        "MPI_Gather" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Gather),
        "MPI_Get_count" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Get_count),
        "MPI_Group_free" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Group_free),
        "MPI_Group_range_incl" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Group_range_incl),
        "MPI_Group_translate_ranks" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Group_translate_ranks),
        "MPI_Init" => Function::new_native_with_env(&store, env.clone(),wasm_mpi_rs::MPI_Init),
        "MPI_Initialized" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Initialized),
        "MPI_Irecv" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Irecv),
        "MPI_Isend" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Isend),
        "MPI_Op_create" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Op_create),
        "MPI_Recv" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Recv),
        "MPI_Reduce" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Reduce),
        "MPI_Scatter" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Scatter),
        "MPI_Send" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Send),
        "MPI_Sendrecv" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Sendrecv),
        "MPI_Type_free" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Type_free),
        "MPI_Type_size" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Type_size),
        "MPI_Wait" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Wait),
        "MPI_Waitall" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Waitall),
        "MPI_Wtime" => Function::new_native_with_env(&store, env.clone(), wasm_mpi_rs::MPI_Wtime),
    };
    import_object.register("env", mpi_imports);

    let instance = Instance::new(&module, &import_object)?;
    let start = instance.exports.get_function("_start")?;

    if let Some(callback) = &opt.callback {
        let job_callback = JobCallback{ state: JobState::Running };
        http_client.put(callback).json(&job_callback).send()?.error_for_status()?;
    }
    start.call(&[])?;

    if let Some(callback) = &opt.callback {
        let job_callback = JobCallback{ state: JobState::Completed };
        http_client.put(callback).json(&job_callback).send()?.error_for_status()?;
    }
    Ok(())
}
