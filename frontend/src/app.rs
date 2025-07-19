use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{HelloAssoPage, Home};
use crate::routes::Route;

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::HelloAsso => html! { <HelloAssoPage /> },
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
