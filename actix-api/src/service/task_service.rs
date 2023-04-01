// use crate::model::{
//     db::{
//         task::Task, 
//         user::User
//     }, 
//     request::task_create_request::TaskCreateRequest, 
//     request::task_update_request::TaskUpdateRequest
// };
// use mongodb::{Client, options::{ClientOptions, UpdateModifications}, Collection, bson::doc};
// use uuid::{Uuid};

// const DB_ADDR: &str = "mongodb://192.168.0.214:27017";
// const DB_NAME: &str = "tasks_app";
// const USER_COLLECTION: &str = "users";

// pub struct TaskService{
//     client: Client,
//     user_collection: Collection<User>
// }

// impl TaskService{
//     pub async fn new() -> TaskService{
//         let opt = ClientOptions::parse(DB_ADDR).await.unwrap();
//         let client = Client::with_options(opt).unwrap();
//         let collection = client.database(DB_NAME).collection(USER_COLLECTION);

//         TaskService { 
//             client: client,
//             user_collection: collection
//         }
//     }

//     pub async fn add_task(self, task: TaskCreateRequest) -> String{
//         let new_task = Task{
//             _id: Uuid::new_v4().to_string(),
//             description: task.description,
//             complete: task.complete
//         };

//         match collection.insert_one(new_task, None).await {
//             Ok(res) => res.inserted_id.to_string(),
//             Err(err) => {panic!("{}", err.kind.to_string())},
//         }
//     }

//     pub async fn get_task(self, task_id: String) -> Task{
//         let collection: Collection<Task> = self.client.database(DB_NAME).collection(TASK_COLLECTION);
//         match collection.find_one(
//             doc!{"_id": &task_id}, 
//             None
//         ).await {
//             Ok(item) => {return item.unwrap()},
//             Err(err) => {panic!("{}", err.kind.to_string())},
//         };
//     }

//     pub async fn update_task(self, task: TaskUpdateRequest) -> String{
//         let collection: Collection<Task> = self.client.database(DB_NAME).collection(TASK_COLLECTION);
//         match collection.find_one_and_update(
//             doc! {"_id": &task._id},
//             UpdateModifications::Document(doc!{
//                 "$set": {
//                     "_id": &task._id,
//                     "description": &task.description,
//                     "complete": &task.complete
//                 }
//             }),
//             None
//         ).await {
//             Ok(res) => {
//                 if res.is_none(){
//                     panic!("Unable to locate task with id {}", &task._id);
//                 }
//                 res.unwrap()._id
//             },
//             Err(err) => {panic!("{}", err.kind.to_string())},
//         }
//     }

//     pub async fn delete_task(self, id: String) -> String{
//         let collection: Collection<Task> = self.client.database(DB_NAME).collection(TASK_COLLECTION);
//         match collection.find_one_and_delete(
//             doc! {
//                 "_id": &id
//             },
//             None
//         ).await {
//             Ok(_) => id,
//             Err(err) => panic!("{}", err.kind.to_string())
//         }
//     }

//     pub async fn get_all_tasks(self) -> Vec<Task>{
//         let collection: Collection<Task> = self.client.database(DB_NAME).collection(TASK_COLLECTION);
//         match collection.find(None,None).await {
//             Ok(mut cursor) => {
//                 let mut return_vector: Vec<Task> = Vec::new();
//                 while let Ok(result) = cursor.advance().await{
//                     if !result{
//                         break;
//                     }
//                     let doc = cursor.current();
//                     return_vector.push(
//                         Task{
//                             _id: doc.get_str("_id").unwrap().to_string(),
//                             description: doc.get_str("description").unwrap().to_string(),
//                             complete: doc.get_bool("complete").unwrap()
//                         }
//                     );
//                 }
//                 return_vector
//             },
//             Err(err) => panic!("{}", err.kind.to_string())
//         }
//     }
// }

// impl Clone for TaskService{
//     fn clone(&self) -> TaskService {
//         TaskService { 
//             client:  self.client.clone()
//         }
//     }
// }