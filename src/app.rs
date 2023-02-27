use md_sns::auth::Auth;
use md_sns::contents_edit::ContentsEdit;
use md_sns::contents_post::ContentsPost;
use md_sns::home::Home;
use md_sns::user::Users;
use yew::prelude::*;
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

fn switch(routes: Route) -> Html {
    match routes {
        // Route::Home => html! {
        //     <Home />
        // },
        // Route::SignIn => html! {
        //     <SignIn />
        // },
        // Route::SignUp => html! {
        //     <SignUp />
        // },
        Route::Auth => html! {
            <Auth />
        },
        Route::Home => html! {
            <Home />
        },
        Route::Users => html! {
            <Users />
        },
        Route::ContentsEdit => html! {
            <ContentsEdit />
        },
        Route::ContentsPost => html! {
            <ContentsPost />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(CustomHeader)]
pub fn custom_header() -> Html {
    html! {
        <div class="header">{"ヘッダーだよ"}</div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <CustomHeader />
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </>
    }
}
