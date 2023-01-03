use std::{ iter::Sum};

use wasm_bindgen::{JsCast, JsValue, Clamped};
use image::{self, imageops::{resize, dither, BiLevel}, buffer::ConvertBuffer};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use js_sys::{Promise, ArrayBuffer, Uint8Array, Uint8ClampedArray};
use web_sys::{HtmlInputElement, HtmlCanvasElement, CanvasRenderingContext2d, ImageData, File, console};


#[function_component(App)]
pub fn app() -> Html {


    let counter= use_state(|| 0);
    let print = use_state(|| "".to_string());
    let input_ref = use_node_ref(); 
    let img_ref = use_node_ref();

    let onchange =  {
        let input_ref = input_ref.clone();
        // let img_ref = img_ref.clone();
        let counter = counter.clone();
        let print = print.clone();
        Callback::from(  move |_| {
            let input = input_ref.cast::<HtmlInputElement>();
     
            // let imgRef = img_ref.cast::<HtmlCanvasElement>();
            if let Some(input) = input {
                // web_sys::console::log_1(&"Hi".to_string().into());
                let file = input.files().unwrap().item(0).unwrap();
                counter.set(1);
                // let image = imgRef.unwrap();
                let print = print.clone();
                // web_sys::console::log_1(&file);
                spawn_local( async move{
                    let contents = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await.unwrap();
                    // web_sys::console::log_1(&"Await".to_string().into());
                    let u8Vec= Uint8Array::new(&contents).to_vec();

                    // web_sys::console::log_1(&"To Rust vec".to_string().into());
                    let mut img = image::load_from_memory_with_format(&u8Vec, image::ImageFormat::Png).unwrap();
                    img = img.grayscale();
                    web_sys::console::log_1(&"Load from memory on rust side".to_string().into());
                    
                    let img_reformat = img;
                    
                    let img_resized = resize(&img_reformat, 200, 200, image::imageops::FilterType::Gaussian);
                    let num_ranges = 5;
                    let interval = 255 /num_ranges;

                    let mut out_string = String::new();
                    let chars_to_use = [" ", ".", "*", "%", "#"];
                    web_sys::console::log_3(&"W, H: ".into(), &img_resized.width().into(), &img_resized.height().into());
                    for i in 0..img_resized.height(){
                        for j in 0..img_resized.width(){
                            let pix:&[u8] = &img_resized.get_pixel(j,i).0[0..3];
                            // console::log_1(&"GOT HERE".into());
                            let mut val:u32 = pix.iter().map(|x| *x as u32).sum::<u32>() / 3;
                            if val == 255{
                                val = 0;
                            }
                            // console::log_1(&val.into());
                            let mut which_interval = 0;
                            let mut left_over:i32 = val.try_into().unwrap();
                            // console::log_4(&left_over.into(), &which_interval.into(), &interval.into(), &(left_over - interval).into());
                            while left_over - interval > 0 {
                                // web_sys::console::log_2(&which_interval.into(), &left_over.into());
                                *&mut left_over = *&left_over - *&interval;
                                *&mut which_interval +=1;
                            };
                            out_string +=chars_to_use[which_interval];

                        };
                        out_string.push('\n');
                    }
                    
                    web_sys::console::log_1(&"Rgb conversion, and resize".to_string().into());
                    

                    // let ctx = image.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
                    // let clamped = Clamped(img_resized.as_raw().as_slice());
                    // let img_data = ImageData::new_with_u8_clamped_array(clamped, img_resized.width()).unwrap();

                    web_sys::console::log_1(&"Imagedata creation".to_string().into());
                    // ctx.put_image_data(&img_data, 0.0, 0.0);
                    // ctx.set_fill_style(&"#22c55e".into());
                    // ctx.set_font("8px sans-serif");
                    // let splits = out_string.lines();
                    // println!("{:?}", splits);
                    // web_sys::console::log_1(&out_string.clone().into());

                    // for (ix, line) in splits.enumerate(){
                    //     web_sys::console::log_1(&line.into());
                    //     let out = ctx.fill_text(&line, 600.0, (ix+1) as f64 * 10.0 );
                    // }
                    
                    let print = print.clone();
                    print.set(out_string.to_string()); 



                });

            }
            // let ctx = img_ref.cast::<HtmlCanvasElement>().unwrap().get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
            // ctx.set_fill_style(&"#ffffff".into());
            // ctx.set_font("8px serif");
            // ctx.fill_text("Hello mumma", 10.0, 10.0);


        })
    };
    html! {
        <main class="w-full h-screen fixed bg-black text-green-500 flex flex-col items-center">
        <div class="w-full h-full overflow-auto flex flex-col items-center">
            <h1>{ "a s c i i f y" }</h1>

            if *counter == 0{<div class="w-10/12 lg:w-5/12 h-4/6 rounded-xl border border-green-500">
                <input ref={input_ref} type="file" onchange={onchange}/> 
            </div>}

            <pre class="text-[10px] leading-[5px] tracking-normal text-green-500">{&print[..]}</pre>
            // <canvas width={1600} height={1000} ref={img_ref}></canvas>
            
        </div>
        </main>
    }
}
