use yew_router::prelude::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/helloasso")]
    HelloAsso,
    #[at("/saison/:id")]
    Saison { id: u32 },
}
