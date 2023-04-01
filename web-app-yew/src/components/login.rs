use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub login_callback: Callback<MouseEvent>
}

#[function_component]
pub fn Login(props: &Props) -> Html{
    html!{
        <div class="login vertical-center">
            <label for="unameinput">{"Username"}</label><br/>
            <input/><br/>
            <label for="passwordinput">{"Password"}</label><br/>
            <input/><br/>
            <button onclick={&props.login_callback} class="login-button">{"Log In"}</button>
            <button class="login-button">{"Create Account"}</button>
        </div>
    }
}