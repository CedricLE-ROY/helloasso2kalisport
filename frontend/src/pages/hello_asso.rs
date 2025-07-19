use gloo_net::http::Request;
use shared::Adherent;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;

/// Page listing all memberships with a refresh button
#[function_component(HelloAssoPage)]
pub fn hello_asso_page() -> Html {
    let adhesions = use_state(Vec::<Adherent>::new);

    let fetch_adhesions = move |state: UseStateHandle<Vec<Adherent>>| {
        spawn_local(async move {
            if let Ok(resp) = Request::get("/api/adhesions").send().await {
                if let Ok(data) = resp.json::<Vec<Adherent>>().await {
                    state.set(data);
                }
            }
        });
    };

    let load = {
        let adhesions = adhesions.clone();
        Callback::from(move |_e: MouseEvent| {
            fetch_adhesions(adhesions.clone());
        })
    };

    {
        let adhesions = adhesions.clone();
        use_effect(move || {
            fetch_adhesions(adhesions);
            || ()
        });
    }

    html! {
        <div class="container">
            <h1>{ "Adhésions" }</h1>
            <button onclick={load.clone()}>{ "Rafraîchir" }</button>
            <ul>
                { for adhesions.iter().map(|a| html!{
                    <li>{ format!("{} {} - {}", a.prenom, a.nom, a.email) }</li>
                }) }
            </ul>
        </div>
    }
}
