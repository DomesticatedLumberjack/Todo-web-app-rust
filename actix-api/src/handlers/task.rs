use actix_web::{web::{Path, Data, Json}, HttpResponse};

use crate::{database::db_client::DbClient};
use model::{request::{create_task_request::CreateTaskRequest, update_task_request::UpdateTaskRequest}, response::task::Task};


pub async fn add_task(db: Data<DbClient>, user_id: Path<String>, task_req: Json<CreateTaskRequest>) -> HttpResponse{
    match db.create_task(&user_id, &task_req.description, task_req.complete).await {
        Ok(task) => {
            let return_task = Task{
                id: task.id,
                description: task.description,
                complete: task.complete,
            };
            HttpResponse::Ok().json(return_task)
        },
        Err(err) => HttpResponse::BadRequest().body(err.to_string())
    }
}

pub async fn get_all_tasks_for_user(db: Data<DbClient>, user_id: Path<String>) -> HttpResponse{
    match db.get_all_tasks_for_user(&user_id).await {
        Ok(tasks) => {
            let mut response_tasks: Vec<Task> = Vec::new();
            for task in tasks.into_iter(){
                response_tasks.push(Task{
                    id: task.id,
                    description: task.description,
                    complete: task.complete,
                });
            }
            HttpResponse::Ok().json(response_tasks)
        },
        Err(err) => HttpResponse::BadRequest().body(err.to_string())
    }
}

pub async fn update_task(db: Data<DbClient>, item_ids: Path<(String, String)>, task_update_req: Json<UpdateTaskRequest>) -> HttpResponse{
    match db.update_task(&item_ids.1, &item_ids.0, &task_update_req.description, task_update_req.complete).await {
        Ok(new_task) => {
            let response_task = Task{
                id: new_task.id,
                description: new_task.description,
                complete: new_task.complete,
            };
            HttpResponse::Ok().json(response_task)
        },
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

pub async fn delete_task(db: Data<DbClient>, item_ids: Path<(String, String)>) -> HttpResponse{
    match db.delete_task(&item_ids.1).await{
        Ok(rows_affected) => {
            if rows_affected > 0 {
                HttpResponse::Ok().body("Successfully deleted task")
            }
            else {
                HttpResponse::BadRequest().body(format!("Unable to locate task with id {}", &item_ids.1))
            }
        },
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}