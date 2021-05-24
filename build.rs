extern crate protoc_grpcio;

use curl::easy::Easy;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut proto = File::create("src/protos/rpc.proto")?;
    let proto_root = "src/protos";
    let output_root = "src";

    println!("cargo:rerun-if-changed={}", proto_root);
    let mut easy = Easy::new();
    easy.url("https://raw.githubusercontent.com/lightningnetwork/lnd/master/lnrpc/rpc.proto").unwrap();
    easy.write_function(move |data| {
        proto.write(data).expect("Could not write to src/protos/rpc.proto");
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    protoc_grpcio::compile_grpc_protos(&["rpc.proto"], &[proto_root], &output_root, None)
        .expect("Failed to compile gRPC definitions!");

    Ok(())
}
