use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;

use actix_web::{App, get, HttpResponse, HttpServer, post, put, Responder};
use actix_web::middleware::Logger;
use actix_web::web;
use anyhow::{bail, Result};
use futures::join;
use log4rs;
use log::info;
use mpi_sys;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use settings::Settings;

mod settings;

#[derive(Copy, Clone, Deserialize, PartialEq, Serialize)]
enum JobState {
    Submitted,
    Running,
    Failed,
    Completed,
}


#[derive(Clone, Deserialize, Serialize)]
struct Job {
    pub uuid: Uuid,
    pub path: String,
    pub argv: Vec<String>,
    pub world_size: i32,
    pub state: JobState,
    pub callback: String,
}
impl Job {
    fn new(job_post: JobPost, uuid: Uuid, callback: String) -> Self {Job{
        uuid: uuid,
        path: job_post.path,
        argv: job_post.argv,
        world_size: job_post.world_size,
        state: JobState::Submitted,
        callback: callback,
    }}
}


#[derive(Clone, Deserialize, Serialize)]
struct JobPost {
    pub path: String,
    pub argv: Vec<String>,
    pub world_size: i32,
}


#[derive(Clone, Deserialize, Serialize)]
struct JobCallback {
    pub state: JobState,
}


#[derive(Clone)]
struct AppState {
    pub settings: Settings,
    pub mpi_free_slots: Arc<RwLock<i32>>,
    pub mpi_tx: Sender<Job>,
    pub jobs: Arc<RwLock<HashMap<Uuid, Job>>>,
}


fn mpi_closure(rx: Receiver<Job>) -> impl FnOnce() -> Result<()> {
    move || {
        for job in rx {
            mpi_handler(&job)?;
        }
        Ok(())
    }
}


fn mpi_handler(job: &Job) -> Result<()> {
    let time_before = Instant::now();

    unsafe {
        let command = CString::new("target/release/embedder").unwrap();

        let mut argv = vec![
            "--callback",
            job.callback.as_str(),
            job.path.as_str(),
        ];
        argv.extend(job.argv.iter().map(
            |x| x.as_str()
        ));

        let argv_cstrings: Vec<CString> = argv.iter().map(
            |x| CString::new(*x).unwrap()
        ).collect();
        let mut argv_ptrs: Vec<*mut c_char> = argv_cstrings.iter().map(
            |x| x.as_ptr() as *mut c_char
        ).collect();
        argv_ptrs.push(null_mut());

        #[cfg(feature = "mvapich")]
        let mut info: mpi_sys::MPI_Info = 0;
        #[cfg(feature = "openmpi")]
        let mut info: mpi_sys::MPI_Info = null_mut();
        mpi_sys::MPI_Info_create(&mut info);

        #[cfg(feature = "mvapich")]
        let mut intercomm: mpi_sys::MPI_Comm = 0;
        #[cfg(feature = "openmpi")]
        let mut intercomm: mpi_sys::MPI_Comm = null_mut();
        let mut errcodes: Vec<i32> = vec![-1; job.world_size as usize];

        mpi_sys::MPI_Comm_spawn(
            command.as_ptr(),
            argv_ptrs.as_mut_ptr(),
            job.world_size,
            info,
            0,
            mpi_sys::RSMPI_COMM_WORLD,
            &mut intercomm,
            errcodes.as_mut_ptr(),
        );
    }

    let time_after = Instant::now();
    let time_delta = time_after - time_before;
    info!("Job {} spawned in {}ms", job.uuid, time_delta.as_millis());
    Ok(())
}


#[get("/jobs")]
async fn get_jobs(app_state: web::Data<AppState>) -> impl Responder {
    let jobs = app_state.jobs.read().unwrap();
    let jobs_vec: Vec<&Job> = jobs.values().collect();

    HttpResponse::Ok().json(jobs_vec)
}


