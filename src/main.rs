use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use rust_todo::{job::AppState, routes::*};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the job list
    let jobs = web::Data::new(AppState {
        jobs: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        let job_controller = web::scope("/jobs")
            .app_data(jobs.clone())
            .service(get_all_jobs)
            .service(get_job_by_id)
            .service(create_job)
            .service(update_job)
            .service(delete_job);
        App::new()
            // Move the job list into the state
            .app_data(jobs.clone())
            // Mount the routes
            .service(job_controller)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
