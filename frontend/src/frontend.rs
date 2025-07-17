use yew::prelude::*;
use yew_router::prelude::*;

/// Application routes
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/helloasso")]
    HelloAsso,
}

/// Home page with introduction and a button leading to the HelloAsso page
#[function_component(Home)]
fn home() -> Html {
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

/// Simple page placeholder for the HelloAsso part of the application
#[function_component(HelloAssoPage)]
fn hello_asso_page() -> Html {
    html! {
        <div class="container">
            <h1>{ "Page HelloAsso" }</h1>
        </div>
    }
}

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
