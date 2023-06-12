use gloo_net::http::Request;
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use model::{response::{user::User, login::Login}, request::{login_request::LoginRequest, create_user_request::CreateUserRequest}};

const API_ADDR: &str = "http://127.0.0.1:5000";

#[component]
pub fn Login(
    cx: Scope
) -> impl IntoView{

    let set_user = use_context::<WriteSignal<Option<User>>>(cx).expect("Unable to find set_user");
    let (username, set_username) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let (api_response, set_api_response) = create_signal(cx, None::<String>);

    let login = move |_| { 
        wasm_bindgen_futures::spawn_local(async move {
            let request = LoginRequest{
                username: username.get_untracked(),
                password: password.get_untracked()
            };

            let response = Request::post(format!("{}/login", API_ADDR).as_str())
                .json(&request)
                .expect("Unable to serialize login request to json")
                .send()
                .await
                .expect(format!("No response from server at {} during login request", API_ADDR).as_str());
            
            if response.ok() {
                let nav = use_navigate(cx);

                let login_resp: Login = response
                    .json()
                    .await
                    .expect("Unable to deserialze login request json body");

                set_user(
                    Some(
                        User{ 
                            id: login_resp.id.clone(), 
                            username: login_resp.username, 
                            token: login_resp.access_token 
                        }
                    )
                );

                set_api_response(
                    Some(login_resp.id)
                );

                nav("/list", NavigateOptions::default()).unwrap();
            }else{
                let api_msg = response
                    .text()
                    .await
                    .expect("Unable to retrieve response body from login request");

                set_api_response(
                    Some(api_msg)
                );
            }
        });
    };

    let create_account = move |e| {
        wasm_bindgen_futures::spawn_local(async move {
            let request = CreateUserRequest{
                username: username.get_untracked(),
                password: password.get_untracked()
            };

            let response = Request::post(format!("{}/user", API_ADDR).as_str())
                .json(&request)
                .expect("Unable to serialize create user request to json")
                .send()
                .await
                .expect(format!("No response from server at {} during create user request", API_ADDR).as_str());
            
            if response.ok() {
                login(e);
            }else{
                let api_msg = response
                    .text()
                    .await
                    .expect("Unable to retrieve response body from login request");

                set_api_response(
                    Some(api_msg)
                );
            }
        });
    };

    view!{ cx,
        <div class="login vertical-center">
            <p class="api-reponse-message">
            {move || match api_response(){
                Some(msg) => msg,
                None => String::new()
            }}
            </p>
            <label for="unameinput">{"Username"}</label>
            <input type="text" on:input=move |ev| set_username(event_target_value(&ev))/>
            <label for="passwordinput">{"Password"}</label>
            <input type="text" on:input=move |ev| set_password(event_target_value(&ev))/>
            <button class="login-button" on:click=login>{"Log In"}</button>
            <button class="login-button" on:click=create_account>{"Create Account"}</button>
        </div>
        <p class="create-account-info">{"*Selecting Create Account will generate an account with the entered credentials"}</p>
    }
}