mod service;
mod model;

use actix_cors::Cors;
use actix_web::{
    web::{
        self, Data
    }, 
    App, 
    HttpServer, 
    main, 
    post, 
    HttpResponse, 
    get, delete, put
};
use model::{
    request::task_update_request::TaskUpdateRequest, 
    db::task::Task
};
use crate::{
    model::{
        request::{
            create_user_request::CreateUserRequest, 
            task_create_request::TaskCreateRequest, 
            task_delete_request::TaskDeleteRequest
        }
    }, 
    service::db_repo::DbRepo
};


#[post("/user")]
pub async fn add_user(db: Data<DbRepo>, json: web::Json<CreateUserRequest>) -> HttpResponse{
     let new_user = CreateUserRequest {
         username: json.username.to_owned(),
         password: json.password.to_owned()
    };

    let result = db.create_user(new_user.username, new_user.password).await;

    match result {
        Ok(x) => HttpResponse::Ok().json(x.inserted_id),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<DbRepo>, id: web::Path<String>) -> HttpResponse{
    let user_id = id.to_owned();
    let result = db.get_user(user_id).await;
    match result {
        Ok(x) => match x {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NotFound().finish(),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user")]
pub async fn get_all_users(db: Data<DbRepo>) -> HttpResponse{
    let result = db.get_all_users().await;
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/user/task")]
pub async fn add_task_to_user(db: Data<DbRepo>, json: web::Json<TaskCreateRequest>) -> HttpResponse {
    let result = db.create_task(json.user_id.to_owned(), json.description.to_owned(), json.complete.to_owned()).await;
    match result{
        Ok(update_result) => HttpResponse::Ok().json(update_result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[delete("/user/task")]
pub async fn delete_task(db: Data<DbRepo>, json: web::Json<TaskDeleteRequest>) -> HttpResponse{
    let result = db.delete_task(json.user_id.to_owned(), json.task_id.to_owned()).await;
    match result {
        Ok(update_result) => {
            if update_result.modified_count > 0 {
                HttpResponse::Ok().finish()
            }
            else{
                HttpResponse::NotFound().finish()
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[put("/user/task")]
pub async fn update_task(db: Data<DbRepo>, task: web::Json<TaskUpdateRequest>) -> HttpResponse{
    let updated_task = Task{
        _id: task.task_id.to_owned(),
        description: task.description.to_owned(),
        complete: task.complete
    };
    let result = db.update_task(task.user_id.to_owned(), updated_task).await;
    match result{
        Ok(update_result) => {
            if update_result.modified_count > 0 {
                HttpResponse::Ok().finish()
            }
            else{
                HttpResponse::NoContent().finish()
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[main]
async fn main() -> std::io::Result<()> {
    //Enable debugging
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    

    let bind_info = ("127.0.0.1", 5000);
    let db = DbRepo::init().await;
    let data = Data::new(db);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .service(add_user)
            .service(get_user)
            .service(get_all_users)
            .service(add_task_to_user)
            .service(delete_task)
            .service(update_task)
    })
    .bind(bind_info)?
    .run()
    .await
}
