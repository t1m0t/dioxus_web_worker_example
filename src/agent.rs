use gloo_worker::{HandlerId, Private, Worker, WorkerLink};
use serde::{Deserialize, Serialize};

pub struct ExampleWorker {
    link: WorkerLink<Self>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExampleWorkerInput {
    pub n: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExampleWorkerOutput {
    pub value: u32,
}

impl Worker for ExampleWorker {
    type Reach = Private<Self>;
    type Message = ();
    type Input = ExampleWorkerInput;
    type Output = ExampleWorkerOutput;

    fn create(link: WorkerLink<Self>) -> Self {
        log::info!("worker created");
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {
        // no messaging
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        // this runs in a web worker
        // and does not block the main
        // browser thread!
        log::info!("something received");
        let n = msg.n;

        fn fib(n: u32) -> u32 {
            if n <= 1 {
                1
            } else {
                fib(n - 1) + fib(n - 2)
            }
        }

        let output = Self::Output { value: fib(n) };

        self.link.respond(id, output);
    }

    fn name_of_resource() -> &'static str {
        "static/wasm.js"
    }
}
