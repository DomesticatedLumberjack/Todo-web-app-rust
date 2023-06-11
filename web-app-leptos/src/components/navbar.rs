use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use model::response::user::User;

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView{
    let user = use_context::<ReadSignal<Option<User>>>(cx).expect("Unable to retrieve user read signal in navbar");
    let set_user = use_context::<WriteSignal<Option<User>>>(cx).expect("Unable to retrieve user write signal in navbar");

    let is_logged_in = move || user().is_some();

    let logout = move |_| {
        set_user(None);
        let nav = use_navigate(cx);
        nav("/", NavigateOptions::default()).unwrap();
    };

    view!{ cx,
        <h2>"Todo App"</h2>
        {move || if is_logged_in(){
            view!{ cx, 
                <button class="logout-button" on:click=logout>"Logout"</button>
            }.into_view(cx)
        }else{
            view!{ cx, 

            }.into_view(cx)
        }}
    }
}