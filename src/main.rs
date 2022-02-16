#![allow(non_snake_case)]
use std::rc::Rc;

use dioxus::prelude::*;
use gloo_worker::Bridged;

mod agent;
use crate::agent::{ExampleWorker, ExampleWorkerInput};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    dioxus::web::launch(App);
}

pub fn App(cx: Scope) -> Element {
    let mut worker = ExampleWorker::bridge(Rc::new(move |e| log::info!("{:?}", e)));
    cx.render(rsx! (
        div { "Hello, world!" }
        button {
            onclick: move |_| {
                worker.send(ExampleWorkerInput {
                    n: 10
                })
            },
            "click here "
        }
    ))
}
