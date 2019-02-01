# gRPC Playgorund

## Features

1. Based on Protocol Buffers and HTTP/2
1. Supports SSL
1. Data types: bool, int, float, string, bytes, list, map, structs...
1. Pre-defined set of [error codes](https://github.com/grpc/grpc/blob/master/doc/statuscodes.md)
1. Streaming RPC -- sending multiple messages per channel (uni- or bidirectional)
1. Multiplexing -- handling multiple channels over one connection
1. Deadlines and cancels

## Python

`grpcio` library is needed only for file generation not for runtime.

[`grpclib`](https://grpclib.readthedocs.io/en/latest/) provides asyncio-compatible gRPC interface for Python.

To re-generate Python definitions run:
```
python -m grpc_tools.protoc --proto_path=. --python_out=python  --python_grpc_out=python helloworld.proto
```

## Rust

Available libs:
1. [`grpc-rust`](https://github.com/stepancheg/grpc-rust) -- Pure Rust implementation, under development (generally works, but has poor performance). Potentially Tokio-compatible (See [this issue](https://github.com/stepancheg/grpc-rust/issues/117)).
1. [`grpc-rs`](https://github.com/pingcap/grpc-rs) -- Wrapper for C++, under development (some features are missing, e.g. reflection, custom metadata and authentication) -- **this one is used here**
1. [`tower-grpc`](https://github.com/tower-rs/tower-grpc) -- Pure Rust implementation based on Tower, very immature.

To re-generate Rust definitions install `protobuf-codegen` and `grpcio-compiler` and run:
```
python -m grpc_tools.protoc 
    --proto_path=.
    --rust_out=rust/helloworld/src
    --grpc_out=rust/helloworld/src
    --plugin=protoc-gen-grpc="&lt;PATH_TO_`grpc_rust_plugin`_EXECUTABLE&gt;"
    helloworld.proto
```

## Other notes

`grpc_cli` has to be built from sources [link](https://github.com/grpc/grpc/blob/master/BUILDING.md)



