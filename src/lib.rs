use wasm_bindgen::prelude::*;
use web_sys::console;

// use yew::prelude::*;;;;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/template-deep-dive/wee_alloc.html#what-is-wee_alloc
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Loop through samples directory
    // Eventually I'll be getting this from an API
    // Hardcoding for now!
    let mut vec = Vec::new();
    vec.push("IMG_20200427_184619981_HDR.jpg");
    vec.push("IMG_20200326_183831454.jpg");
    vec.push("IMG_20200406_132301544.jpg");
    for x in &vec {
        console::log_1(&JsValue::from_str(x));
    }
    console::log_1(&JsValue::from_str("Hello world!"));

    demo();

    Ok(())
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, test-wasm!");
}

#[wasm_bindgen]
pub fn get_photos() {
    return create_demo_photos()
}

fn create_demo_photos() {
    // Eventually get this information from DB.
    // let photo_names = 
    // let photo_names = [
    //     "1.jpg"
    // ];
    let path = "samples/1.png";

    // let first_photo = Photo { path };

    // return photo_names
}

fn get_sample_photos() {
    // Create models and map to file name..
}

struct Photo {
    // id: u64, // Is this needed?
    path: str
    // name, Have two values? Human readable, and url encoded?
    // description
}

pub struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Photos" }</h1>
                <img src="samples/IMG_20200406_132301544.jpg"/>
                <button onclick=self.link.callback(|_| Msg::Click)>{ "Click ( wasm-bindgen )" }</button>
            </div>
        }
    }
}

fn demo() {
    yew::start_app::<Model>();
}
