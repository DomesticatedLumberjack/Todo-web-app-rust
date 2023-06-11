use gloo_net::http::Request;
use leptos::*;
use model::{response::{task::Task, user::User}, request::{update_task_request::UpdateTaskRequest, create_task_request::CreateTaskRequest}};

const API_ADDR: &str = "http://127.0.0.1:5000";

#[component]
pub fn List(
    cx: Scope
) -> impl IntoView{

    let user = use_context::<ReadSignal<Option<User>>>(cx).expect("Unable to retrieve user read signal in list componenet");
    let (tasks, set_tasks) = create_signal(cx, Vec::<Task>::new());
    let (new_task_desc, set_new_task_desc) = create_signal(cx, String::new());

    let get_tasks = move || {
        wasm_bindgen_futures::spawn_local(async move {
            let u = user.get_untracked().expect("Unable to get user");
            let tasks_req: Vec::<Task> = Request::get(format!("{}/user/{}/task", API_ADDR, u.id).as_str())
                .header("access_token", u.token.as_str())
                .send()
                .await
                .expect("Unable to reach server during task list retrieval request")
                .json()
                .await
                .expect("Unable to deserialize task list request");

            set_tasks(
                tasks_req
            );
        })
    };

    create_effect(cx, move |_| get_tasks());

    let update_task = move |id: &String| {
        let id = id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            //iterate through array and return the mutated new value
            let new_task_list: Vec::<Task> = tasks.get_untracked().into_iter().map(|mut t| {
                if t.id == id {
                    t.complete = !t.complete;
                }
                t
            }).collect();

            let user = user.get_untracked().expect("Unable to get user");
            let new_task = new_task_list.iter().find(|t| t.id == id).expect("Unable to find task with matching id");
            let new_task_req = UpdateTaskRequest{
                complete: new_task.complete.clone(),
                description: new_task.description.clone()
            };
            
            Request::put(format!("{}/user/{}/task/{}", API_ADDR, user.id, new_task.id).as_str())
                .header("access_token", &user.token)
                .json(&new_task_req)
                .expect("Unable to serialize task update request")
                .send()
                .await
                .expect("No response from server during update task request");

            set_tasks(
                new_task_list
            );
        })
    };

    let delete_task = move |id: &String| {
        let id = id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let user = user.get_untracked().expect("Unable to get user");
            let res = Request::delete(format!("{}/user/{}/task/{}", API_ADDR, user.id, id).as_str())
                .header("access_token", &user.token)
                .send()
                .await
                .expect("Unable to reach server for delete task request");

            if res.ok() {
                let new_task_list: Vec::<Task> = tasks.get_untracked().into_iter().filter(|t| t.id != id).collect();

                set_tasks(
                    new_task_list
                );
            }
        });
    };

    let create_task = move || {
        let user = user.get_untracked().expect("Unable to retrieve user for create task request");
        if tasks.get().len() > 0 {
            wasm_bindgen_futures::spawn_local(async move {
                let new_task = CreateTaskRequest{
                    description: new_task_desc.get_untracked(),
                    complete: false
                };

                let res = Request::post(format!("{}/user/{}/task", API_ADDR, user.id).as_str())
                    .header("access_token", &user.token)
                    .json(&new_task)
                    .expect("Unable to serialize create task request")
                    .send()
                    .await
                    .expect("Unable to reach server for create task request");

                if res.ok() {
                    get_tasks();
                }
            });
        }
    };

    view!{cx, 
        <table>
        <h3>{user().unwrap().username}</h3>
        <tr>
            <th colspan="2">
                <input on:input=move |ev| set_new_task_desc(event_target_value(&ev)) class="add-item-input"/>
            </th>
            <th>
                <button on:click=move |_| create_task() class="add-item-button">"Add Task"</button>
            </th>
        </tr>
        <For
            each=tasks
            key=|task| task.id.clone()
            view=move |cx, task| {
                let update_id = task.id.clone();
                let delete_id = task.id.clone();
                let description = task.description.clone();
                view! { cx,
                <tr>
                    <td>
                        <input on:click=move |_| update_task(&update_id) type={"checkbox"} checked={task.complete}/>
                    </td>
                    <td>
                        {description}
                    </td>
                    <td>
                        <button on:click=move |_| delete_task(&delete_id) class="delete-item-button">{"X"}</button>
                    </td>
                </tr>
                }
            }
        />
        
        </table>
    }
}