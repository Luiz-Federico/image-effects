use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{engine::general_purpose, Engine as _};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"grayscale called".into());

    let base64_to_vector = general_purpose::STANDARD.decode(encoded_file).unwrap();
    log(&"image decoded".into());

    let mut image = load_from_memory(&base64_to_vector).unwrap();

    log(&"image loaded".into());

    image = image.grayscale();
    log(&"grayscale effect applied".into());

    let mut buffer = vec![];
    image.write_to(&mut buffer, Png ).unwrap();
    log(&"new image written".into());

    let encoded_img = general_purpose::STANDARD.encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);

    return data_url;

}