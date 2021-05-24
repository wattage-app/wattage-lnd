pub mod rpc;
pub mod rpc_grpc;

use std::sync::Arc;
use crate::rpc_grpc::LightningClient;
use grpcio::{CallOption, MetadataBuilder, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};

pub fn get_secure_channel_client(cert_path: &str, addr: &str) -> LightningClient {
    let ca_pem = std::fs::read_to_string(cert_path).expect("Could not read CA Cert file.");
    let env = Arc::new(EnvBuilder::new().build());
    let creds = ChannelCredentialsBuilder::new()
        .root_cert(ca_pem.into_bytes())
        .build();

    let ch = ChannelBuilder::new(env).secure_connect(addr, creds);
    LightningClient::new(ch)
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
