use yew::prelude::*;

struct PostProps {
    id: String,
    author_id: String,
    content_md: String,
    content_html: String,
    reply_at: String,
    created_at: String,
    updated_at: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let test = use_state(|| true);
    let post_list: UseStateHandle<Vec<PostProps>> = use_state(|| vec![]);
    //     let post_list: UseStateHandle<PostProps> = use_state(|| PostProps[] {[{
    //         id: String::from(""),
    // author_id: String::from(""),
    // content_md: String::from(""),
    // content_html: String::from(""),
    // reply_at: String::from(""),
    // created_at: String::from(""),
    // updated_at: String::from(""),
    // }]});
    // let play_list = use_state(|| vec![]);
    let mut demo: Vec<PostProps> = vec![];
    demo.push(PostProps {
        id: String::from("1"),
        author_id: String::from("grljksbgl"),
        content_md: String::from("# title"),
        content_html: String::from("<h1>title</h1>"),
        reply_at: String::from(""),
        created_at: String::from(""),
        updated_at: String::from(""),
    });
    demo.push(PostProps {
        id: String::from("2"),
        author_id: String::from("grljksbgl"),
        content_md: String::from("# title2"),
        content_html: String::from("<h1>title2</h1>"),
        reply_at: String::from(""),
        created_at: String::from(""),
        updated_at: String::from(""),
    });
    // let value_setter = move |e| {
    //     let test = test.clone();
    //     test.set(e)
    // };
    {
        let post_list = post_list.clone();
        use_effect_with_deps(
            move |_| {
                // Make a call to DOM API after component is rendered
                // value_setter(false);
                // let test = test.clone();
                post_list.set(demo)

                // Perform the cleanup
            },
            test.clone(),
        );
    }

    // post_list.set(demo);
    // Callback::from(|_: _| post_list.set(demo));

    html! {
        <div>
            <div>{{"投稿一覧"}}</div>
            <div>
            { for post_list.iter().map(|e| {
                html!{
                    <div>
                        {e.id.to_string()}
                        {e.content_md.to_string()}
                    </div>
                }

            }) }
            <button id="login">{{"test"}}</button>
            // {
            //     *test
            // }
    //         <div id="g_id_onload"
    //      data-client_id="53465699991-r2g68g53aqvoa9muu8sbf2sn4789lk4l.apps.googleusercontent.com"
    //     //  data-login_uri="https://your.domain/your_login_endpoint"
    //      data-auto_prompt="false">
    //   </div>
    //   <div class="g_id_signin"
    //      data-type="standard"
    //      data-size="large"
    //      data-theme="outline"
    //      data-text="sign_in_with"
    //      data-shape="rectangular"
    //      data-logo_alignment="left">
    //   </div>
            </div>
        </div>
    }
}
