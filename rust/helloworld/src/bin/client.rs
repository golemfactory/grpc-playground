extern crate grpcio;
extern crate helloworld;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use helloworld::helloworld::HelloRequest;
use helloworld::helloworld_grpc::GreeterClient;



fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect("127.0.0.1:54321");
    let client = GreeterClient::new(channel);

    let mut request = HelloRequest::new();
    request.set_name("World".to_string());
    let reply = client.say_hello(&request).unwrap();
    println!("Received reply: \"{}\"", reply.get_message());
}