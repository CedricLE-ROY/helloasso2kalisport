use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::routes::Route;

/// Home page with introduction and a button leading to the HelloAsso page
#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().expect("navigator");
    let onclick = Callback::from(move |_| navigator.push(&Route::HelloAsso));

    html! {
        <div class="container">
            <h1>{ "HelloAsso2Kalisport" }</h1>
            <p>{ "Bienvenue ! Cette application vous permettra d'exporter vos inscriptions HelloAsso vers Kalisport." }</p>
            <button {onclick}>{ "Commencer" }</button>
        </div>
    }
}
