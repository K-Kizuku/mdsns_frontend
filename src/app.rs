use md_sns::auth::Auth;
use md_sns::contents_edit::ContentsEdit;
use md_sns::contents_post::ContentsPost;
use md_sns::home::Home;
// use md_sns::route::Route;
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
    <header class="header__parent">
      <div class="header__inner">
        <h1 class="header__title header-title">
          <a href="/">
            {"MDSNS"}
          </a>
        </h1>

        <nav class="header__nav nav" id="js-nav">
          <ul class="nav__items nav-items">
            <li class="nav-items__item"><a href="/auth">{"サインイン"}</a></li>
            <li class="nav-items__item"><a href="/">{"タイムライン"}</a></li>
            <li class="nav-items__item"><a href="/users">{"マイページ"}</a></li>
            <li class="nav-items__item post"><a href="/contents_post">{"投稿する"}</a></li>
          </ul>
        </nav>
      </div>
    </header>
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
