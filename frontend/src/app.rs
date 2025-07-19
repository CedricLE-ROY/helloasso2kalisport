use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{HelloAssoPage, Home, SaisonPage};
use crate::routes::Route;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::HelloAsso => html! { <HelloAssoPage /> },
        Route::Saison { id } => html! { <SaisonPage saison_id={id} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
