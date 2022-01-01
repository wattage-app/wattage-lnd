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

const METHOD_ROUTER_SEND_PAYMENT_V2: ::grpcio::Method<super::router::SendPaymentRequest, super::rpc::Payment> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/routerrpc.Router/SendPaymentV2",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_TRACK_PAYMENT_V2: ::grpcio::Method<super::router::TrackPaymentRequest, super::rpc::Payment> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/routerrpc.Router/TrackPaymentV2",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_ESTIMATE_ROUTE_FEE: ::grpcio::Method<super::router::RouteFeeRequest, super::router::RouteFeeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/EstimateRouteFee",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_SEND_TO_ROUTE: ::grpcio::Method<super::router::SendToRouteRequest, super::router::SendToRouteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/SendToRoute",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_SEND_TO_ROUTE_V2: ::grpcio::Method<super::router::SendToRouteRequest, super::rpc::HTLCAttempt> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/SendToRouteV2",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_RESET_MISSION_CONTROL: ::grpcio::Method<super::router::ResetMissionControlRequest, super::router::ResetMissionControlResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/ResetMissionControl",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_QUERY_MISSION_CONTROL: ::grpcio::Method<super::router::QueryMissionControlRequest, super::router::QueryMissionControlResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/QueryMissionControl",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_X_IMPORT_MISSION_CONTROL: ::grpcio::Method<super::router::XImportMissionControlRequest, super::router::XImportMissionControlResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/XImportMissionControl",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_GET_MISSION_CONTROL_CONFIG: ::grpcio::Method<super::router::GetMissionControlConfigRequest, super::router::GetMissionControlConfigResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/GetMissionControlConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_SET_MISSION_CONTROL_CONFIG: ::grpcio::Method<super::router::SetMissionControlConfigRequest, super::router::SetMissionControlConfigResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/SetMissionControlConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_QUERY_PROBABILITY: ::grpcio::Method<super::router::QueryProbabilityRequest, super::router::QueryProbabilityResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/QueryProbability",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_BUILD_ROUTE: ::grpcio::Method<super::router::BuildRouteRequest, super::router::BuildRouteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/BuildRoute",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_SUBSCRIBE_HTLC_EVENTS: ::grpcio::Method<super::router::SubscribeHtlcEventsRequest, super::router::HtlcEvent> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/routerrpc.Router/SubscribeHtlcEvents",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_SEND_PAYMENT: ::grpcio::Method<super::router::SendPaymentRequest, super::router::PaymentStatus> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/routerrpc.Router/SendPayment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_TRACK_PAYMENT: ::grpcio::Method<super::router::TrackPaymentRequest, super::router::PaymentStatus> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/routerrpc.Router/TrackPayment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_HTLC_INTERCEPTOR: ::grpcio::Method<super::router::ForwardHtlcInterceptResponse, super::router::ForwardHtlcInterceptRequest> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/routerrpc.Router/HtlcInterceptor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTER_UPDATE_CHAN_STATUS: ::grpcio::Method<super::router::UpdateChanStatusRequest, super::router::UpdateChanStatusResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/routerrpc.Router/UpdateChanStatus",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct RouterClient {
    client: ::grpcio::Client,
}

