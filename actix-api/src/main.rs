mod handlers;
mod database;
mod config;

use std::{time::SystemTime};
use actix_cors::Cors;
use actix_jwt_auth_middleware::{Authority, TokenSigner, use_jwt::UseJWTOnApp};
use actix_web::{dev::Service as _, web::{Data, scope, get, post, delete, put}, HttpServer, App, main};
use config::config::Config;
use database::db_client::DbClient;
use handlers::{user::{create_user, get_all_users, get_user, delete_user}, task::{add_task, get_all_tasks_for_user, update_task, delete_task}};
use log::{info, error};
use model::db::user::User;
use jwt_compact::alg::{Hs256, Hs256Key};
use futures_util::future::FutureExt;
use uuid::Uuid;
use crate::handlers::user::login_user;


#[main]
async fn main() -> std::io::Result<()> {
    //Enable debugging
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let bind_info = ("127.0.0.1", 5000);
    info!("Starting server on {}:{}", bind_info.0, bind_info.1);
    
    let config = Config::init();
    let connection_string = config.get("DATABASE_URL").expect("Unable to retrieve db connection string from env");
    let db = DbClient::init(&connection_string).await;
    
    let config_data = Data::new(config);
    let db_data = Data::new(db);

    let secret_key = Hs256Key::new(Uuid::new_v4()); //Just get a randomized uuid for a key for now

    HttpServer::new(move || {
        //Set up cors
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        //Set up jwt auth
        let auth = Authority::<User, Hs256, _, _>::new()
            .refresh_authorizer(|| async move {Ok(())})
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(secret_key.clone())
                    .algorithm(Hs256)
                    .build()
                    .expect("Unable to create token signer")
            ))
            .enable_header_tokens(true) //Force to check header instead of weird default cookie auth
            .verifying_key(secret_key.clone())
            .build()
            .expect("Unable to create auth middleware");

        App::new()
            .wrap(cors)
            .wrap_fn(|req, srv|{
                let now = SystemTime::now();
                let method = req.method().to_string();
                let path = req.path().to_string();
                srv.call(req).map(move |res| {
                    match now.elapsed() {
                        Ok(elapsed) => info!("{} request to {} completed in {}ms", method, path, elapsed.as_millis()),
                        Err(e) => error!("Unable to calc request time: {}", e.to_string())
                    }
                    res
                })
            })
            .app_data(db_data.clone())
            .app_data(config_data.clone())
            .route("/login", post().to(login_user))
            .route("/user", post().to(create_user))
            .use_jwt(auth,
                scope("/user")
                    .route("", get().to(get_all_users))
                    .service(
                        scope("/{user_id}")
                            .route("", delete().to(delete_user))
                            .route("", get().to(get_user))
                            .service(
                                scope("/task")
                                    .route("", post().to(add_task))
                                    .route("", get().to(get_all_tasks_for_user))
                                    .service(
                                        scope("/{task_id}")
                                            .route("", put().to(update_task))
                                            .route("", delete().to(delete_task))
                                    )
                            )
                    )
            )
    })
    .bind(bind_info)?
    .run()
    .await
}
