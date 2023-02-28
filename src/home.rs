use gloo::storage::LocalStorage;
use gloo_net::http::Request;
use gloo_net::http::Response;
use gloo_storage::Storage;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use serde_json::json;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Clone, PartialEq, Deserialize)]
struct Favorite {
    id: String,
    name: String,
    description: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Author {
    id: String,
    name: String,
    description: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct PostProps {
    id: String,
    content_md: String,
    content_html: String,
    reply_at: Option<String>,
    author: Author,
    favorite_count: i64,
    favorited_by: Vec<PostProps>,
    // [
    //   {
    //     "id": "d290f1ee-6c54-4b01-90e6-d701748f0851",
    //     "name": "string",
    //     "description": "string",
    //     "created_at": "2016-08-29T09:12:33.001Z",
    //     "updated_at": "2016-08-29T09:12:33.001Z"
    //   }
    // ],
    replied_count: i64,
    created_at: String,
    updated_at: String,
}

#[function_component(Home)]
pub fn home() -> Html {
    let post_list: UseStateHandle<Vec<PostProps>> = use_state(|| vec![]);
    let favo: UseStateHandle<i64> = use_state(|| 0);
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
    // let mut demo: Vec<PostProps> = vec![];
    // demo.push(PostProps {
    //     id: String::from("1"),
    //     author_id: String::from("grljksbgl"),
    //     content_md: String::from("# title"),
    //     content_html: String::from("<h1>title</h1>"),
    //     reply_at: String::from(""),
    //     created_at: String::from(""),
    //     updated_at: String::from(""),
    // });
    // demo.push(PostProps {
    //     id: String::from("2"),
    //     author_id: String::from("grljksbgl"),
    //     content_md: String::from("# title2"),
    //     content_html: String::from("<h1>title2</h1>"),
    //     reply_at: String::from(""),
    //     created_at: String::from(""),
    //     updated_at: String::from(""),
    // });
    // let value_setter = move |e| {
    //     let test = test.clone();
    //     test.set(e)
    // };
    {
        let post_list = post_list.clone();
        use_effect_with_deps(
            move |_| {
                let post_list = post_list.clone();
                let token: String = LocalStorage::get("authentication").unwrap_or_default();
                wasm_bindgen_futures::spawn_local(async move {
                    let posts: Vec<PostProps> =
                        Request::get("http://mdsns.pigeons.house/api/posts")
                            .header("Google", &token)
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    post_list.set(posts);
                    // let html = cmark(post_list.conte);
                    // let div = web_sys::window()
                    //     .unwrap()
                    //     .document()
                    //     .unwrap()
                    //     .create_element("div")
                    //     .unwrap();
                    // div.set_inner_html(&html);

                    // let node = Node::from(div);
                    // let vnode = VNode::VRef(node);
                });
                || ()
            },
            (),
        );

        // use_effect_with_deps(
        //     move |_| {
        //         // Make a call to DOM API after component is rendered
        //         // value_setter(false);
        //         // let test = test.clone();
        //         post_list.set(demo)

        //         // Perform the cleanup
        //     },
        //     test.clone(),
        // );
    }
    let mut favo_button = |id: String| {
        let onclick = Callback::from(move |e: MouseEvent| {
            let id = id.clone();
            let favo = favo.clone();
            let token: String = LocalStorage::get("token").unwrap_or_default();
            wasm_bindgen_futures::spawn_local(async move {
                // let client = reqwest::Client::new();
                // let res = client
                //     .post(format!(
                //         "{}{}",
                //         "http://mdsns.pigeons.house/api/favorites/", id
                //     ))
                //     // .header(key, value)
                //     .header("Authorization", &token)
                //     // .body(serde_json::to_string(&authorization))
                //     // .form(&authorization)
                //     // .json(&serde_json::to_string(&authorization).unwrap())
                //     // .body(Json(&json!({"post_id": "value"})))
                //     // .json(&(*favo))
                //     .send()
                //     .await
                //     .unwrap()
                //     .json()
                //     .await
                //     .unwrap();
                let favos: i64 =
                    Request::post(&("http://mdsns.pigeons.house/api/favorites/".to_owned() + &id))
                        .header("Google", &token)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                favo.set(favos);
            });
        });
        html! {
            <div>
                <button {onclick}>{"いいね"}</button>
            </div>
        }
    };

    // post_list.set(demo);
    // Callback::from(|_: _| post_list.set(demo));

    html! {
        <div>
            <div>{{"投稿一覧"}}</div>
            <div>
            { for post_list.iter().map(|e| {
                html!{
                    <div>
                    <div>
                    {e.created_at.to_string()}
                    {e.author.name.to_string()}
                    // {e.content_md.to_string()}
                    {{let html = cmark(e.content_md.to_string());
                    let div = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .create_element("div")
                        .unwrap();
                    div.set_inner_html(&html);

                    let node = Node::from(div);
                    let vnode = VNode::VRef(node);
                vnode}}
                    // {favo_button(e.id.clone())}
                    </div>

                    </div>
                }

            }) }
            // <button id="login">{{"test"}}</button>
            </div>
        </div>
    }
}

fn cmark(text: String) -> String {
    let mut options = Options::empty();
    options.insert(
        Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_SMART_PUNCTUATION,
    );
    let parser = Parser::new_ext(&text, options);

    let mut html_output = String::new();
    // parser changes rendered String for markdown
    html::push_html(&mut html_output, parser);

    html_output
}
