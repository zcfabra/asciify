use wasm_bindgen::{JsCast, JsValue, Clamped};
use image::{self, imageops::resize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use js_sys::{Promise, ArrayBuffer, Uint8Array, Uint8ClampedArray};
use web_sys::{HtmlInputElement, HtmlCanvasElement, CanvasRenderingContext2d, ImageData, File};


#[function_component(App)]
pub fn app() -> Html {


    let counter= use_state(|| 0);
    let input_ref = use_node_ref(); 
    let img_ref = use_node_ref();
    async fn get_image_array_buffer(to_resolve: js_sys::Promise)-> JsValue {
        return wasm_bindgen_futures::JsFuture::from(to_resolve).await.unwrap();
    }
    let onchange =   {
        let input_ref = input_ref.clone();
        let img_ref = img_ref.clone();
        let counter = counter.clone();
        Callback::from(  move |_| {
            let input = input_ref.cast::<HtmlInputElement>();
            let imgRef = img_ref.cast::<HtmlCanvasElement>();
            if let Some(input) = input {
                web_sys::console::log_1(&"Hi".to_string().into());
                let file = input.files().unwrap().item(0).unwrap();
                counter.set(1);
                let image = imgRef.unwrap();
                web_sys::console::log_1(&file);
                spawn_local( async move{
                    let contents = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await.unwrap();
                    // web_sys::console::log_1(&"Await".to_string().into());
                    let u8Vec= Uint8Array::new(&contents).to_vec();

                    // web_sys::console::log_1(&"To Rust vec".to_string().into());
                    let img = image::load_from_memory_with_format(&u8Vec, image::ImageFormat::Png).unwrap();
                    web_sys::console::log_1(&"Load from memory on rust side".to_string().into());
                    let img_reformat = img.to_rgba8();
                    let img_resized = resize(&img_reformat, 300, 300, image::imageops::FilterType::Gaussian);

                    web_sys::console::log_1(&"Rgb conversion, and resize".to_string().into());

                    let ctx = image.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
                    let clamped = Clamped(img_resized.as_raw().as_slice());
                    let img_data = ImageData::new_with_u8_clamped_array(clamped, img_resized.width()).unwrap();

                    // web_sys::console::log_1(&"Imagedata creation".to_string().into());
                    ctx.put_image_data(&img_data, 0.0, 0.0);

                    // web_sys::console::log_1(&"Draw ".to_string().into());


                })

            }
        })
    };
    html! {
        <main class="w-full h-full bg-black text-green-500 flex flex-col items-center">
            <h1>{ "a s c i i f y" }</h1>

            if *counter == 0{<div class="w-10/12 lg:w-5/12 h-4/6 rounded-xl border border-green-500">
                <input ref={input_ref} type="file" onchange={onchange}/> 
            </div>}
            <canvas width={300} height={300} ref={img_ref}></canvas>
            
        </main>
    }
}
