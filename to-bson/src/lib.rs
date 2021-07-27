mod utils;
use bson::*;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Example {
    pub bson: Bson
}

#[derive(Serialize, Deserialize)]
pub struct Disk {
    pub CD: Vec<Song>,
    pub TIME: i64
}

#[derive(Serialize, Deserialize)]
pub struct Song {
    pub TITLE: String,
    pub ARTIST: String,
    pub COUNTRY: String,
    pub COMPANY: String,
    pub PRICE: String,
    pub YEAR: String
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn get_bson(val: &JsValue) -> JsValue {
    let disk: Disk = val.into_serde().unwrap();
    let bson: Bson = to_bson(&disk).unwrap();
    let example = Example{
        bson,
    };
    JsValue::from_serde(&example).unwrap()
}

#[wasm_bindgen]
pub fn get_json(val: &JsValue) -> JsValue {
    let example: Example = val.into_serde().unwrap();
    let disk: Disk = from_bson(example.bson).unwrap();
    JsValue::from_serde(&disk).unwrap()
}
