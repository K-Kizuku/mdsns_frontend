use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    // #[at("/sign_up")]
    // SignUp,
    #[at("/auth")]
    Auth,
    #[at("/users")]
    Users,
    #[at("/contents_post")]
    ContentsPost,
    #[at("/contents_edit")]
    ContentsEdit,
    #[not_found]
    #[at("/404")]
    NotFound,
}