impl RouterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RouterClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn send_payment_v2_opt(&self, req: &super::router::SendPaymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::Payment>> {
        self.client.server_streaming(&METHOD_ROUTER_SEND_PAYMENT_V2, req, opt)
    }

    pub fn send_payment_v2(&self, req: &super::router::SendPaymentRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::Payment>> {
        self.send_payment_v2_opt(req, ::grpcio::CallOption::default())
    }

    pub fn track_payment_v2_opt(&self, req: &super::router::TrackPaymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::Payment>> {
        self.client.server_streaming(&METHOD_ROUTER_TRACK_PAYMENT_V2, req, opt)
    }

    pub fn track_payment_v2(&self, req: &super::router::TrackPaymentRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::Payment>> {
        self.track_payment_v2_opt(req, ::grpcio::CallOption::default())
    }

    pub fn estimate_route_fee_opt(&self, req: &super::router::RouteFeeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::router::RouteFeeResponse> {
        self.client.unary_call(&METHOD_ROUTER_ESTIMATE_ROUTE_FEE, req, opt)
    }

    pub fn estimate_route_fee(&self, req: &super::router::RouteFeeRequest) -> ::grpcio::Result<super::router::RouteFeeResponse> {
        self.estimate_route_fee_opt(req, ::grpcio::CallOption::default())
    }

    pub fn estimate_route_fee_async_opt(&self, req: &super::router::RouteFeeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::RouteFeeResponse>> {
        self.client.unary_call_async(&METHOD_ROUTER_ESTIMATE_ROUTE_FEE, req, opt)
    }

    pub fn estimate_route_fee_async(&self, req: &super::router::RouteFeeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::RouteFeeResponse>> {
        self.estimate_route_fee_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_to_route_opt(&self, req: &super::router::SendToRouteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::router::SendToRouteResponse> {
        self.client.unary_call(&METHOD_ROUTER_SEND_TO_ROUTE, req, opt)
    }

    pub fn send_to_route(&self, req: &super::router::SendToRouteRequest) -> ::grpcio::Result<super::router::SendToRouteResponse> {
        self.send_to_route_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_to_route_async_opt(&self, req: &super::router::SendToRouteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::SendToRouteResponse>> {
        self.client.unary_call_async(&METHOD_ROUTER_SEND_TO_ROUTE, req, opt)
    }

    pub fn send_to_route_async(&self, req: &super::router::SendToRouteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::SendToRouteResponse>> {
        self.send_to_route_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_to_route_v2_opt(&self, req: &super::router::SendToRouteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::HTLCAttempt> {
        self.client.unary_call(&METHOD_ROUTER_SEND_TO_ROUTE_V2, req, opt)
    }

    pub fn send_to_route_v2(&self, req: &super::router::SendToRouteRequest) -> ::grpcio::Result<super::rpc::HTLCAttempt> {
        self.send_to_route_v2_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_to_route_v2_async_opt(&self, req: &super::router::SendToRouteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::HTLCAttempt>> {
        self.client.unary_call_async(&METHOD_ROUTER_SEND_TO_ROUTE_V2, req, opt)
    }

    pub fn send_to_route_v2_async(&self, req: &super::router::SendToRouteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::HTLCAttempt>> {
        self.send_to_route_v2_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reset_mission_control_opt(&self, req: &super::router::ResetMissionControlRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::router::ResetMissionControlResponse> {
        self.client.unary_call(&METHOD_ROUTER_RESET_MISSION_CONTROL, req, opt)
    }

    pub fn reset_mission_control(&self, req: &super::router::ResetMissionControlRequest) -> ::grpcio::Result<super::router::ResetMissionControlResponse> {
        self.reset_mission_control_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reset_mission_control_async_opt(&self, req: &super::router::ResetMissionControlRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::ResetMissionControlResponse>> {
        self.client.unary_call_async(&METHOD_ROUTER_RESET_MISSION_CONTROL, req, opt)
    }

    pub fn reset_mission_control_async(&self, req: &super::router::ResetMissionControlRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::ResetMissionControlResponse>> {
        self.reset_mission_control_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_mission_control_opt(&self, req: &super::router::QueryMissionControlRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::router::QueryMissionControlResponse> {
        self.client.unary_call(&METHOD_ROUTER_QUERY_MISSION_CONTROL, req, opt)
    }

    pub fn query_mission_control(&self, req: &super::router::QueryMissionControlRequest) -> ::grpcio::Result<super::router::QueryMissionControlResponse> {
        self.query_mission_control_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_mission_control_async_opt(&self, req: &super::router::QueryMissionControlRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::QueryMissionControlResponse>> {
        self.client.unary_call_async(&METHOD_ROUTER_QUERY_MISSION_CONTROL, req, opt)
    }

    pub fn query_mission_control_async(&self, req: &super::router::QueryMissionControlRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::QueryMissionControlResponse>> {
        self.query_mission_control_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn x_import_mission_control_opt(&self, req: &super::router::XImportMissionControlRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::router::XImportMissionControlResponse> {
        self.client.unary_call(&METHOD_ROUTER_X_IMPORT_MISSION_CONTROL, req, opt)
    }

    pub fn x_import_mission_control(&self, req: &super::router::XImportMissionControlRequest) -> ::grpcio::Result<super::router::XImportMissionControlResponse> {
        self.x_import_mission_control_opt(req, ::grpcio::CallOption::default())
    }

    pub fn x_import_mission_control_async_opt(&self, req: &super::router::XImportMissionControlRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::XImportMissionControlResponse>> {
        self.client.unary_call_async(&METHOD_ROUTER_X_IMPORT_MISSION_CONTROL, req, opt)
    }

    pub fn x_import_mission_control_async(&self, req: &super::router::XImportMissionControlRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::XImportMissionControlResponse>> {
        self.x_import_mission_control_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_mission_control_config_opt(&self, req: &super::router::GetMissionControlConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::router::GetMissionControlConfigResponse> {
        self.client.unary_call(&METHOD_ROUTER_GET_MISSION_CONTROL_CONFIG, req, opt)
    }

    pub fn get_mission_control_config(&self, req: &super::router::GetMissionControlConfigRequest) -> ::grpcio::Result<super::router::GetMissionControlConfigResponse> {
        self.get_mission_control_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_mission_control_config_async_opt(&self, req: &super::router::GetMissionControlConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::GetMissionControlConfigResponse>> {
        self.client.unary_call_async(&METHOD_ROUTER_GET_MISSION_CONTROL_CONFIG, req, opt)
    }

    pub fn get_mission_control_config_async(&self, req: &super::router::GetMissionControlConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::GetMissionControlConfigResponse>> {
        self.get_mission_control_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_mission_control_config_opt(&self, req: &super::router::SetMissionControlConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::router::SetMissionControlConfigResponse> {
        self.client.unary_call(&METHOD_ROUTER_SET_MISSION_CONTROL_CONFIG, req, opt)
    }

    pub fn set_mission_control_config(&self, req: &super::router::SetMissionControlConfigRequest) -> ::grpcio::Result<super::router::SetMissionControlConfigResponse> {
        self.set_mission_control_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_mission_control_config_async_opt(&self, req: &super::router::SetMissionControlConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::SetMissionControlConfigResponse>> {
        self.client.unary_call_async(&METHOD_ROUTER_SET_MISSION_CONTROL_CONFIG, req, opt)
    }

    pub fn set_mission_control_config_async(&self, req: &super::router::SetMissionControlConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::SetMissionControlConfigResponse>> {
        self.set_mission_control_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_probability_opt(&self, req: &super::router::QueryProbabilityRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::router::QueryProbabilityResponse> {
        self.client.unary_call(&METHOD_ROUTER_QUERY_PROBABILITY, req, opt)
    }

    pub fn query_probability(&self, req: &super::router::QueryProbabilityRequest) -> ::grpcio::Result<super::router::QueryProbabilityResponse> {
        self.query_probability_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_probability_async_opt(&self, req: &super::router::QueryProbabilityRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::QueryProbabilityResponse>> {
        self.client.unary_call_async(&METHOD_ROUTER_QUERY_PROBABILITY, req, opt)
    }

    pub fn query_probability_async(&self, req: &super::router::QueryProbabilityRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::QueryProbabilityResponse>> {
        self.query_probability_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn build_route_opt(&self, req: &super::router::BuildRouteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::router::BuildRouteResponse> {
        self.client.unary_call(&METHOD_ROUTER_BUILD_ROUTE, req, opt)
    }

    pub fn build_route(&self, req: &super::router::BuildRouteRequest) -> ::grpcio::Result<super::router::BuildRouteResponse> {
        self.build_route_opt(req, ::grpcio::CallOption::default())
    }

    pub fn build_route_async_opt(&self, req: &super::router::BuildRouteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::BuildRouteResponse>> {
        self.client.unary_call_async(&METHOD_ROUTER_BUILD_ROUTE, req, opt)
    }

    pub fn build_route_async(&self, req: &super::router::BuildRouteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::BuildRouteResponse>> {
        self.build_route_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_htlc_events_opt(&self, req: &super::router::SubscribeHtlcEventsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::router::HtlcEvent>> {
        self.client.server_streaming(&METHOD_ROUTER_SUBSCRIBE_HTLC_EVENTS, req, opt)
    }

    pub fn subscribe_htlc_events(&self, req: &super::router::SubscribeHtlcEventsRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::router::HtlcEvent>> {
        self.subscribe_htlc_events_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_payment_opt(&self, req: &super::router::SendPaymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::router::PaymentStatus>> {
        self.client.server_streaming(&METHOD_ROUTER_SEND_PAYMENT, req, opt)
    }

    pub fn send_payment(&self, req: &super::router::SendPaymentRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::router::PaymentStatus>> {
        self.send_payment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn track_payment_opt(&self, req: &super::router::TrackPaymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::router::PaymentStatus>> {
        self.client.server_streaming(&METHOD_ROUTER_TRACK_PAYMENT, req, opt)
    }

    pub fn track_payment(&self, req: &super::router::TrackPaymentRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::router::PaymentStatus>> {
        self.track_payment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn htlc_interceptor_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::router::ForwardHtlcInterceptResponse>, ::grpcio::ClientDuplexReceiver<super::router::ForwardHtlcInterceptRequest>)> {
        self.client.duplex_streaming(&METHOD_ROUTER_HTLC_INTERCEPTOR, opt)
    }

    pub fn htlc_interceptor(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::router::ForwardHtlcInterceptResponse>, ::grpcio::ClientDuplexReceiver<super::router::ForwardHtlcInterceptRequest>)> {
        self.htlc_interceptor_opt(::grpcio::CallOption::default())
    }

    pub fn update_chan_status_opt(&self, req: &super::router::UpdateChanStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::router::UpdateChanStatusResponse> {
        self.client.unary_call(&METHOD_ROUTER_UPDATE_CHAN_STATUS, req, opt)
    }

    pub fn update_chan_status(&self, req: &super::router::UpdateChanStatusRequest) -> ::grpcio::Result<super::router::UpdateChanStatusResponse> {
        self.update_chan_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_chan_status_async_opt(&self, req: &super::router::UpdateChanStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::UpdateChanStatusResponse>> {
        self.client.unary_call_async(&METHOD_ROUTER_UPDATE_CHAN_STATUS, req, opt)
    }

    pub fn update_chan_status_async(&self, req: &super::router::UpdateChanStatusRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::router::UpdateChanStatusResponse>> {
        self.update_chan_status_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Router {
    fn send_payment_v2(&mut self, ctx: ::grpcio::RpcContext, req: super::router::SendPaymentRequest, sink: ::grpcio::ServerStreamingSink<super::rpc::Payment>);
    fn track_payment_v2(&mut self, ctx: ::grpcio::RpcContext, req: super::router::TrackPaymentRequest, sink: ::grpcio::ServerStreamingSink<super::rpc::Payment>);
    fn estimate_route_fee(&mut self, ctx: ::grpcio::RpcContext, req: super::router::RouteFeeRequest, sink: ::grpcio::UnarySink<super::router::RouteFeeResponse>);
    fn send_to_route(&mut self, ctx: ::grpcio::RpcContext, req: super::router::SendToRouteRequest, sink: ::grpcio::UnarySink<super::router::SendToRouteResponse>);
    fn send_to_route_v2(&mut self, ctx: ::grpcio::RpcContext, req: super::router::SendToRouteRequest, sink: ::grpcio::UnarySink<super::rpc::HTLCAttempt>);
    fn reset_mission_control(&mut self, ctx: ::grpcio::RpcContext, req: super::router::ResetMissionControlRequest, sink: ::grpcio::UnarySink<super::router::ResetMissionControlResponse>);
    fn query_mission_control(&mut self, ctx: ::grpcio::RpcContext, req: super::router::QueryMissionControlRequest, sink: ::grpcio::UnarySink<super::router::QueryMissionControlResponse>);
    fn x_import_mission_control(&mut self, ctx: ::grpcio::RpcContext, req: super::router::XImportMissionControlRequest, sink: ::grpcio::UnarySink<super::router::XImportMissionControlResponse>);
    fn get_mission_control_config(&mut self, ctx: ::grpcio::RpcContext, req: super::router::GetMissionControlConfigRequest, sink: ::grpcio::UnarySink<super::router::GetMissionControlConfigResponse>);
    fn set_mission_control_config(&mut self, ctx: ::grpcio::RpcContext, req: super::router::SetMissionControlConfigRequest, sink: ::grpcio::UnarySink<super::router::SetMissionControlConfigResponse>);
    fn query_probability(&mut self, ctx: ::grpcio::RpcContext, req: super::router::QueryProbabilityRequest, sink: ::grpcio::UnarySink<super::router::QueryProbabilityResponse>);
    fn build_route(&mut self, ctx: ::grpcio::RpcContext, req: super::router::BuildRouteRequest, sink: ::grpcio::UnarySink<super::router::BuildRouteResponse>);
    fn subscribe_htlc_events(&mut self, ctx: ::grpcio::RpcContext, req: super::router::SubscribeHtlcEventsRequest, sink: ::grpcio::ServerStreamingSink<super::router::HtlcEvent>);
    fn send_payment(&mut self, ctx: ::grpcio::RpcContext, req: super::router::SendPaymentRequest, sink: ::grpcio::ServerStreamingSink<super::router::PaymentStatus>);
    fn track_payment(&mut self, ctx: ::grpcio::RpcContext, req: super::router::TrackPaymentRequest, sink: ::grpcio::ServerStreamingSink<super::router::PaymentStatus>);
    fn htlc_interceptor(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::router::ForwardHtlcInterceptResponse>, sink: ::grpcio::DuplexSink<super::router::ForwardHtlcInterceptRequest>);
    fn update_chan_status(&mut self, ctx: ::grpcio::RpcContext, req: super::router::UpdateChanStatusRequest, sink: ::grpcio::UnarySink<super::router::UpdateChanStatusResponse>);
}

pub fn create_router<S: Router + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_ROUTER_SEND_PAYMENT_V2, move |ctx, req, resp| {
        instance.send_payment_v2(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_ROUTER_TRACK_PAYMENT_V2, move |ctx, req, resp| {
        instance.track_payment_v2(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTER_ESTIMATE_ROUTE_FEE, move |ctx, req, resp| {
        instance.estimate_route_fee(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTER_SEND_TO_ROUTE, move |ctx, req, resp| {
        instance.send_to_route(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTER_SEND_TO_ROUTE_V2, move |ctx, req, resp| {
        instance.send_to_route_v2(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTER_RESET_MISSION_CONTROL, move |ctx, req, resp| {
        instance.reset_mission_control(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTER_QUERY_MISSION_CONTROL, move |ctx, req, resp| {
        instance.query_mission_control(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTER_X_IMPORT_MISSION_CONTROL, move |ctx, req, resp| {
        instance.x_import_mission_control(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTER_GET_MISSION_CONTROL_CONFIG, move |ctx, req, resp| {
        instance.get_mission_control_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTER_SET_MISSION_CONTROL_CONFIG, move |ctx, req, resp| {
        instance.set_mission_control_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTER_QUERY_PROBABILITY, move |ctx, req, resp| {
        instance.query_probability(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTER_BUILD_ROUTE, move |ctx, req, resp| {
        instance.build_route(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_ROUTER_SUBSCRIBE_HTLC_EVENTS, move |ctx, req, resp| {
        instance.subscribe_htlc_events(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_ROUTER_SEND_PAYMENT, move |ctx, req, resp| {
        instance.send_payment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_ROUTER_TRACK_PAYMENT, move |ctx, req, resp| {
        instance.track_payment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_ROUTER_HTLC_INTERCEPTOR, move |ctx, req, resp| {
        instance.htlc_interceptor(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_ROUTER_UPDATE_CHAN_STATUS, move |ctx, req, resp| {
        instance.update_chan_status(ctx, req, resp)
    });
    builder.build()
}
