#![allow(non_snake_case)]
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    fmt::{self},
    rc::Rc,
};

use dioxus::prelude::*;
use gloo_worker::Bridged;

mod agent;
use crate::agent::{ExampleWorker, ExampleWorkerInput};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    dioxus::web::launch(App);
}

struct ComputedValue<T> {
    value: RefCell<T>,
}

impl<T> fmt::Display for ComputedValue<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.borrow())
    }
}

impl<T> ComputedValue<T> {
    fn new(val: T) -> Self {
        Self {
            value: RefCell::new(val),
        }
    }
}

pub fn App(cx: Scope) -> Element {
    let val = Rc::new(ComputedValue::new(0 as u32));
    let status = Rc::new(ComputedValue::new(false));
    let (result, set_result) = use_state(&cx, || 0 as u32);

    let mut worker = ExampleWorker::bridge(Rc::new({
        let inside_val = val.clone();
        let status = status.clone();
        move |e| {
            *inside_val.value.borrow_mut() = e.value;
            *status.value.borrow_mut() = true;
        }
    }));

    let (input, set_input) = use_state(&cx, move || 0 as u32);
    let (error, set_error) = use_state(&cx, move || false);
    let (error_message, set_error_message) = use_state(&cx, move || "");

    let input_message = if *error {
        set_error_message("error: input should be between 1 and 50");
        cx.render(rsx! {
            "{error_message}"
        })
    } else {
        cx.render(rsx! {
            "entered value {input}"
        })
    };

    let result_message = if *status.value.borrow_mut() {
        set_result(*val.value.borrow_mut());
        cx.render(rsx! {
            "result of computation is: {result}"
        })
    } else {
        cx.render(rsx! {
            "no computed value yet!"
        })
    };

    cx.render(rsx! (
        div { "Fibonacci example" }
        input {
            r#type: "text",
            placeholder: "0",
            oninput: move |e| {
                match e.value.parse::<u32>() {
                    Ok(val) => {
                        if val > 0 && val < 51 {
                            if *error {set_error(false)} else {};
                            set_input(val)
                        } else {
                            set_error(true);
                            set_input(0)
                        }
                    },
                    Err(_) => {()}
                }
            }
        }
        button {
            onclick: move |_| {
                worker.send(ExampleWorkerInput {
                    n: *input
                });
            },
            "click me to compute"
        }
        div {
            input_message
        }
        div {
            result_message
        }
        div {
            "{val}"
        }
    ))
}
