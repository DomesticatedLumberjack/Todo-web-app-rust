mod components;
mod model;

use gloo_net::http::Request;
use log::info;
use serde_json::json;
use yew::prelude::*;

use crate::{model::db::{user::User, task::Task}, components::{list::List, login::Login, header::Header}};

const API_ADDR: &str = "http://127.0.0.1:5000";
const USER_ID: &str = "184f3f17-157d-41e4-a541-093a33325dfe";

#[function_component] 
fn App() -> Html {
    let user: UseStateHandle<Option<User>> = use_state(|| None);

    let login_user = user.clone();
    let login_callback = Callback::from(move |_| {
        let login_user = login_user.clone();
        if login_user.is_none(){
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_user: User = Request::get(format!("{}/user/{}", API_ADDR, USER_ID).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                login_user.set(Some(fetched_user));
            });
        }
        else {
            login_user.set(None)
        }
    });

    let update_task_user = user.clone();
    let update_task: Callback<Task> = Callback::from(move |task: Task| {
        let update_task_user = update_task_user.clone();
        let mut edit_data = match &*update_task_user{
            Some(u) => u.clone(),
            None => panic!("No user found")
        };

        for mut t in &mut edit_data.task_items{
            if t._id == task._id {
                info!("testing");
                t.complete = !task.complete;
                let submit_data = t.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let update_task_user = update_task_user.clone();
                    
                    let json_body = json!({
                        "user_id": edit_data._id.to_string(),
                        "task_id": submit_data._id.to_string(),
                        "description": submit_data.description.to_string(),
                        "complete": submit_data.complete
                    });

                    match Request::put(format!("{}/user/task",API_ADDR).as_str())
                        .json(&json_body)
                        .unwrap()
                        .send()
                        .await 
                    {
                        Ok(_) => &update_task_user.set(Some(edit_data)),
                        Err(_) => panic!{"Unable to update task"}
                    };
                });
                
                break;
            }
        }

        
    });

    html! {
        <>
            {
                match &*user
                {
                    Some(u) => {
                        html!{
                        <>
                        <Header {login_callback} user={u.username.clone()}/>
                        <div class="list vertical-center">
                            <List list_items={u.task_items.clone()} {update_task}/>
                        </div>
                        </>
                        }
                    },
                    None => html!{ <Login {login_callback}/> }
                }
            }
            <div class="bottom-tag">{"Made with Rust"}</div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}