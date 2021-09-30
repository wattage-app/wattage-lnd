extern crate protoc_grpcio;

use curl::easy::Easy;
use std::fs::File;
use std::io::Write;
use protoc_rust::Customize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "src/protos";
    let output_root = "src";
	let root_protos = vec!["rpc.proto", "walletunlocker.proto", "stateservice.proto", "invoices.proto"];

	compile_protos(proto_root, output_root, root_protos);
    Ok(())
}

fn compile_protos(proto_root: &str, output_root: &str, root_protos: Vec<&str>) {
	protoc_grpcio::compile_grpc_protos(root_protos.as_slice(), &[proto_root], &output_root, Some(Customize {
		serde_derive: Some(true),
		..Default::default()
	}))
		.expect("Failed to compile gRPC definitions!");
}
