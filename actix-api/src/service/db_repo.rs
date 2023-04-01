
use futures::{TryStreamExt};
use mongodb::{
    Client, 
    Collection, 
    options::{ClientOptions, FindOneAndUpdateOptions, ReturnDocument}, 
    bson::doc, 
    results::{InsertOneResult, UpdateResult}, 
    error::Error
};
use argon2::{self, Config};

use crate::model::db::{user::User, task::Task};
const DB_ADDR: &str = "mongodb://192.168.0.120:27017";
const DB_NAME: &str = "tasks_app";
const USER_COLLECTION: &str = "users";
const TESTING_SALT: &str = "super_secret_salt";

#[derive(Clone)]
pub struct DbRepo{
    user_collection: Collection<User>
}

impl DbRepo{
    pub async fn init() -> DbRepo{
        let opt = ClientOptions::parse(DB_ADDR).await.unwrap();
        let client = Client::with_options(opt).unwrap();
        let collection = client.database(DB_NAME).collection(USER_COLLECTION);

        DbRepo {
            user_collection: collection
        }
    }

    pub async fn create_user(&self, username: String, password: String) -> Result<InsertOneResult, Error>{
        //Handle password hashing
        let config = Config::default();
        let hash = argon2::hash_encoded(password.as_bytes(), TESTING_SALT.as_bytes(), &config).unwrap();

        let new_user =  User {
            _id: uuid::Uuid::new_v4().to_string(),
            username,
            password_hash: hash,
            task_items: Vec::new()
        };
        self.user_collection.insert_one(
            new_user,
            None
        ).await
    }

    pub async fn get_user(&self, user_id: String) -> Result<Option<User>, Error>{
        self.user_collection.find_one(
            doc!{
            "_id": user_id
            },
            None
        ).await
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error>{
        let cursor = self.user_collection.find(None, None).await.unwrap();
        let return_vec = cursor.try_collect().await;
        return_vec
    }

    pub async fn create_task(&self, user_id: String, task_description: String, task_complete: bool) -> Result<Option<User>, Error>{
        let opt = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();

        self.user_collection.find_one_and_update(
            doc!{"_id": user_id}, 
            doc!{
                "$push": {
                    "task_items": {
                        "_id": uuid::Uuid::new_v4().to_string(),
                        "description": task_description,
                        "complete": task_complete
                    }
                },
            },
            opt
        ).await
    }

    pub async fn delete_task(&self, user_id: String, task_id: String) -> Result<UpdateResult, Error>{
        self.user_collection.update_one(
            doc!{"_id": user_id},
            doc!{
                "$pull": {
                    "task_items": {
                        "_id": task_id
                    }
                }
            }, 
            None
        ).await
    }

    pub async fn update_task(&self, user_id: String, task: Task) -> Result<UpdateResult, Error>{
        println!("{}", task._id);
        self.user_collection.update_one(
            doc!{
                "_id": user_id, 
                "task_items": {
                    "$elemMatch": { "_id": task._id}
                }},
            doc!{
                "$set": {
                    "task_items.$.description": task.description,
                    "task_items.$.complete": task.complete
                }
            },
            None
        ).await
    }

    
}