#[post("/jobs")]
async fn post_jobs(job_post: web::Json<JobPost>, app_state: web::Data<AppState>) -> impl Responder {
    let mut mpi_free_slots = app_state.mpi_free_slots.write().unwrap();
    let mut jobs = app_state.jobs.write().unwrap();

    let uuid = Uuid::new_v4();
    let callback = format!(
        "http://{}/jobs/{}/callback",
        &app_state.settings.host,
        &uuid,
    );
    let j = Job::new(job_post.into_inner(), uuid, callback);
    if *mpi_free_slots < j.world_size {
        HttpResponse::BadRequest().json(format!(
            "Cannot start job with world_size {}: Only {} slots free",
            j.world_size,
            mpi_free_slots,
        ))
    } else {
        *mpi_free_slots -= j.world_size;
        jobs.insert(j.uuid, j.clone());
        app_state.mpi_tx.send(j.clone()).unwrap();

        HttpResponse::Ok().json(j)
    }
}


#[put("/jobs/{uuid}/callback")]
async fn put_jobs_callback(uuid: web::Path<Uuid>, job_callback: web::Json<JobCallback>, app_state: web::Data<AppState>) -> impl Responder {
    let mut mpi_free_slots = app_state.mpi_free_slots.write().unwrap();
    let mut jobs = app_state.jobs.write().unwrap();

    if let Some(j) = jobs.get_mut(&uuid) {
        if j.state == job_callback.state {
            HttpResponse::NoContent().body("")
        } else {
            if j.state == JobState::Failed || j.state == JobState::Completed {
                HttpResponse::BadRequest().json("Job state is final")
            } else {
                if job_callback.state == JobState::Failed || job_callback.state == JobState::Completed {
                    *mpi_free_slots += j.world_size;
                }

                j.state = job_callback.state;
                HttpResponse::NoContent().body("")
            }
        }
    } else {
        HttpResponse::NotFound().body("")
    }
}


#[actix_web::main]
async fn main() -> Result<()> {
    let mut provided_threads: i32 = -1;
    unsafe {
        mpi_sys::MPI_Init_thread(
            null_mut(),
            null_mut(),
            mpi_sys::MPI_THREAD_SERIALIZED as i32,
            &mut provided_threads,
        );
    }

    let mut mpi_comm_rank: i32 = -1;
    let mut mpi_comm_size: i32 = -1;
    unsafe {
        mpi_sys::MPI_Comm_rank(mpi_sys::RSMPI_COMM_WORLD, &mut mpi_comm_rank);
        mpi_sys::MPI_Comm_size(mpi_sys::RSMPI_COMM_WORLD, &mut mpi_comm_size);
    }
    if mpi_comm_size > 1 || mpi_comm_rank != 0 {
        bail!(
            "Running more than one instance of the API is not supported yet, but WASI-MPI-RS will \
            still use all available slots to run submitted jobs!"
        )
    }
    let mpi_universe_size: i32 = env::var("OMPI_UNIVERSE_SIZE")?.parse()?;


    // Read settings
    let settings = Settings::from("./config")?;
    // Init logging
    log4rs::init_file("config/log4rs.yaml", Default::default())?;
    info!("Initialized logging");
    info!("MPI_UNIVERSE_SIZE: {}", mpi_universe_size);

    let (tx, rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
    let mpi_future = web::block(mpi_closure(rx));

    let app_state = AppState{
        settings: settings.clone(),
        mpi_free_slots: Arc::new(RwLock::new(mpi_universe_size - 1)),
        mpi_tx: tx.clone(),
        jobs: Arc::new(RwLock::new(HashMap::new())),
    };

    // Define application
    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(app_state.clone())
            .service(get_jobs)
            .service(post_jobs)
            .service(put_jobs_callback)
    });

    info!("Starting server at http://{}", settings.host);
    // Bind application to host+port and run it
    let http_future = http_server.bind(settings.host)?.run();
    let (mpi_result, http_result) = join!(mpi_future, http_future);
    mpi_result?;
    http_result?;

    unsafe {
        mpi_sys::MPI_Finalize();
    }

    Ok(())
}
