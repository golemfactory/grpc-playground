extern crate crossbeam_channel;
extern crate ctrlc;
extern crate futures;
extern crate grpcio;
extern crate helloworld;

use std::env;
use std::fs::read;
use std::sync::Arc;

use crossbeam_channel::{unbounded};

use futures::Future;

use grpcio::{EnvBuilder, RpcContext, Server, ServerBuilder, ServerCredentialsBuilder, UnarySink};

use helloworld::helloworld::{HelloRequest, HelloReply};
use helloworld::helloworld_grpc::{create_greeter, Greeter};


#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext, req: HelloRequest, sink: UnarySink<HelloReply>) {

        println!("Got message from {}.", req.get_name());
        let message = format!("Hello, {}!", req.get_name());

        let mut reply = HelloReply::new();
        reply.set_message(message);

        let result = sink.success(reply)
            .map_err(move |e| println!("Failed to reply {:?}: {:?}", req, e));
        ctx.spawn(result)
    }
}

fn build_server(args: Vec<String>) -> Server {
    let host = args[1].to_string();
    let port = args[2].parse::<u16>().unwrap();

    let server_cert_path = &args[3];
    let server_key_path = &args[4];
    let client_cert_path = &args[5];

    let server_cert = read(server_cert_path).unwrap();
    let server_key = read(server_key_path).unwrap();
    let client_cert = read(client_cert_path).unwrap();

    let credentials = ServerCredentialsBuilder::new()
        .add_cert(server_cert, server_key)
        .root_cert(client_cert, true)
        .build();

    let env = Arc::new(EnvBuilder::new().cq_count(1).build());
    let greeter = create_greeter(GreeterService);

    ServerBuilder::new(env)
        .register_service(greeter)
        .bind_secure(host, port, credentials)
        .build()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut server = build_server(args);

    // Start server
    println!("Starting server...");
    server.start();
    println!("Started. Press Ctrl+c to stop.");

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