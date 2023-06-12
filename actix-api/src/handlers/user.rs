use actix_jwt_auth_middleware::{AuthResult, TokenSigner};
use actix_web::{web::{Data, Json, Path}, HttpResponse};
use jwt_compact::{alg::Hs256};
use model::{request::create_user_request::CreateUserRequest, response::user::User, response::login::Login, request::login_request::LoginRequest};
use time::Duration;

use crate::{database::db_client::DbClient, config::config::Config};

fn hash_str(input_str: &str, config: Data<Config>) -> String{
    let salt = config.get("SALT").expect("Unable to locate value (SALT) in config");
    let hash = argon2::hash_encoded(&input_str.as_bytes(), &salt.as_bytes(), &argon2::Config::default()).expect("Unable to hash password for new user request");
    hash
}

pub async fn create_user(db: Data<DbClient>, config: Data<Config>, request: Json<CreateUserRequest>, token_signer: Data<TokenSigner<model::db::user::User, Hs256>>) -> HttpResponse{
    let password_hash = hash_str(&request.password, config);

    let existing_user = db.get_user_by_username(&request.username).await;

    if existing_user.is_ok(){
        return HttpResponse::Conflict().body(format!("User with username {} already exists", request.username));
    }

    match db.create_user(&request.username, &password_hash).await {
        Ok(user) => {
            let token = token_signer.create_signed_token(&user, Duration::days(1)).expect("Unable to create signed token for newly created user");

            let return_user = User{
                id: user.id,
                username: user.username,
                token: token
            };
            HttpResponse::Created().json(return_user)
        },
        Err(err) => HttpResponse::NotAcceptable().body(err.to_string())
    }
}

pub async fn get_all_users(db: Data<DbClient>) -> HttpResponse{
    match db.get_all_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::BadRequest().body(err.to_string())
    }
}

pub async fn get_user(db: Data<DbClient>, user_id: Path<String>) -> HttpResponse{
    match db.get_user(&user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

pub async fn delete_user(db: Data<DbClient>, user_id: Path<String>) -> HttpResponse{
    match db.delete_user(&user_id).await {
        Ok(num_items) => {
            if num_items > 0 {
                HttpResponse::Ok().body(format!("Deleted user with id: {}", user_id))
            }
            else {
                HttpResponse::BadRequest().body(format!("Unable to locate user with id: {} in db", user_id))
            }
        },
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

pub async fn login_user(db: Data<DbClient>, token_signer: Data<TokenSigner<model::db::user::User, Hs256>>, request: Json<LoginRequest>, config: Data<Config>) -> AuthResult<HttpResponse>{
    match db.get_user_by_username(&request.username).await{
        Ok(user) => {
            if hash_str(&request.password, config) == user.password_hash{
                //This is stupid and I hate this library
                let token = token_signer.create_signed_token(&user, Duration::days(1))?;

                let response = Login{
                    id: user.id,
                    username: user.username,
                    access_token: token
                };

                Ok(
                    HttpResponse::Ok()
                        .json(response)
                )
            }
            else{
                Ok(HttpResponse::BadRequest().body("Incorrect password or username"))
            }

        },
        Err(_) => Ok(HttpResponse::BadRequest().body("Incorrect password or username"))
    }

    // Ok(HttpResponse::Ok()
    //     .cookie(token_signer.create_access_cookie(&user)?)
    //     .cookie(token_signer.create_refresh_cookie(&user)?)
    //     .body("You are now logged in"))
}