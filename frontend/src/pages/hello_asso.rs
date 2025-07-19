use crate::routes::Route;
use gloo_net::http::Request;
use shared::Saison;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::Link;

/// Page listing all memberships with a refresh button
#[function_component(HelloAssoPage)]
pub fn hello_asso_page() -> Html {
    let saisons = use_state(Vec::<Saison>::new);

    let fetch_saisons = move |state: UseStateHandle<Vec<Saison>>| {
        spawn_local(async move {
            if let Ok(resp) = Request::get("/api/saisons").send().await {
                if let Ok(data) = resp.json::<Vec<Saison>>().await {
                    state.set(data);
                }
            }
        });
    };

    let load = {
        let saisons = saisons.clone();
        Callback::from(move |_e: MouseEvent| {
            fetch_saisons(saisons.clone());
        })
    };

    {
        let saisons = saisons.clone();
        use_effect(move || {
            fetch_saisons(saisons);
            || ()
        });
    }

    html! {
        <div class="container">
            <h1>{ "Adhésions" }</h1>
            <button onclick={load.clone()}>{ "Rafraîchir" }</button>
            <ul>
                { for saisons.iter().map(|s| html!{
                    <li>
                        <Link<Route> to={Route::Saison { id: s.id }}>
                            { &s.nom }
                        </Link<Route>>
                    </li>
                }) }
            </ul>
        </div>
    }
}
