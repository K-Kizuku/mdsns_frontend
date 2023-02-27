use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(Auth)]
pub fn auth() -> Html {
    // wasm-bindgen will automatically take care of including this script
    // #[wasm_bindgen(module = "/auth0.js")]
    // extern "C" {
    //     #[wasm_bindgen(js_name = "login")]
    //     pub fn login();
    // }

    fn login_auth() {
        #[wasm_bindgen(module = "/auth0.js")]
        extern "C" {
            fn login();
        }

        #[wasm_bindgen]
        pub fn temp() {
            login()
        }
        temp();
    }
    let button_login = {
        // let test = SignInProps { email: "".to_string(), password: "".to_string() };
        // let authorization = authorization.clone();
        let onclick = Callback::from(move |e: MouseEvent| {
            // let authorization = authorization.clone();

            // wasm_bindgen_futures::spawn_local(async move {
            //     // let authorization = authorization.clone();
            //     // let temp = &authorization;
            //     // let auth_json = serde_json::to_string(&authorization.serialize(serializer)).unwrap();
            //     // let post_data = json!(authorization);
            //     // Request::post("http://httpbin.org/post").json(&authorization).await;
            //     let post_data = SignUpProps {
            //         name: String::from(authorization.name.clone()),
            //         email: String::from(authorization.email.clone()),
            //         password: String::from(authorization.password.clone()),
            //     };

            //     let client = reqwest::Client::new();
            //     let res = client
            //         .post("http://megalo.pigeons.house/api/signup")
            //         // .body(serde_json::to_string(&authorization))
            //         // .form(&authorization)
            //         // .json(&serde_json::to_string(&authorization).unwrap())
            //         .json(&post_data)
            //         .send()
            //         .await
            //         .unwrap()
            //         .text()
            //         .await;
            //     LocalStorage::set("jwt", res.ok());
            // });
            login_auth();
        });
        html! {
            <button class="primary-button" {onclick}>{"ログイン"}</button>
        }
    };
    html! {
        <div>
            <div class="header">{"authだよ"}</div>
            {button_login}
        </div>
    }
}
