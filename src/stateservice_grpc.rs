// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_STATE_SUBSCRIBE_STATE: ::grpcio::Method<super::stateservice::SubscribeStateRequest, super::stateservice::SubscribeStateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.State/SubscribeState",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_STATE_GET_STATE: ::grpcio::Method<super::stateservice::GetStateRequest, super::stateservice::GetStateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.State/GetState",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct StateClient {
    client: ::grpcio::Client,
}

impl StateClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        StateClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn subscribe_state_opt(&self, req: &super::stateservice::SubscribeStateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::stateservice::SubscribeStateResponse>> {
        self.client.server_streaming(&METHOD_STATE_SUBSCRIBE_STATE, req, opt)
    }

    pub fn subscribe_state(&self, req: &super::stateservice::SubscribeStateRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::stateservice::SubscribeStateResponse>> {
        self.subscribe_state_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_state_opt(&self, req: &super::stateservice::GetStateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::stateservice::GetStateResponse> {
        self.client.unary_call(&METHOD_STATE_GET_STATE, req, opt)
    }

    pub fn get_state(&self, req: &super::stateservice::GetStateRequest) -> ::grpcio::Result<super::stateservice::GetStateResponse> {
        self.get_state_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_state_async_opt(&self, req: &super::stateservice::GetStateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::stateservice::GetStateResponse>> {
        self.client.unary_call_async(&METHOD_STATE_GET_STATE, req, opt)
    }

    pub fn get_state_async(&self, req: &super::stateservice::GetStateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::stateservice::GetStateResponse>> {
        self.get_state_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait State {
    fn subscribe_state(&mut self, ctx: ::grpcio::RpcContext, req: super::stateservice::SubscribeStateRequest, sink: ::grpcio::ServerStreamingSink<super::stateservice::SubscribeStateResponse>);
    fn get_state(&mut self, ctx: ::grpcio::RpcContext, req: super::stateservice::GetStateRequest, sink: ::grpcio::UnarySink<super::stateservice::GetStateResponse>);
}

pub fn create_state<S: State + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_STATE_SUBSCRIBE_STATE, move |ctx, req, resp| {
        instance.subscribe_state(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_STATE_GET_STATE, move |ctx, req, resp| {
        instance.get_state(ctx, req, resp)
    });
    builder.build()
}
