#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
use js_sys::{Array, Reflect, Uint8Array};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{
    Blob, BlobPropertyBag, DedicatedWorkerGlobalScope, MessageEvent, Url, Worker, WorkerOptions,
};

pub mod agent;
use gloo_worker::PrivateWorker;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    agent::ExampleWorker::register();
}

fn worker_new(name_of_resource: &str, resource_is_relative: bool, is_module: bool) -> Worker {
    let blob = make_blob(resource_is_relative, name_of_resource);
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    let worker = make_worker(is_module, url);

    worker
}

fn make_worker(is_module: bool, url: String) -> Worker {
    let worker = if is_module {
        let options = WorkerOptions::new();
        Reflect::set(
            options.as_ref(),
            &JsValue::from_str("type"),
            &JsValue::from_str("module"),
        )
        .unwrap();
        web_sys::Worker::new_with_options(&url, &options).expect("failed to spawn worker")
    } else {
        web_sys::Worker::new(&url).expect("failed to spawn worker")
    };

    worker
}

fn make_blob(resource_is_relative: bool, name_of_resource: &str) -> Blob {
    let origin = gloo_utils::document()
        .location()
        .unwrap_throw()
        .origin()
        .unwrap_throw();
    let pathname = gloo_utils::window().location().pathname().unwrap_throw();
    let prefix = if resource_is_relative {
        pathname
            .rfind(|c| c == '/')
            .map(|i| &pathname[..i])
            .unwrap_or_default()
    } else {
        ""
    };

    let script_url = format!("{}{}/{}", origin, prefix, name_of_resource);
    let wasm_url = format!(
        "{}{}/{}",
        origin,
        prefix,
        name_of_resource.replace(".js", "_bg.wasm")
    );

    let array = Array::new();
    array.push(
        &format!(
            r#"importScripts("{}");wasm_bindgen("{}");"#,
            script_url, wasm_url
        )
        .into(),
    );
    let blob = Blob::new_with_str_sequence_and_options(
        &array,
        BlobPropertyBag::new().type_("application/javascript"),
    )
    .unwrap();
    blob
}

//to be used for worker_self().post_message_vec(data); after
fn worker_self() -> DedicatedWorkerGlobalScope {
    JsValue::from(js_sys::global()).into()
}

trait WorkerExt {
    fn set_onmessage_closure(&self, handler: impl 'static + Fn(Vec<u8>));

    fn post_message_vec(&self, data: Vec<u8>);
}

macro_rules! worker_ext_impl {
    ($($type:path),+) => {$(
        impl WorkerExt for $type {
            fn set_onmessage_closure(&self, handler: impl 'static + Fn(Vec<u8>)) {
                let handler = move |message: MessageEvent| {
                    let data = Uint8Array::from(message.data()).to_vec();
                    handler(data);
                };
                let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(MessageEvent)>);
                self.set_onmessage(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            }

            fn post_message_vec(&self, data: Vec<u8>) {
                self.post_message(&Uint8Array::from(data.as_slice()))
                    .expect("failed to post message");
            }
        }
    )+};
}

worker_ext_impl! {
    web_sys::Worker, DedicatedWorkerGlobalScope
}

pub fn start_worker() {
    let worker = worker_new("static/module/i18n/wasm-i18n.js", true, true);
    let mess = JsValue::from("worker");
    let _ = worker_self().post_message(&mess);
}
