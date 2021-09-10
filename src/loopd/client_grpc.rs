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

const METHOD_SWAP_CLIENT_LOOP_OUT: ::grpcio::Method<super::client::LoopOutRequest, super::client::SwapResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/LoopOut",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_LOOP_IN: ::grpcio::Method<super::client::LoopInRequest, super::client::SwapResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/LoopIn",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_MONITOR: ::grpcio::Method<super::client::MonitorRequest, super::client::SwapStatus> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/looprpc.SwapClient/Monitor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_LIST_SWAPS: ::grpcio::Method<super::client::ListSwapsRequest, super::client::ListSwapsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/ListSwaps",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_SWAP_INFO: ::grpcio::Method<super::client::SwapInfoRequest, super::client::SwapStatus> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/SwapInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_LOOP_OUT_TERMS: ::grpcio::Method<super::client::TermsRequest, super::client::OutTermsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/LoopOutTerms",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_LOOP_OUT_QUOTE: ::grpcio::Method<super::client::QuoteRequest, super::client::OutQuoteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/LoopOutQuote",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_GET_LOOP_IN_TERMS: ::grpcio::Method<super::client::TermsRequest, super::client::InTermsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/GetLoopInTerms",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_GET_LOOP_IN_QUOTE: ::grpcio::Method<super::client::QuoteRequest, super::client::InQuoteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/GetLoopInQuote",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_PROBE: ::grpcio::Method<super::client::ProbeRequest, super::client::ProbeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/Probe",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_GET_LSAT_TOKENS: ::grpcio::Method<super::client::TokensRequest, super::client::TokensResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/GetLsatTokens",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_GET_LIQUIDITY_PARAMS: ::grpcio::Method<super::client::GetLiquidityParamsRequest, super::client::LiquidityParameters> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/GetLiquidityParams",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_SET_LIQUIDITY_PARAMS: ::grpcio::Method<super::client::SetLiquidityParamsRequest, super::client::SetLiquidityParamsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/SetLiquidityParams",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SWAP_CLIENT_SUGGEST_SWAPS: ::grpcio::Method<super::client::SuggestSwapsRequest, super::client::SuggestSwapsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/looprpc.SwapClient/SuggestSwaps",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SwapClientClient {
    client: ::grpcio::Client,
}

