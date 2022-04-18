use crate::{
    dto::{createJobDto::CreateJobDto, updateJobDto::UpdateJobDto},
    job::*,
};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
#[get("/")]
pub async fn get_all_jobs(data: web::Data<AppState>) -> impl Responder {
    let jobs = data.jobs.lock().expect("Cannot be accessed");
    HttpResponse::Ok().json(jobs.clone())
}

#[get("/{id}")]
pub async fn get_job_by_id(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let jobs = data.jobs.lock().expect("Cannot be accessed");
    for job in jobs.iter() {
        if job.id.eq(&id) {
            return HttpResponse::Ok().json(job.clone());
        }
    }
    HttpResponse::NotFound().finish()
}

#[post("/")]
pub async fn create_job(job: web::Json<CreateJobDto>, data: web::Data<AppState>) -> impl Responder {
    let job = Job::new(job.title.clone(), job.description.clone());
    data.jobs.lock().unwrap().push(job.clone());
    HttpResponse::Ok().json(job)
}

#[patch("/{id}")]
pub async fn update_job(
    path: web::Path<String>,
    update_job: web::Json<UpdateJobDto>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let mut jobs = data.jobs.lock().unwrap();
    for job in jobs.iter_mut() {
        if job.id.eq(&id) {
            job.update(
                update_job.title.clone(),
                update_job.description.clone(),
                update_job.done,
            );
            return HttpResponse::Ok().json(job.clone());
        }
    }
    HttpResponse::NotFound().finish()
}

#[delete("/{id}")]
pub async fn delete_job(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let mut jobs = data.jobs.lock().unwrap();
    let job = jobs.iter().position(|job| job.id.eq(&id)).unwrap();
    jobs.remove(job);
    HttpResponse::Ok().finish()
}
