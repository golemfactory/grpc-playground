extern crate crossbeam_channel;
extern crate ctrlc;
extern crate futures;
extern crate grpcio;
extern crate helloworld;

use std::sync::Arc;

use crossbeam_channel::{unbounded};

use futures::Future;

use grpcio::{EnvBuilder, RpcContext, ServerBuilder, UnarySink};

use helloworld::helloworld::{HelloRequest, HelloReply};
use helloworld::helloworld_grpc::{create_greeter, Greeter};

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {
        println!("Got message from {}", req.get_name());

        let message = format!("Hello, {}!", req.get_name());
        let mut reply = HelloReply::new();
        reply.set_message(message);
        let result = sink.success(reply)
            .map_err(move |e| println!("Failed to reply {:?}: {:?}", req, e));
        ctx.spawn(result)
    }
}

fn main() {
    let env = Arc::new(EnvBuilder::new().cq_count(1).build());
    let greeter = create_greeter(GreeterService);
    let mut server = ServerBuilder::new(env)
        .register_service(greeter)
        .bind("127.0.0.1", 54321)
        .build()
        .unwrap();

    // Start server
    println!("Starting server...");
    server.start();
    println!("Started.");

    // Run until closed
    let (sender, receiver) = unbounded();
    ctrlc::set_handler(move || {
        sender.send(()).unwrap();
    }).unwrap();
    receiver.recv().unwrap();

    // Shutdown server
    println!("Shutting down server...");
    server.shutdown().wait().unwrap();
}