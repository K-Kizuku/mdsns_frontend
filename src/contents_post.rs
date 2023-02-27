use gloo::storage::LocalStorage;
use gloo_storage::Storage;

use pulldown_cmark::{html, Options, Parser};
use reqwest;
use web_sys::Node;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[function_component(ContentsPost)]
pub fn contents_post() -> Html {
    let input_md = use_state(|| String::from(""));
    let input_md = input_md.clone();
    // let vnode_ref = use_state(|| );
    let input_textarea = {
        let input_md = input_md.clone();
        let input_md2 = input_md.clone();

        let onchange = Callback::from(move |e: InputEvent| {
            let input_md = input_md.clone();
            let input = e.target_dyn_into::<HtmlTextAreaElement>();
            // if let Some(input) = input {
            input_md.set(input.expect("REASON").value());
            LocalStorage::set("test", input_md.to_string()).unwrap();
            // }
        });
        let html = cmark(input_md2.to_string());
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&html);

        let node = Node::from(div);
        let vnode = VNode::VRef(node);
        // let input_md = input_md.clone();
        // let html = cmark(input_md.to_string());
        // let div = web_sys::window()
        //     .unwrap()
        //     .document()
        //     .unwrap()
        //     .create_element("div")
        //     .unwrap();
        // div.set_inner_html(&html);

        // let node = Node::from(div);
        // let vnode = VNode::VRef(node);

        // let input_md = input_md.clone();
        html! {
            <div>
                <textarea oninput={onchange} />
                {vnode}
            </div>
        }
    };

    let post_button = {
        let onclick = Callback::from(move |e: MouseEvent| {
            let input_md = input_md.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = reqwest::Client::new();
                let res = client
                    .post("https://app.swaggerhub.com/apis/BULLBULL1230/MDSNS/1.0.0#/api/posts")
                    // .body(serde_json::to_string(&authorization))
                    // .form(&authorization)
                    // .json(&serde_json::to_string(&authorization).unwrap())
                    .json(&(*input_md))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await;
            });
        });
        html! {
            <div>
                <button {onclick}>{"投稿"}</button>
            </div>
        }
    };
    html! {
        <div>
            <h1>{"投稿画面だよ"}</h1>
            {input_textarea}
            {post_button}
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
