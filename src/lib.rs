use wasm_bindgen::prelude::*;
//use wasm_bindgen::JsCast;
//use js_sys::Object;
//use crate::game::{Game, Creep};

//mod game;

macro_rules! log {
    ($($args:tt)*) => {
        web_sys::console::log_1(&JsValue::from(format!($($args)*)));
    };
}


#[wasm_bindgen(js_name = loop)]
pub fn loop_() {
    //console_error_panic_hook::set_once();

    log!("Hello world!");

    /*for spawn in Object::values(&Game.spawns()).iter() {

    }

    for creep in Object::values(&Game.creeps()).iter() {
        let creep = creep.unchecked_into::<Creep>();
    }

    for structure in Object::values(&Game.structures()).iter() {

    }*/
}
