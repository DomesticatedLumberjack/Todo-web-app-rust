use actix_web::{http::header::Date};
use model::db::{user::User, task::Task};
use sqlx::{SqlitePool, Pool, Sqlite, Error, Row, sqlite::SqliteConnectOptions};
use uuid::Uuid;

#[derive(Clone)]
pub struct DbClient{
    pool: Pool<Sqlite>
}

impl DbClient{
    pub async fn init(connection_string: &str) -> DbClient{
        let options: SqliteConnectOptions = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(connection_string);
            


        let new_pool = SqlitePool::connect_with(options) 
            .await
            .expect("Unable to create db pool");

        sqlx::migrate!("./migrations")
            .run(&new_pool)
            .await
            .expect("Error while running db migrations");
        
        DbClient{
            pool: new_pool
        }
    }

    //Users
    pub async fn create_user(&self, username: &str, password_hash: &str) -> Result<User, Error> {
        let query = "INSERT INTO users VALUES($1, $2, $3, $4)";
        let new_id = Uuid::new_v4().to_string();
        let create_date = Date::now().to_string();

        sqlx::query(&query)
            .bind(&new_id)
            .bind(&username)
            .bind(&password_hash)
            .bind(&create_date)
            .execute(&self.pool)
            .await?;

        let ret_user = self.get_user(&new_id).await?;

        Ok(ret_user)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error>{
        let query = "SELECT * FROM users";

        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await?;

        let mut users: Vec<User> = Vec::new();
        for (_, row) in rows.iter().enumerate() {
            users.push(User{
                id: row.get("id"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                create_time: row.get("create_time")
            })
        }
        Ok(users)
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User, Error>{
        let query = "SELECT * FROM users WHERE id = $1";

        let row = sqlx::query(&query)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        let user = User{
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            create_time: row.get("create_time")
        };

        Ok(user)
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<i32, Error>{
        let query = "DELETE FROM users WHERE id = $1";

        let row = sqlx::query(&query)
            .bind(&user_id)
            .execute(&self.pool)
            .await?;

        Ok(row.rows_affected() as i32)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, Error>{
        let query = "SELECT * FROM users WHERE username = $1";

        let row = sqlx::query(&query)
            .bind(username)
            .fetch_one(&self.pool)
            .await?;

        let user = User{
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            create_time: row.get("create_time")
        };

        Ok(user)
    }

    //Tasks
    pub async fn create_task(&self, user_id: &str, description: &str, complete: bool) -> Result<Task, Error>{
        let query = "INSERT INTO tasks VALUES ($1, $2, $3, $4, $5, $6)";
        let new_id = Uuid::new_v4().to_string();
        let create_date = Date::now().to_string();


        sqlx::query(&query)
            .bind(&new_id)
            .bind(&user_id)
            .bind(&description)
            .bind(&complete)
            .bind(&create_date)
            .bind(&create_date)
            .execute(&self.pool)
            .await?;

        let return_task = self.get_task(&new_id).await?;

        Ok(return_task)
    }

    pub async fn get_all_tasks_for_user(&self, user_id: &str) -> Result<Vec<Task>, Error>{
        let query = "SELECT * FROM tasks WHERE user_id = $1";

        let rows = sqlx::query(&query)
            .bind(&user_id)
            .fetch_all(&self.pool)
            .await?;

        let mut tasks: Vec<Task> = Vec::new();
        for (_, row) in rows.iter().enumerate() {
            tasks.push(Task{
                id: row.get("id"),
                user_id: row.get("user_id"),
                description: row.get("description"),
                complete: row.get("complete"),
                create_date: row.get("create_date"),
                modified_date: row.get("modified_date")
            })
        }
        Ok(tasks)
    }

    pub async fn get_task(&self, task_id: &str) -> Result<Task, Error>{
        let query = "SELECT * FROM tasks WHERE id = $1";

        let row = sqlx::query(&query)
            .bind(task_id)
            .fetch_one(&self.pool)
            .await?;

        let task = Task{
            id: row.get("id"),
            user_id: row.get("user_id"),
            description: row.get("description"),
            complete: row.get("complete"),
            create_date: row.get("create_date"),
            modified_date: row.get("modified_date"),
        };

        Ok(task)
    }

    pub async fn update_task(&self, task_id: &str, user_id: &str, new_desc: &str, new_complete: bool) -> Result<Task, Error>{
        let query = "UPDATE tasks SET description = $1, complete = $2, modified_date = $3 WHERE id = $4 AND user_id = $5";
        let modified_date = Date::now().to_string();

        sqlx::query(&query)
            .bind(&new_desc)
            .bind(&new_complete)
            .bind(&modified_date)
            .bind(&task_id)
            .bind(&user_id)
            .execute(&self.pool)
            .await?;

        let return_task = self.get_task(task_id).await?;

        Ok(return_task)
    }

    pub async fn delete_task(&self, task_id: &str) -> Result<i32, Error>{
        let query = "DELETE FROM tasks WHERE id = $1";

        let row = sqlx::query(&query)
            .bind(&task_id)
            .execute(&self.pool)
            .await?;

        Ok(row.rows_affected() as i32)
    }
}