impl SwapClientClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SwapClientClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn loop_out_opt(&self, req: &super::client::LoopOutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::SwapResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_LOOP_OUT, req, opt)
    }

    pub fn loop_out(&self, req: &super::client::LoopOutRequest) -> ::grpcio::Result<super::client::SwapResponse> {
        self.loop_out_opt(req, ::grpcio::CallOption::default())
    }

    pub fn loop_out_async_opt(&self, req: &super::client::LoopOutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::SwapResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_LOOP_OUT, req, opt)
    }

    pub fn loop_out_async(&self, req: &super::client::LoopOutRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::SwapResponse>> {
        self.loop_out_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn loop_in_opt(&self, req: &super::client::LoopInRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::SwapResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_LOOP_IN, req, opt)
    }

    pub fn loop_in(&self, req: &super::client::LoopInRequest) -> ::grpcio::Result<super::client::SwapResponse> {
        self.loop_in_opt(req, ::grpcio::CallOption::default())
    }

    pub fn loop_in_async_opt(&self, req: &super::client::LoopInRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::SwapResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_LOOP_IN, req, opt)
    }

    pub fn loop_in_async(&self, req: &super::client::LoopInRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::SwapResponse>> {
        self.loop_in_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn monitor_opt(&self, req: &super::client::MonitorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::client::SwapStatus>> {
        self.client.server_streaming(&METHOD_SWAP_CLIENT_MONITOR, req, opt)
    }

    pub fn monitor(&self, req: &super::client::MonitorRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::client::SwapStatus>> {
        self.monitor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_swaps_opt(&self, req: &super::client::ListSwapsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::ListSwapsResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_LIST_SWAPS, req, opt)
    }

    pub fn list_swaps(&self, req: &super::client::ListSwapsRequest) -> ::grpcio::Result<super::client::ListSwapsResponse> {
        self.list_swaps_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_swaps_async_opt(&self, req: &super::client::ListSwapsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::ListSwapsResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_LIST_SWAPS, req, opt)
    }

    pub fn list_swaps_async(&self, req: &super::client::ListSwapsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::ListSwapsResponse>> {
        self.list_swaps_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn swap_info_opt(&self, req: &super::client::SwapInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::SwapStatus> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_SWAP_INFO, req, opt)
    }

    pub fn swap_info(&self, req: &super::client::SwapInfoRequest) -> ::grpcio::Result<super::client::SwapStatus> {
        self.swap_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn swap_info_async_opt(&self, req: &super::client::SwapInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::SwapStatus>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_SWAP_INFO, req, opt)
    }

    pub fn swap_info_async(&self, req: &super::client::SwapInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::SwapStatus>> {
        self.swap_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn loop_out_terms_opt(&self, req: &super::client::TermsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::OutTermsResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_LOOP_OUT_TERMS, req, opt)
    }

    pub fn loop_out_terms(&self, req: &super::client::TermsRequest) -> ::grpcio::Result<super::client::OutTermsResponse> {
        self.loop_out_terms_opt(req, ::grpcio::CallOption::default())
    }

    pub fn loop_out_terms_async_opt(&self, req: &super::client::TermsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::OutTermsResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_LOOP_OUT_TERMS, req, opt)
    }

    pub fn loop_out_terms_async(&self, req: &super::client::TermsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::OutTermsResponse>> {
        self.loop_out_terms_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn loop_out_quote_opt(&self, req: &super::client::QuoteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::OutQuoteResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_LOOP_OUT_QUOTE, req, opt)
    }

    pub fn loop_out_quote(&self, req: &super::client::QuoteRequest) -> ::grpcio::Result<super::client::OutQuoteResponse> {
        self.loop_out_quote_opt(req, ::grpcio::CallOption::default())
    }

    pub fn loop_out_quote_async_opt(&self, req: &super::client::QuoteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::OutQuoteResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_LOOP_OUT_QUOTE, req, opt)
    }

    pub fn loop_out_quote_async(&self, req: &super::client::QuoteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::OutQuoteResponse>> {
        self.loop_out_quote_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_loop_in_terms_opt(&self, req: &super::client::TermsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::InTermsResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_GET_LOOP_IN_TERMS, req, opt)
    }

    pub fn get_loop_in_terms(&self, req: &super::client::TermsRequest) -> ::grpcio::Result<super::client::InTermsResponse> {
        self.get_loop_in_terms_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_loop_in_terms_async_opt(&self, req: &super::client::TermsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::InTermsResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_GET_LOOP_IN_TERMS, req, opt)
    }

    pub fn get_loop_in_terms_async(&self, req: &super::client::TermsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::InTermsResponse>> {
        self.get_loop_in_terms_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_loop_in_quote_opt(&self, req: &super::client::QuoteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::InQuoteResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_GET_LOOP_IN_QUOTE, req, opt)
    }

    pub fn get_loop_in_quote(&self, req: &super::client::QuoteRequest) -> ::grpcio::Result<super::client::InQuoteResponse> {
        self.get_loop_in_quote_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_loop_in_quote_async_opt(&self, req: &super::client::QuoteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::InQuoteResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_GET_LOOP_IN_QUOTE, req, opt)
    }

    pub fn get_loop_in_quote_async(&self, req: &super::client::QuoteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::InQuoteResponse>> {
        self.get_loop_in_quote_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn probe_opt(&self, req: &super::client::ProbeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::ProbeResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_PROBE, req, opt)
    }

    pub fn probe(&self, req: &super::client::ProbeRequest) -> ::grpcio::Result<super::client::ProbeResponse> {
        self.probe_opt(req, ::grpcio::CallOption::default())
    }

    pub fn probe_async_opt(&self, req: &super::client::ProbeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::ProbeResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_PROBE, req, opt)
    }

    pub fn probe_async(&self, req: &super::client::ProbeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::ProbeResponse>> {
        self.probe_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_lsat_tokens_opt(&self, req: &super::client::TokensRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::TokensResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_GET_LSAT_TOKENS, req, opt)
    }

    pub fn get_lsat_tokens(&self, req: &super::client::TokensRequest) -> ::grpcio::Result<super::client::TokensResponse> {
        self.get_lsat_tokens_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_lsat_tokens_async_opt(&self, req: &super::client::TokensRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::TokensResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_GET_LSAT_TOKENS, req, opt)
    }

    pub fn get_lsat_tokens_async(&self, req: &super::client::TokensRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::TokensResponse>> {
        self.get_lsat_tokens_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_liquidity_params_opt(&self, req: &super::client::GetLiquidityParamsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::LiquidityParameters> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_GET_LIQUIDITY_PARAMS, req, opt)
    }

    pub fn get_liquidity_params(&self, req: &super::client::GetLiquidityParamsRequest) -> ::grpcio::Result<super::client::LiquidityParameters> {
        self.get_liquidity_params_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_liquidity_params_async_opt(&self, req: &super::client::GetLiquidityParamsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::LiquidityParameters>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_GET_LIQUIDITY_PARAMS, req, opt)
    }

    pub fn get_liquidity_params_async(&self, req: &super::client::GetLiquidityParamsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::LiquidityParameters>> {
        self.get_liquidity_params_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_liquidity_params_opt(&self, req: &super::client::SetLiquidityParamsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::SetLiquidityParamsResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_SET_LIQUIDITY_PARAMS, req, opt)
    }

    pub fn set_liquidity_params(&self, req: &super::client::SetLiquidityParamsRequest) -> ::grpcio::Result<super::client::SetLiquidityParamsResponse> {
        self.set_liquidity_params_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_liquidity_params_async_opt(&self, req: &super::client::SetLiquidityParamsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::SetLiquidityParamsResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_SET_LIQUIDITY_PARAMS, req, opt)
    }

    pub fn set_liquidity_params_async(&self, req: &super::client::SetLiquidityParamsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::SetLiquidityParamsResponse>> {
        self.set_liquidity_params_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn suggest_swaps_opt(&self, req: &super::client::SuggestSwapsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::client::SuggestSwapsResponse> {
        self.client.unary_call(&METHOD_SWAP_CLIENT_SUGGEST_SWAPS, req, opt)
    }

    pub fn suggest_swaps(&self, req: &super::client::SuggestSwapsRequest) -> ::grpcio::Result<super::client::SuggestSwapsResponse> {
        self.suggest_swaps_opt(req, ::grpcio::CallOption::default())
    }

    pub fn suggest_swaps_async_opt(&self, req: &super::client::SuggestSwapsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::SuggestSwapsResponse>> {
        self.client.unary_call_async(&METHOD_SWAP_CLIENT_SUGGEST_SWAPS, req, opt)
    }

    pub fn suggest_swaps_async(&self, req: &super::client::SuggestSwapsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::client::SuggestSwapsResponse>> {
        self.suggest_swaps_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SwapClient {
    fn loop_out(&mut self, ctx: ::grpcio::RpcContext, req: super::client::LoopOutRequest, sink: ::grpcio::UnarySink<super::client::SwapResponse>);
    fn loop_in(&mut self, ctx: ::grpcio::RpcContext, req: super::client::LoopInRequest, sink: ::grpcio::UnarySink<super::client::SwapResponse>);
    fn monitor(&mut self, ctx: ::grpcio::RpcContext, req: super::client::MonitorRequest, sink: ::grpcio::ServerStreamingSink<super::client::SwapStatus>);
    fn list_swaps(&mut self, ctx: ::grpcio::RpcContext, req: super::client::ListSwapsRequest, sink: ::grpcio::UnarySink<super::client::ListSwapsResponse>);
    fn swap_info(&mut self, ctx: ::grpcio::RpcContext, req: super::client::SwapInfoRequest, sink: ::grpcio::UnarySink<super::client::SwapStatus>);
    fn loop_out_terms(&mut self, ctx: ::grpcio::RpcContext, req: super::client::TermsRequest, sink: ::grpcio::UnarySink<super::client::OutTermsResponse>);
    fn loop_out_quote(&mut self, ctx: ::grpcio::RpcContext, req: super::client::QuoteRequest, sink: ::grpcio::UnarySink<super::client::OutQuoteResponse>);
    fn get_loop_in_terms(&mut self, ctx: ::grpcio::RpcContext, req: super::client::TermsRequest, sink: ::grpcio::UnarySink<super::client::InTermsResponse>);
    fn get_loop_in_quote(&mut self, ctx: ::grpcio::RpcContext, req: super::client::QuoteRequest, sink: ::grpcio::UnarySink<super::client::InQuoteResponse>);
    fn probe(&mut self, ctx: ::grpcio::RpcContext, req: super::client::ProbeRequest, sink: ::grpcio::UnarySink<super::client::ProbeResponse>);
    fn get_lsat_tokens(&mut self, ctx: ::grpcio::RpcContext, req: super::client::TokensRequest, sink: ::grpcio::UnarySink<super::client::TokensResponse>);
    fn get_liquidity_params(&mut self, ctx: ::grpcio::RpcContext, req: super::client::GetLiquidityParamsRequest, sink: ::grpcio::UnarySink<super::client::LiquidityParameters>);
    fn set_liquidity_params(&mut self, ctx: ::grpcio::RpcContext, req: super::client::SetLiquidityParamsRequest, sink: ::grpcio::UnarySink<super::client::SetLiquidityParamsResponse>);
    fn suggest_swaps(&mut self, ctx: ::grpcio::RpcContext, req: super::client::SuggestSwapsRequest, sink: ::grpcio::UnarySink<super::client::SuggestSwapsResponse>);
}

pub fn create_swap_client<S: SwapClient + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_LOOP_OUT, move |ctx, req, resp| {
        instance.loop_out(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_LOOP_IN, move |ctx, req, resp| {
        instance.loop_in(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_SWAP_CLIENT_MONITOR, move |ctx, req, resp| {
        instance.monitor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_LIST_SWAPS, move |ctx, req, resp| {
        instance.list_swaps(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_SWAP_INFO, move |ctx, req, resp| {
        instance.swap_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_LOOP_OUT_TERMS, move |ctx, req, resp| {
        instance.loop_out_terms(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_LOOP_OUT_QUOTE, move |ctx, req, resp| {
        instance.loop_out_quote(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_GET_LOOP_IN_TERMS, move |ctx, req, resp| {
        instance.get_loop_in_terms(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_GET_LOOP_IN_QUOTE, move |ctx, req, resp| {
        instance.get_loop_in_quote(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_PROBE, move |ctx, req, resp| {
        instance.probe(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_GET_LSAT_TOKENS, move |ctx, req, resp| {
        instance.get_lsat_tokens(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_GET_LIQUIDITY_PARAMS, move |ctx, req, resp| {
        instance.get_liquidity_params(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_SET_LIQUIDITY_PARAMS, move |ctx, req, resp| {
        instance.set_liquidity_params(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_SWAP_CLIENT_SUGGEST_SWAPS, move |ctx, req, resp| {
        instance.suggest_swaps(ctx, req, resp)
    });
    builder.build()
}
