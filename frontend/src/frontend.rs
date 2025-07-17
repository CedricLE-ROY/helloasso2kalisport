use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <h1>{ "HelloAsso2Kalisport" }</h1>
            <p>{ "Bienvenue ! Cette application vous permettra d'exporter vos inscriptions HelloAsso vers Kalisport." }</p>
            <button>{ "Commencer" }</button>
        </div>
    }
}
