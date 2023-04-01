use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub user: String,
    pub login_callback: Callback<MouseEvent>
}

#[function_component]
pub fn Header(props: &Props) -> Html{
    html!{
        <>
            <button onclick={&props.login_callback} class="logout-button">{"Log Out"}</button>
            <div class="username">{&props.user}</div>
        </>
    }
}