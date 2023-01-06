use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use tauri_invalid_args_mre_ui::Greeting;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs {
    greeting: Greeting,
}

#[function_component(App)]
pub fn app() -> Html {
    spawn_local(async move {
        let greeting = Greeting::Hello(None); // error
                                              // let greeting = Greeting::Hello(Some("Alice".to_string())); // works

        invoke("greet", to_value(&GreetArgs { greeting }).unwrap()).await;
    });

    html! {
        <main>
        </main>
    }
}
