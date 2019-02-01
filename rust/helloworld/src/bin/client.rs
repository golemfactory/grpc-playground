extern crate grpcio;
extern crate helloworld;

use std::env;
use std::fs::read;
use std::sync::Arc;

use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};

use helloworld::helloworld::HelloRequest;
use helloworld::helloworld_grpc::GreeterClient;


fn main() {
    let args: Vec<String> = env::args().collect();

    let host = &args[1];
    let port = &args[2];
    let addr = format!("{}:{}", host, port);

    let client_cert_path = &args[3];
    let client_key_path = &args[4];
    let server_cert_path = &args[5];

    let client_cert = read(client_cert_path).unwrap();
    let client_key = read(client_key_path).unwrap();
    let server_cert = read(server_cert_path).unwrap();

    let credentials = ChannelCredentialsBuilder::new()
        .cert(client_cert, client_key)
        .root_cert(server_cert)
        .build();

    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env)
        .secure_connect(&addr, credentials);
    let client = GreeterClient::new(channel);

    let mut request = HelloRequest::new();
    request.set_name("World".to_string());

    println!("Sending request...");
    let reply = client.say_hello(&request).unwrap();
    println!("Got reply: \"{}\"", reply.get_message());
}