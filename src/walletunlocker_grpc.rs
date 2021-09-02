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

const METHOD_WALLET_UNLOCKER_GEN_SEED: ::grpcio::Method<super::walletunlocker::GenSeedRequest, super::walletunlocker::GenSeedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.WalletUnlocker/GenSeed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_WALLET_UNLOCKER_INIT_WALLET: ::grpcio::Method<super::walletunlocker::InitWalletRequest, super::walletunlocker::InitWalletResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.WalletUnlocker/InitWallet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_WALLET_UNLOCKER_UNLOCK_WALLET: ::grpcio::Method<super::walletunlocker::UnlockWalletRequest, super::walletunlocker::UnlockWalletResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.WalletUnlocker/UnlockWallet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_WALLET_UNLOCKER_CHANGE_PASSWORD: ::grpcio::Method<super::walletunlocker::ChangePasswordRequest, super::walletunlocker::ChangePasswordResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.WalletUnlocker/ChangePassword",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct WalletUnlockerClient {
    client: ::grpcio::Client,
}

impl WalletUnlockerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        WalletUnlockerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn gen_seed_opt(&self, req: &super::walletunlocker::GenSeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::walletunlocker::GenSeedResponse> {
        self.client.unary_call(&METHOD_WALLET_UNLOCKER_GEN_SEED, req, opt)
    }

    pub fn gen_seed(&self, req: &super::walletunlocker::GenSeedRequest) -> ::grpcio::Result<super::walletunlocker::GenSeedResponse> {
        self.gen_seed_opt(req, ::grpcio::CallOption::default())
    }

    pub fn gen_seed_async_opt(&self, req: &super::walletunlocker::GenSeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::walletunlocker::GenSeedResponse>> {
        self.client.unary_call_async(&METHOD_WALLET_UNLOCKER_GEN_SEED, req, opt)
    }

    pub fn gen_seed_async(&self, req: &super::walletunlocker::GenSeedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::walletunlocker::GenSeedResponse>> {
        self.gen_seed_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn init_wallet_opt(&self, req: &super::walletunlocker::InitWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::walletunlocker::InitWalletResponse> {
        self.client.unary_call(&METHOD_WALLET_UNLOCKER_INIT_WALLET, req, opt)
    }

    pub fn init_wallet(&self, req: &super::walletunlocker::InitWalletRequest) -> ::grpcio::Result<super::walletunlocker::InitWalletResponse> {
        self.init_wallet_opt(req, ::grpcio::CallOption::default())
    }

    pub fn init_wallet_async_opt(&self, req: &super::walletunlocker::InitWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::walletunlocker::InitWalletResponse>> {
        self.client.unary_call_async(&METHOD_WALLET_UNLOCKER_INIT_WALLET, req, opt)
    }

    pub fn init_wallet_async(&self, req: &super::walletunlocker::InitWalletRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::walletunlocker::InitWalletResponse>> {
        self.init_wallet_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unlock_wallet_opt(&self, req: &super::walletunlocker::UnlockWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::walletunlocker::UnlockWalletResponse> {
        self.client.unary_call(&METHOD_WALLET_UNLOCKER_UNLOCK_WALLET, req, opt)
    }

    pub fn unlock_wallet(&self, req: &super::walletunlocker::UnlockWalletRequest) -> ::grpcio::Result<super::walletunlocker::UnlockWalletResponse> {
        self.unlock_wallet_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unlock_wallet_async_opt(&self, req: &super::walletunlocker::UnlockWalletRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::walletunlocker::UnlockWalletResponse>> {
        self.client.unary_call_async(&METHOD_WALLET_UNLOCKER_UNLOCK_WALLET, req, opt)
    }

    pub fn unlock_wallet_async(&self, req: &super::walletunlocker::UnlockWalletRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::walletunlocker::UnlockWalletResponse>> {
        self.unlock_wallet_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn change_password_opt(&self, req: &super::walletunlocker::ChangePasswordRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::walletunlocker::ChangePasswordResponse> {
        self.client.unary_call(&METHOD_WALLET_UNLOCKER_CHANGE_PASSWORD, req, opt)
    }

    pub fn change_password(&self, req: &super::walletunlocker::ChangePasswordRequest) -> ::grpcio::Result<super::walletunlocker::ChangePasswordResponse> {
        self.change_password_opt(req, ::grpcio::CallOption::default())
    }

    pub fn change_password_async_opt(&self, req: &super::walletunlocker::ChangePasswordRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::walletunlocker::ChangePasswordResponse>> {
        self.client.unary_call_async(&METHOD_WALLET_UNLOCKER_CHANGE_PASSWORD, req, opt)
    }

    pub fn change_password_async(&self, req: &super::walletunlocker::ChangePasswordRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::walletunlocker::ChangePasswordResponse>> {
        self.change_password_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait WalletUnlocker {
    fn gen_seed(&mut self, ctx: ::grpcio::RpcContext, req: super::walletunlocker::GenSeedRequest, sink: ::grpcio::UnarySink<super::walletunlocker::GenSeedResponse>);
    fn init_wallet(&mut self, ctx: ::grpcio::RpcContext, req: super::walletunlocker::InitWalletRequest, sink: ::grpcio::UnarySink<super::walletunlocker::InitWalletResponse>);
    fn unlock_wallet(&mut self, ctx: ::grpcio::RpcContext, req: super::walletunlocker::UnlockWalletRequest, sink: ::grpcio::UnarySink<super::walletunlocker::UnlockWalletResponse>);
    fn change_password(&mut self, ctx: ::grpcio::RpcContext, req: super::walletunlocker::ChangePasswordRequest, sink: ::grpcio::UnarySink<super::walletunlocker::ChangePasswordResponse>);
}

pub fn create_wallet_unlocker<S: WalletUnlocker + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_WALLET_UNLOCKER_GEN_SEED, move |ctx, req, resp| {
        instance.gen_seed(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_WALLET_UNLOCKER_INIT_WALLET, move |ctx, req, resp| {
        instance.init_wallet(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_WALLET_UNLOCKER_UNLOCK_WALLET, move |ctx, req, resp| {
        instance.unlock_wallet(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_WALLET_UNLOCKER_CHANGE_PASSWORD, move |ctx, req, resp| {
        instance.change_password(ctx, req, resp)
    });
    builder.build()
}
