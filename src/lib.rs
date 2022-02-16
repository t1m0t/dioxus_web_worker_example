#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]

pub mod agent;
use gloo_worker::PrivateWorker;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    agent::ExampleWorker::register();
}
