use leptos::*;

#[component]
pub fn Footer(
    cx: Scope
) -> impl IntoView{
    view!{
        cx, 
        <p class="bottom-tag">"Made with rust"</p>
    }
}