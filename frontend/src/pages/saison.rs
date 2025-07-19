use gloo_net::http::Request;
use shared::Adherent;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SaisonPageProps {
    pub saison_id: u32,
}

#[function_component(SaisonPage)]
pub fn saison_page(props: &SaisonPageProps) -> Html {
    let adherents = use_state(Vec::<Adherent>::new);

    {
        let adherents = adherents.clone();
        let id = props.saison_id;
        use_effect_with(id, move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get(&format!("/api/saisons/{}/adhesions", id))
                    .send()
                    .await
                {
                    if let Ok(data) = resp.json::<Vec<Adherent>>().await {
                        adherents.set(data);
                    }
                }
            });
            || ()
        });
    }

    let onclick = {
        let adherents = adherents.clone();
        let id = props.saison_id;
        Callback::from(move |_e: MouseEvent| {
            let adherents = adherents.clone();
            spawn_local(async move {
                if let Ok(resp) = Request::get(&format!("/api/saisons/{}/adhesions", id))
                    .send()
                    .await
                {
                    if let Ok(data) = resp.json::<Vec<Adherent>>().await {
                        adherents.set(data);
                    }
                }
            });
        })
    };

    html! {
        <div class="container">
            <h1>{ format!("Inscriptions saison {}", props.saison_id) }</h1>
            <button {onclick}>{ "Rafraîchir" }</button>
            <ul>
                { for adherents.iter().map(|a| html!{
                    <li>{ format!("{} {} - {}", a.prenom, a.nom, a.email) }</li>
                }) }
            </ul>
        </div>
    }
}
