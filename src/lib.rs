pub mod rpc;
pub mod rpc_grpc;
pub mod walletunlocker;
pub mod walletunlocker_grpc;
pub mod stateservice;
pub mod stateservice_grpc;
pub mod loopd;
use std::sync::Arc;
use crate::rpc_grpc::LightningClient;
use grpcio::{CallOption, MetadataBuilder, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder, Channel};
use crate::walletunlocker_grpc::WalletUnlockerClient;
use crate::stateservice_grpc::StateClient;
use crate::loopd::client_grpc::SwapClientClient;

pub fn get_secure_channel_client(cert_path: &str, addr: &str) -> LightningClient {
    let ch = generate_secure_channel(cert_path, addr);

    LightningClient::new(ch)
}

pub fn get_secure_channel_unlocker(cert_path: &str, addr: &str) -> WalletUnlockerClient {
    let ch = generate_secure_channel(cert_path, addr);

    WalletUnlockerClient::new(ch)
}

pub fn get_secure_channel_stateservice(cert_path: &str, addr: &str) -> StateClient {
	let ch = generate_secure_channel(cert_path, addr);

	StateClient::new(ch)
}

fn generate_secure_channel(cert_path: &str, addr: &str) -> Channel {
    let ca_pem = std::fs::read_to_string(cert_path).expect("Could not read CA Cert file.");
    let env = Arc::new(EnvBuilder::new().build());
    let creds = ChannelCredentialsBuilder::new()
        .root_cert(ca_pem.into_bytes())
        .build();

    ChannelBuilder::new(env).secure_connect(addr, creds)
}

pub fn get_secure_channel_loop_client(cert_path: &str, addr: &str) -> SwapClientClient {
	let ch = generate_secure_channel(cert_path, addr);

	SwapClientClient::new(ch)
}

pub fn lnd_req_opt(macaroon: &Vec<u8>) -> CallOption {
    let mut builder = MetadataBuilder::new();
    builder.add_str("macaroon", &hex::encode(macaroon)).expect("Could not add macaroon to lnd call");
    let meta = builder.build();

    CallOption::default().headers(meta.clone())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
