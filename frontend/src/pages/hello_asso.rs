use gloo_net::http::Request;
use shared::{HelloAssoForm, HelloAssoForms};
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;

/// Page listing all memberships with a refresh button
#[function_component(HelloAssoPage)]
pub fn hello_asso_page() -> Html {
    let forms = use_state(Vec::<HelloAssoForm>::new);

    let fetch_forms = move |state: UseStateHandle<Vec<HelloAssoForm>>| {
        spawn_local(async move {
            if let Ok(data) = async {
                let resp = Request::get("/api/helloasso/forms").send().await?;
                resp.json::<HelloAssoForms>().await.map(|f| f.data)
            }
            .await
            {
                state.set(data);
            }
        });
    };

    let load = {
        let forms = forms.clone();
        Callback::from(move |_e: MouseEvent| {
            fetch_forms(forms.clone());
        })
    };

    {
        let forms = forms.clone();
        use_effect(move || {
            fetch_forms(forms);
            || ()
        });
    }

    html! {
        <div class="container">
            <h1>{ "Adhésions" }</h1>
            <button onclick={load.clone()}>{ "Rafraîchir" }</button>
            <ul>
                { for forms.iter().map(|f| html!{
                    <li>{ &f.name }</li>
                }) }
            </ul>
        </div>
    }
}
