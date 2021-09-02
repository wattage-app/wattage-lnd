extern crate protoc_grpcio;

use curl::easy::Easy;
use std::fs::File;
use std::io::Write;
use protoc_rust::Customize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "src/protos";
    let output_root = "src";

    println!("cargo:rerun-if-changed={}", proto_root);
    let fns = vec!["rpc.proto", "walletunlocker.proto", "stateservice.proto"];

    // for filename in &fns {
    //     let rpc_proto_url = format!("https://raw.githubusercontent.com/lightningnetwork/lnd/v0.12.1-beta/lnrpc/{}", filename);
    //     let write_path = format!("src/protos/{}", filename);
    //     fetch_proto(rpc_proto_url.as_str(), write_path.as_str())?;
    // }

    protoc_grpcio::compile_grpc_protos(fns.as_slice(), &[proto_root], &output_root, Some(Customize {
        serde_derive: Some(true),
        ..Default::default()
    }))
        .expect("Failed to compile gRPC definitions!");

    Ok(())
}

fn fetch_proto(proto_url: &str, write_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut easy = Easy::new();
    let mut rpc_proto = File::create(write_path)?;
    easy.url(proto_url).unwrap();
    easy.write_function(move |data| {
        rpc_proto.write(data).expect("Could not write to src/protos/rpc.proto");
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    Ok(())
}
