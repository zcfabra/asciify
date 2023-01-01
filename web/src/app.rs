use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::{HtmlInputElement};
#[function_component(App)]
pub fn app() -> Html {



    let input_ref = use_node_ref(); 
    let onchange = {
        let input_ref = input_ref.clone();

        Callback::from(  move |_|{
            let input = input_ref.cast::<HtmlInputElement>();
            if let Some(input) = input {
                web_sys::console::log_1(&"Hi".to_string().into());
                let file = input.files().unwrap().item(0).unwrap();
                web_sys::console::log_1(&file);
                let contents = file.array_buffer();
                spawn_local(async {
                    let conts = wasm_bindgen_futures::JsFuture::from(contents).await.unwrap();
                    web_sys::console::log_1(&conts);
                });
            }
        })
    };
    html! {
        <main class="w-full h-full bg-black text-green-500 flex flex-col items-center">
            <h1>{ "a s c i i f y" }</h1>
            <div class="w-10/12 lg:w-5/12 h-4/6 rounded-xl border border-green-500">
                <input ref={input_ref} type="file" {onchange}/> 
            </div>
        </main>
    }
}
