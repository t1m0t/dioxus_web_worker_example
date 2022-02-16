#![allow(non_snake_case)]
use std::rc::Rc;

use dioxus::prelude::*;
use gloo_worker::Bridged;
use serde::__private::de::IdentifierDeserializer;

mod agent;
use crate::agent::{ExampleWorker, ExampleWorkerInput};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    dioxus::web::launch(App);
}

pub fn App(cx: Scope) -> Element {
    let (result, set_result) = use_state(&cx, || 0 as u32);
    let mut worker = ExampleWorker::bridge(Rc::new(move |e| log::info!("{:?}", e)));
    let (input, set_input) = use_state(&cx, move || 0 as u32);
    cx.render(rsx! (
        div { "Fibonacci example" }
        input {
            r#type: "text",
            value: "0",
            oninput: move |e| {
                set_input(e.value)
            }
        }
        button {
            onclick: move |e| {
                worker.send(ExampleWorkerInput {
                    n: 10
                });
            },
            "click me"
        }
    ))
}
