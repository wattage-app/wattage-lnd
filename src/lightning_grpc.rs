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

const METHOD_LIGHTNING_WALLET_BALANCE: ::grpcio::Method<super::lightning::WalletBalanceRequest, super::lightning::WalletBalanceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/WalletBalance",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CHANNEL_BALANCE: ::grpcio::Method<super::lightning::ChannelBalanceRequest, super::lightning::ChannelBalanceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ChannelBalance",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_TRANSACTIONS: ::grpcio::Method<super::lightning::GetTransactionsRequest, super::lightning::TransactionDetails> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetTransactions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_ESTIMATE_FEE: ::grpcio::Method<super::lightning::EstimateFeeRequest, super::lightning::EstimateFeeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/EstimateFee",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_COINS: ::grpcio::Method<super::lightning::SendCoinsRequest, super::lightning::SendCoinsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SendCoins",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_UNSPENT: ::grpcio::Method<super::lightning::ListUnspentRequest, super::lightning::ListUnspentResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListUnspent",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_TRANSACTIONS: ::grpcio::Method<super::lightning::GetTransactionsRequest, super::lightning::Transaction> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeTransactions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_MANY: ::grpcio::Method<super::lightning::SendManyRequest, super::lightning::SendManyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SendMany",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_NEW_ADDRESS: ::grpcio::Method<super::lightning::NewAddressRequest, super::lightning::NewAddressResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/NewAddress",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SIGN_MESSAGE: ::grpcio::Method<super::lightning::SignMessageRequest, super::lightning::SignMessageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SignMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_VERIFY_MESSAGE: ::grpcio::Method<super::lightning::VerifyMessageRequest, super::lightning::VerifyMessageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/VerifyMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CONNECT_PEER: ::grpcio::Method<super::lightning::ConnectPeerRequest, super::lightning::ConnectPeerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ConnectPeer",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DISCONNECT_PEER: ::grpcio::Method<super::lightning::DisconnectPeerRequest, super::lightning::DisconnectPeerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DisconnectPeer",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_PEERS: ::grpcio::Method<super::lightning::ListPeersRequest, super::lightning::ListPeersResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListPeers",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_PEER_EVENTS: ::grpcio::Method<super::lightning::PeerEventSubscription, super::lightning::PeerEvent> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribePeerEvents",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_INFO: ::grpcio::Method<super::lightning::GetInfoRequest, super::lightning::GetInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_RECOVERY_INFO: ::grpcio::Method<super::lightning::GetRecoveryInfoRequest, super::lightning::GetRecoveryInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetRecoveryInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_PENDING_CHANNELS: ::grpcio::Method<super::lightning::PendingChannelsRequest, super::lightning::PendingChannelsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/PendingChannels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_CHANNELS: ::grpcio::Method<super::lightning::ListChannelsRequest, super::lightning::ListChannelsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListChannels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_EVENTS: ::grpcio::Method<super::lightning::ChannelEventSubscription, super::lightning::ChannelEventUpdate> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeChannelEvents",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CLOSED_CHANNELS: ::grpcio::Method<super::lightning::ClosedChannelsRequest, super::lightning::ClosedChannelsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ClosedChannels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_OPEN_CHANNEL_SYNC: ::grpcio::Method<super::lightning::OpenChannelRequest, super::lightning::ChannelPoint> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/OpenChannelSync",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_OPEN_CHANNEL: ::grpcio::Method<super::lightning::OpenChannelRequest, super::lightning::OpenStatusUpdate> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/OpenChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_BATCH_OPEN_CHANNEL: ::grpcio::Method<super::lightning::BatchOpenChannelRequest, super::lightning::BatchOpenChannelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/BatchOpenChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_FUNDING_STATE_STEP: ::grpcio::Method<super::lightning::FundingTransitionMsg, super::lightning::FundingStateStepResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/FundingStateStep",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CHANNEL_ACCEPTOR: ::grpcio::Method<super::lightning::ChannelAcceptResponse, super::lightning::ChannelAcceptRequest> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/lnrpc.Lightning/ChannelAcceptor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CLOSE_CHANNEL: ::grpcio::Method<super::lightning::CloseChannelRequest, super::lightning::CloseStatusUpdate> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/CloseChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_ABANDON_CHANNEL: ::grpcio::Method<super::lightning::AbandonChannelRequest, super::lightning::AbandonChannelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/AbandonChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_PAYMENT: ::grpcio::Method<super::lightning::SendRequest, super::lightning::SendResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/lnrpc.Lightning/SendPayment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_PAYMENT_SYNC: ::grpcio::Method<super::lightning::SendRequest, super::lightning::SendResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SendPaymentSync",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_TO_ROUTE: ::grpcio::Method<super::lightning::SendToRouteRequest, super::lightning::SendResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/lnrpc.Lightning/SendToRoute",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_TO_ROUTE_SYNC: ::grpcio::Method<super::lightning::SendToRouteRequest, super::lightning::SendResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SendToRouteSync",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_ADD_INVOICE: ::grpcio::Method<super::lightning::Invoice, super::lightning::AddInvoiceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/AddInvoice",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_INVOICES: ::grpcio::Method<super::lightning::ListInvoiceRequest, super::lightning::ListInvoiceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListInvoices",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LOOKUP_INVOICE: ::grpcio::Method<super::lightning::PaymentHash, super::lightning::Invoice> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/LookupInvoice",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_INVOICES: ::grpcio::Method<super::lightning::InvoiceSubscription, super::lightning::Invoice> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeInvoices",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DECODE_PAY_REQ: ::grpcio::Method<super::lightning::PayReqString, super::lightning::PayReq> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DecodePayReq",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_PAYMENTS: ::grpcio::Method<super::lightning::ListPaymentsRequest, super::lightning::ListPaymentsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListPayments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DELETE_PAYMENT: ::grpcio::Method<super::lightning::DeletePaymentRequest, super::lightning::DeletePaymentResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DeletePayment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DELETE_ALL_PAYMENTS: ::grpcio::Method<super::lightning::DeleteAllPaymentsRequest, super::lightning::DeleteAllPaymentsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DeleteAllPayments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DESCRIBE_GRAPH: ::grpcio::Method<super::lightning::ChannelGraphRequest, super::lightning::ChannelGraph> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DescribeGraph",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_NODE_METRICS: ::grpcio::Method<super::lightning::NodeMetricsRequest, super::lightning::NodeMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetNodeMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_CHAN_INFO: ::grpcio::Method<super::lightning::ChanInfoRequest, super::lightning::ChannelEdge> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetChanInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_NODE_INFO: ::grpcio::Method<super::lightning::NodeInfoRequest, super::lightning::NodeInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetNodeInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_QUERY_ROUTES: ::grpcio::Method<super::lightning::QueryRoutesRequest, super::lightning::QueryRoutesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/QueryRoutes",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_NETWORK_INFO: ::grpcio::Method<super::lightning::NetworkInfoRequest, super::lightning::NetworkInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetNetworkInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_STOP_DAEMON: ::grpcio::Method<super::lightning::StopRequest, super::lightning::StopResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/StopDaemon",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_GRAPH: ::grpcio::Method<super::lightning::GraphTopologySubscription, super::lightning::GraphTopologyUpdate> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeChannelGraph",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DEBUG_LEVEL: ::grpcio::Method<super::lightning::DebugLevelRequest, super::lightning::DebugLevelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DebugLevel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_FEE_REPORT: ::grpcio::Method<super::lightning::FeeReportRequest, super::lightning::FeeReportResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/FeeReport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_UPDATE_CHANNEL_POLICY: ::grpcio::Method<super::lightning::PolicyUpdateRequest, super::lightning::PolicyUpdateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/UpdateChannelPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_FORWARDING_HISTORY: ::grpcio::Method<super::lightning::ForwardingHistoryRequest, super::lightning::ForwardingHistoryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ForwardingHistory",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_EXPORT_CHANNEL_BACKUP: ::grpcio::Method<super::lightning::ExportChannelBackupRequest, super::lightning::ChannelBackup> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ExportChannelBackup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_EXPORT_ALL_CHANNEL_BACKUPS: ::grpcio::Method<super::lightning::ChanBackupExportRequest, super::lightning::ChanBackupSnapshot> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ExportAllChannelBackups",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_VERIFY_CHAN_BACKUP: ::grpcio::Method<super::lightning::ChanBackupSnapshot, super::lightning::VerifyChanBackupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/VerifyChanBackup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_RESTORE_CHANNEL_BACKUPS: ::grpcio::Method<super::lightning::RestoreChanBackupRequest, super::lightning::RestoreBackupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/RestoreChannelBackups",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_BACKUPS: ::grpcio::Method<super::lightning::ChannelBackupSubscription, super::lightning::ChanBackupSnapshot> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeChannelBackups",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_BAKE_MACAROON: ::grpcio::Method<super::lightning::BakeMacaroonRequest, super::lightning::BakeMacaroonResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/BakeMacaroon",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_MACAROON_I_DS: ::grpcio::Method<super::lightning::ListMacaroonIDsRequest, super::lightning::ListMacaroonIDsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListMacaroonIDs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DELETE_MACAROON_ID: ::grpcio::Method<super::lightning::DeleteMacaroonIDRequest, super::lightning::DeleteMacaroonIDResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DeleteMacaroonID",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_PERMISSIONS: ::grpcio::Method<super::lightning::ListPermissionsRequest, super::lightning::ListPermissionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListPermissions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CHECK_MACAROON_PERMISSIONS: ::grpcio::Method<super::lightning::CheckMacPermRequest, super::lightning::CheckMacPermResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/CheckMacaroonPermissions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_REGISTER_RPC_MIDDLEWARE: ::grpcio::Method<super::lightning::RPCMiddlewareResponse, super::lightning::RPCMiddlewareRequest> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/lnrpc.Lightning/RegisterRPCMiddleware",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_CUSTOM_MESSAGE: ::grpcio::Method<super::lightning::SendCustomMessageRequest, super::lightning::SendCustomMessageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SendCustomMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_CUSTOM_MESSAGES: ::grpcio::Method<super::lightning::SubscribeCustomMessagesRequest, super::lightning::CustomMessage> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeCustomMessages",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct LightningClient {
    client: ::grpcio::Client,
}

impl LightningClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        LightningClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn wallet_balance_opt(&self, req: &super::lightning::WalletBalanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::WalletBalanceResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_WALLET_BALANCE, req, opt)
    }

    pub fn wallet_balance(&self, req: &super::lightning::WalletBalanceRequest) -> ::grpcio::Result<super::lightning::WalletBalanceResponse> {
        self.wallet_balance_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wallet_balance_async_opt(&self, req: &super::lightning::WalletBalanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::WalletBalanceResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_WALLET_BALANCE, req, opt)
    }

    pub fn wallet_balance_async(&self, req: &super::lightning::WalletBalanceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::WalletBalanceResponse>> {
        self.wallet_balance_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_balance_opt(&self, req: &super::lightning::ChannelBalanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ChannelBalanceResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_CHANNEL_BALANCE, req, opt)
    }

    pub fn channel_balance(&self, req: &super::lightning::ChannelBalanceRequest) -> ::grpcio::Result<super::lightning::ChannelBalanceResponse> {
        self.channel_balance_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_balance_async_opt(&self, req: &super::lightning::ChannelBalanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChannelBalanceResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_CHANNEL_BALANCE, req, opt)
    }

    pub fn channel_balance_async(&self, req: &super::lightning::ChannelBalanceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChannelBalanceResponse>> {
        self.channel_balance_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_transactions_opt(&self, req: &super::lightning::GetTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::TransactionDetails> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_TRANSACTIONS, req, opt)
    }

    pub fn get_transactions(&self, req: &super::lightning::GetTransactionsRequest) -> ::grpcio::Result<super::lightning::TransactionDetails> {
        self.get_transactions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_transactions_async_opt(&self, req: &super::lightning::GetTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::TransactionDetails>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_TRANSACTIONS, req, opt)
    }

    pub fn get_transactions_async(&self, req: &super::lightning::GetTransactionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::TransactionDetails>> {
        self.get_transactions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn estimate_fee_opt(&self, req: &super::lightning::EstimateFeeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::EstimateFeeResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_ESTIMATE_FEE, req, opt)
    }

    pub fn estimate_fee(&self, req: &super::lightning::EstimateFeeRequest) -> ::grpcio::Result<super::lightning::EstimateFeeResponse> {
        self.estimate_fee_opt(req, ::grpcio::CallOption::default())
    }

    pub fn estimate_fee_async_opt(&self, req: &super::lightning::EstimateFeeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::EstimateFeeResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_ESTIMATE_FEE, req, opt)
    }

    pub fn estimate_fee_async(&self, req: &super::lightning::EstimateFeeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::EstimateFeeResponse>> {
        self.estimate_fee_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_coins_opt(&self, req: &super::lightning::SendCoinsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::SendCoinsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SEND_COINS, req, opt)
    }

    pub fn send_coins(&self, req: &super::lightning::SendCoinsRequest) -> ::grpcio::Result<super::lightning::SendCoinsResponse> {
        self.send_coins_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_coins_async_opt(&self, req: &super::lightning::SendCoinsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SendCoinsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SEND_COINS, req, opt)
    }

    pub fn send_coins_async(&self, req: &super::lightning::SendCoinsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SendCoinsResponse>> {
        self.send_coins_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_unspent_opt(&self, req: &super::lightning::ListUnspentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ListUnspentResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_UNSPENT, req, opt)
    }

    pub fn list_unspent(&self, req: &super::lightning::ListUnspentRequest) -> ::grpcio::Result<super::lightning::ListUnspentResponse> {
        self.list_unspent_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_unspent_async_opt(&self, req: &super::lightning::ListUnspentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListUnspentResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_UNSPENT, req, opt)
    }

    pub fn list_unspent_async(&self, req: &super::lightning::ListUnspentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListUnspentResponse>> {
        self.list_unspent_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_transactions_opt(&self, req: &super::lightning::GetTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::Transaction>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_TRANSACTIONS, req, opt)
    }

    pub fn subscribe_transactions(&self, req: &super::lightning::GetTransactionsRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::Transaction>> {
        self.subscribe_transactions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_many_opt(&self, req: &super::lightning::SendManyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::SendManyResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SEND_MANY, req, opt)
    }

    pub fn send_many(&self, req: &super::lightning::SendManyRequest) -> ::grpcio::Result<super::lightning::SendManyResponse> {
        self.send_many_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_many_async_opt(&self, req: &super::lightning::SendManyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SendManyResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SEND_MANY, req, opt)
    }

    pub fn send_many_async(&self, req: &super::lightning::SendManyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SendManyResponse>> {
        self.send_many_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn new_address_opt(&self, req: &super::lightning::NewAddressRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::NewAddressResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_NEW_ADDRESS, req, opt)
    }

    pub fn new_address(&self, req: &super::lightning::NewAddressRequest) -> ::grpcio::Result<super::lightning::NewAddressResponse> {
        self.new_address_opt(req, ::grpcio::CallOption::default())
    }

    pub fn new_address_async_opt(&self, req: &super::lightning::NewAddressRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::NewAddressResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_NEW_ADDRESS, req, opt)
    }

    pub fn new_address_async(&self, req: &super::lightning::NewAddressRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::NewAddressResponse>> {
        self.new_address_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_message_opt(&self, req: &super::lightning::SignMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::SignMessageResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SIGN_MESSAGE, req, opt)
    }

    pub fn sign_message(&self, req: &super::lightning::SignMessageRequest) -> ::grpcio::Result<super::lightning::SignMessageResponse> {
        self.sign_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_message_async_opt(&self, req: &super::lightning::SignMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SignMessageResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SIGN_MESSAGE, req, opt)
    }

    pub fn sign_message_async(&self, req: &super::lightning::SignMessageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SignMessageResponse>> {
        self.sign_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_message_opt(&self, req: &super::lightning::VerifyMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::VerifyMessageResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_VERIFY_MESSAGE, req, opt)
    }

    pub fn verify_message(&self, req: &super::lightning::VerifyMessageRequest) -> ::grpcio::Result<super::lightning::VerifyMessageResponse> {
        self.verify_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_message_async_opt(&self, req: &super::lightning::VerifyMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::VerifyMessageResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_VERIFY_MESSAGE, req, opt)
    }

    pub fn verify_message_async(&self, req: &super::lightning::VerifyMessageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::VerifyMessageResponse>> {
        self.verify_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn connect_peer_opt(&self, req: &super::lightning::ConnectPeerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ConnectPeerResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_CONNECT_PEER, req, opt)
    }

    pub fn connect_peer(&self, req: &super::lightning::ConnectPeerRequest) -> ::grpcio::Result<super::lightning::ConnectPeerResponse> {
        self.connect_peer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn connect_peer_async_opt(&self, req: &super::lightning::ConnectPeerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ConnectPeerResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_CONNECT_PEER, req, opt)
    }

    pub fn connect_peer_async(&self, req: &super::lightning::ConnectPeerRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ConnectPeerResponse>> {
        self.connect_peer_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn disconnect_peer_opt(&self, req: &super::lightning::DisconnectPeerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::DisconnectPeerResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_DISCONNECT_PEER, req, opt)
    }

    pub fn disconnect_peer(&self, req: &super::lightning::DisconnectPeerRequest) -> ::grpcio::Result<super::lightning::DisconnectPeerResponse> {
        self.disconnect_peer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn disconnect_peer_async_opt(&self, req: &super::lightning::DisconnectPeerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::DisconnectPeerResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DISCONNECT_PEER, req, opt)
    }

    pub fn disconnect_peer_async(&self, req: &super::lightning::DisconnectPeerRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::DisconnectPeerResponse>> {
        self.disconnect_peer_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_peers_opt(&self, req: &super::lightning::ListPeersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ListPeersResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_PEERS, req, opt)
    }

    pub fn list_peers(&self, req: &super::lightning::ListPeersRequest) -> ::grpcio::Result<super::lightning::ListPeersResponse> {
        self.list_peers_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_peers_async_opt(&self, req: &super::lightning::ListPeersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListPeersResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_PEERS, req, opt)
    }

    pub fn list_peers_async(&self, req: &super::lightning::ListPeersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListPeersResponse>> {
        self.list_peers_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_peer_events_opt(&self, req: &super::lightning::PeerEventSubscription, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::PeerEvent>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_PEER_EVENTS, req, opt)
    }

    pub fn subscribe_peer_events(&self, req: &super::lightning::PeerEventSubscription) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::PeerEvent>> {
        self.subscribe_peer_events_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_info_opt(&self, req: &super::lightning::GetInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::GetInfoResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_INFO, req, opt)
    }

    pub fn get_info(&self, req: &super::lightning::GetInfoRequest) -> ::grpcio::Result<super::lightning::GetInfoResponse> {
        self.get_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_info_async_opt(&self, req: &super::lightning::GetInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::GetInfoResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_INFO, req, opt)
    }

    pub fn get_info_async(&self, req: &super::lightning::GetInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::GetInfoResponse>> {
        self.get_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_recovery_info_opt(&self, req: &super::lightning::GetRecoveryInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::GetRecoveryInfoResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_RECOVERY_INFO, req, opt)
    }

    pub fn get_recovery_info(&self, req: &super::lightning::GetRecoveryInfoRequest) -> ::grpcio::Result<super::lightning::GetRecoveryInfoResponse> {
        self.get_recovery_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_recovery_info_async_opt(&self, req: &super::lightning::GetRecoveryInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::GetRecoveryInfoResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_RECOVERY_INFO, req, opt)
    }

    pub fn get_recovery_info_async(&self, req: &super::lightning::GetRecoveryInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::GetRecoveryInfoResponse>> {
        self.get_recovery_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn pending_channels_opt(&self, req: &super::lightning::PendingChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::PendingChannelsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_PENDING_CHANNELS, req, opt)
    }

    pub fn pending_channels(&self, req: &super::lightning::PendingChannelsRequest) -> ::grpcio::Result<super::lightning::PendingChannelsResponse> {
        self.pending_channels_opt(req, ::grpcio::CallOption::default())
    }

    pub fn pending_channels_async_opt(&self, req: &super::lightning::PendingChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::PendingChannelsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_PENDING_CHANNELS, req, opt)
    }

    pub fn pending_channels_async(&self, req: &super::lightning::PendingChannelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::PendingChannelsResponse>> {
        self.pending_channels_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_channels_opt(&self, req: &super::lightning::ListChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ListChannelsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_CHANNELS, req, opt)
    }

    pub fn list_channels(&self, req: &super::lightning::ListChannelsRequest) -> ::grpcio::Result<super::lightning::ListChannelsResponse> {
        self.list_channels_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_channels_async_opt(&self, req: &super::lightning::ListChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListChannelsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_CHANNELS, req, opt)
    }

    pub fn list_channels_async(&self, req: &super::lightning::ListChannelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListChannelsResponse>> {
        self.list_channels_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_channel_events_opt(&self, req: &super::lightning::ChannelEventSubscription, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::ChannelEventUpdate>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_EVENTS, req, opt)
    }

    pub fn subscribe_channel_events(&self, req: &super::lightning::ChannelEventSubscription) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::ChannelEventUpdate>> {
        self.subscribe_channel_events_opt(req, ::grpcio::CallOption::default())
    }

    pub fn closed_channels_opt(&self, req: &super::lightning::ClosedChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ClosedChannelsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_CLOSED_CHANNELS, req, opt)
    }

    pub fn closed_channels(&self, req: &super::lightning::ClosedChannelsRequest) -> ::grpcio::Result<super::lightning::ClosedChannelsResponse> {
        self.closed_channels_opt(req, ::grpcio::CallOption::default())
    }

    pub fn closed_channels_async_opt(&self, req: &super::lightning::ClosedChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ClosedChannelsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_CLOSED_CHANNELS, req, opt)
    }

    pub fn closed_channels_async(&self, req: &super::lightning::ClosedChannelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ClosedChannelsResponse>> {
        self.closed_channels_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn open_channel_sync_opt(&self, req: &super::lightning::OpenChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ChannelPoint> {
        self.client.unary_call(&METHOD_LIGHTNING_OPEN_CHANNEL_SYNC, req, opt)
    }

    pub fn open_channel_sync(&self, req: &super::lightning::OpenChannelRequest) -> ::grpcio::Result<super::lightning::ChannelPoint> {
        self.open_channel_sync_opt(req, ::grpcio::CallOption::default())
    }

    pub fn open_channel_sync_async_opt(&self, req: &super::lightning::OpenChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChannelPoint>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_OPEN_CHANNEL_SYNC, req, opt)
    }

    pub fn open_channel_sync_async(&self, req: &super::lightning::OpenChannelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChannelPoint>> {
        self.open_channel_sync_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn open_channel_opt(&self, req: &super::lightning::OpenChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::OpenStatusUpdate>> {
        self.client.server_streaming(&METHOD_LIGHTNING_OPEN_CHANNEL, req, opt)
    }

    pub fn open_channel(&self, req: &super::lightning::OpenChannelRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::OpenStatusUpdate>> {
        self.open_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_open_channel_opt(&self, req: &super::lightning::BatchOpenChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::BatchOpenChannelResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_BATCH_OPEN_CHANNEL, req, opt)
    }

    pub fn batch_open_channel(&self, req: &super::lightning::BatchOpenChannelRequest) -> ::grpcio::Result<super::lightning::BatchOpenChannelResponse> {
        self.batch_open_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_open_channel_async_opt(&self, req: &super::lightning::BatchOpenChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::BatchOpenChannelResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_BATCH_OPEN_CHANNEL, req, opt)
    }

    pub fn batch_open_channel_async(&self, req: &super::lightning::BatchOpenChannelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::BatchOpenChannelResponse>> {
        self.batch_open_channel_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn funding_state_step_opt(&self, req: &super::lightning::FundingTransitionMsg, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::FundingStateStepResp> {
        self.client.unary_call(&METHOD_LIGHTNING_FUNDING_STATE_STEP, req, opt)
    }

    pub fn funding_state_step(&self, req: &super::lightning::FundingTransitionMsg) -> ::grpcio::Result<super::lightning::FundingStateStepResp> {
        self.funding_state_step_opt(req, ::grpcio::CallOption::default())
    }

    pub fn funding_state_step_async_opt(&self, req: &super::lightning::FundingTransitionMsg, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::FundingStateStepResp>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_FUNDING_STATE_STEP, req, opt)
    }

    pub fn funding_state_step_async(&self, req: &super::lightning::FundingTransitionMsg) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::FundingStateStepResp>> {
        self.funding_state_step_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_acceptor_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::lightning::ChannelAcceptResponse>, ::grpcio::ClientDuplexReceiver<super::lightning::ChannelAcceptRequest>)> {
        self.client.duplex_streaming(&METHOD_LIGHTNING_CHANNEL_ACCEPTOR, opt)
    }

    pub fn channel_acceptor(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::lightning::ChannelAcceptResponse>, ::grpcio::ClientDuplexReceiver<super::lightning::ChannelAcceptRequest>)> {
        self.channel_acceptor_opt(::grpcio::CallOption::default())
    }

    pub fn close_channel_opt(&self, req: &super::lightning::CloseChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::CloseStatusUpdate>> {
        self.client.server_streaming(&METHOD_LIGHTNING_CLOSE_CHANNEL, req, opt)
    }

    pub fn close_channel(&self, req: &super::lightning::CloseChannelRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::CloseStatusUpdate>> {
        self.close_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn abandon_channel_opt(&self, req: &super::lightning::AbandonChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::AbandonChannelResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_ABANDON_CHANNEL, req, opt)
    }

    pub fn abandon_channel(&self, req: &super::lightning::AbandonChannelRequest) -> ::grpcio::Result<super::lightning::AbandonChannelResponse> {
        self.abandon_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn abandon_channel_async_opt(&self, req: &super::lightning::AbandonChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::AbandonChannelResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_ABANDON_CHANNEL, req, opt)
    }

    pub fn abandon_channel_async(&self, req: &super::lightning::AbandonChannelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::AbandonChannelResponse>> {
        self.abandon_channel_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_payment_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::lightning::SendRequest>, ::grpcio::ClientDuplexReceiver<super::lightning::SendResponse>)> {
        self.client.duplex_streaming(&METHOD_LIGHTNING_SEND_PAYMENT, opt)
    }

    pub fn send_payment(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::lightning::SendRequest>, ::grpcio::ClientDuplexReceiver<super::lightning::SendResponse>)> {
        self.send_payment_opt(::grpcio::CallOption::default())
    }

    pub fn send_payment_sync_opt(&self, req: &super::lightning::SendRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::SendResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SEND_PAYMENT_SYNC, req, opt)
    }

    pub fn send_payment_sync(&self, req: &super::lightning::SendRequest) -> ::grpcio::Result<super::lightning::SendResponse> {
        self.send_payment_sync_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_payment_sync_async_opt(&self, req: &super::lightning::SendRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SendResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SEND_PAYMENT_SYNC, req, opt)
    }

    pub fn send_payment_sync_async(&self, req: &super::lightning::SendRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SendResponse>> {
        self.send_payment_sync_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_to_route_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::lightning::SendToRouteRequest>, ::grpcio::ClientDuplexReceiver<super::lightning::SendResponse>)> {
        self.client.duplex_streaming(&METHOD_LIGHTNING_SEND_TO_ROUTE, opt)
    }

    pub fn send_to_route(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::lightning::SendToRouteRequest>, ::grpcio::ClientDuplexReceiver<super::lightning::SendResponse>)> {
        self.send_to_route_opt(::grpcio::CallOption::default())
    }

    pub fn send_to_route_sync_opt(&self, req: &super::lightning::SendToRouteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::SendResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SEND_TO_ROUTE_SYNC, req, opt)
    }

    pub fn send_to_route_sync(&self, req: &super::lightning::SendToRouteRequest) -> ::grpcio::Result<super::lightning::SendResponse> {
        self.send_to_route_sync_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_to_route_sync_async_opt(&self, req: &super::lightning::SendToRouteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SendResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SEND_TO_ROUTE_SYNC, req, opt)
    }

    pub fn send_to_route_sync_async(&self, req: &super::lightning::SendToRouteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SendResponse>> {
        self.send_to_route_sync_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_invoice_opt(&self, req: &super::lightning::Invoice, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::AddInvoiceResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_ADD_INVOICE, req, opt)
    }

    pub fn add_invoice(&self, req: &super::lightning::Invoice) -> ::grpcio::Result<super::lightning::AddInvoiceResponse> {
        self.add_invoice_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_invoice_async_opt(&self, req: &super::lightning::Invoice, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::AddInvoiceResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_ADD_INVOICE, req, opt)
    }

    pub fn add_invoice_async(&self, req: &super::lightning::Invoice) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::AddInvoiceResponse>> {
        self.add_invoice_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_invoices_opt(&self, req: &super::lightning::ListInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ListInvoiceResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_INVOICES, req, opt)
    }

    pub fn list_invoices(&self, req: &super::lightning::ListInvoiceRequest) -> ::grpcio::Result<super::lightning::ListInvoiceResponse> {
        self.list_invoices_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_invoices_async_opt(&self, req: &super::lightning::ListInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListInvoiceResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_INVOICES, req, opt)
    }

    pub fn list_invoices_async(&self, req: &super::lightning::ListInvoiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListInvoiceResponse>> {
        self.list_invoices_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lookup_invoice_opt(&self, req: &super::lightning::PaymentHash, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::Invoice> {
        self.client.unary_call(&METHOD_LIGHTNING_LOOKUP_INVOICE, req, opt)
    }

    pub fn lookup_invoice(&self, req: &super::lightning::PaymentHash) -> ::grpcio::Result<super::lightning::Invoice> {
        self.lookup_invoice_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lookup_invoice_async_opt(&self, req: &super::lightning::PaymentHash, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::Invoice>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LOOKUP_INVOICE, req, opt)
    }

    pub fn lookup_invoice_async(&self, req: &super::lightning::PaymentHash) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::Invoice>> {
        self.lookup_invoice_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_invoices_opt(&self, req: &super::lightning::InvoiceSubscription, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::Invoice>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_INVOICES, req, opt)
    }

    pub fn subscribe_invoices(&self, req: &super::lightning::InvoiceSubscription) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::Invoice>> {
        self.subscribe_invoices_opt(req, ::grpcio::CallOption::default())
    }

    pub fn decode_pay_req_opt(&self, req: &super::lightning::PayReqString, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::PayReq> {
        self.client.unary_call(&METHOD_LIGHTNING_DECODE_PAY_REQ, req, opt)
    }

    pub fn decode_pay_req(&self, req: &super::lightning::PayReqString) -> ::grpcio::Result<super::lightning::PayReq> {
        self.decode_pay_req_opt(req, ::grpcio::CallOption::default())
    }

    pub fn decode_pay_req_async_opt(&self, req: &super::lightning::PayReqString, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::PayReq>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DECODE_PAY_REQ, req, opt)
    }

    pub fn decode_pay_req_async(&self, req: &super::lightning::PayReqString) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::PayReq>> {
        self.decode_pay_req_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_payments_opt(&self, req: &super::lightning::ListPaymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ListPaymentsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_PAYMENTS, req, opt)
    }

    pub fn list_payments(&self, req: &super::lightning::ListPaymentsRequest) -> ::grpcio::Result<super::lightning::ListPaymentsResponse> {
        self.list_payments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_payments_async_opt(&self, req: &super::lightning::ListPaymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListPaymentsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_PAYMENTS, req, opt)
    }

    pub fn list_payments_async(&self, req: &super::lightning::ListPaymentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListPaymentsResponse>> {
        self.list_payments_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_payment_opt(&self, req: &super::lightning::DeletePaymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::DeletePaymentResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_DELETE_PAYMENT, req, opt)
    }

    pub fn delete_payment(&self, req: &super::lightning::DeletePaymentRequest) -> ::grpcio::Result<super::lightning::DeletePaymentResponse> {
        self.delete_payment_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_payment_async_opt(&self, req: &super::lightning::DeletePaymentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::DeletePaymentResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DELETE_PAYMENT, req, opt)
    }

    pub fn delete_payment_async(&self, req: &super::lightning::DeletePaymentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::DeletePaymentResponse>> {
        self.delete_payment_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_all_payments_opt(&self, req: &super::lightning::DeleteAllPaymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::DeleteAllPaymentsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_DELETE_ALL_PAYMENTS, req, opt)
    }

    pub fn delete_all_payments(&self, req: &super::lightning::DeleteAllPaymentsRequest) -> ::grpcio::Result<super::lightning::DeleteAllPaymentsResponse> {
        self.delete_all_payments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_all_payments_async_opt(&self, req: &super::lightning::DeleteAllPaymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::DeleteAllPaymentsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DELETE_ALL_PAYMENTS, req, opt)
    }

    pub fn delete_all_payments_async(&self, req: &super::lightning::DeleteAllPaymentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::DeleteAllPaymentsResponse>> {
        self.delete_all_payments_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn describe_graph_opt(&self, req: &super::lightning::ChannelGraphRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ChannelGraph> {
        self.client.unary_call(&METHOD_LIGHTNING_DESCRIBE_GRAPH, req, opt)
    }

    pub fn describe_graph(&self, req: &super::lightning::ChannelGraphRequest) -> ::grpcio::Result<super::lightning::ChannelGraph> {
        self.describe_graph_opt(req, ::grpcio::CallOption::default())
    }

    pub fn describe_graph_async_opt(&self, req: &super::lightning::ChannelGraphRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChannelGraph>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DESCRIBE_GRAPH, req, opt)
    }

    pub fn describe_graph_async(&self, req: &super::lightning::ChannelGraphRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChannelGraph>> {
        self.describe_graph_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_metrics_opt(&self, req: &super::lightning::NodeMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::NodeMetricsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_NODE_METRICS, req, opt)
    }

    pub fn get_node_metrics(&self, req: &super::lightning::NodeMetricsRequest) -> ::grpcio::Result<super::lightning::NodeMetricsResponse> {
        self.get_node_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_metrics_async_opt(&self, req: &super::lightning::NodeMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::NodeMetricsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_NODE_METRICS, req, opt)
    }

    pub fn get_node_metrics_async(&self, req: &super::lightning::NodeMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::NodeMetricsResponse>> {
        self.get_node_metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_chan_info_opt(&self, req: &super::lightning::ChanInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ChannelEdge> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_CHAN_INFO, req, opt)
    }

    pub fn get_chan_info(&self, req: &super::lightning::ChanInfoRequest) -> ::grpcio::Result<super::lightning::ChannelEdge> {
        self.get_chan_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_chan_info_async_opt(&self, req: &super::lightning::ChanInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChannelEdge>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_CHAN_INFO, req, opt)
    }

    pub fn get_chan_info_async(&self, req: &super::lightning::ChanInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChannelEdge>> {
        self.get_chan_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_info_opt(&self, req: &super::lightning::NodeInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::NodeInfo> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_NODE_INFO, req, opt)
    }

    pub fn get_node_info(&self, req: &super::lightning::NodeInfoRequest) -> ::grpcio::Result<super::lightning::NodeInfo> {
        self.get_node_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_info_async_opt(&self, req: &super::lightning::NodeInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::NodeInfo>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_NODE_INFO, req, opt)
    }

    pub fn get_node_info_async(&self, req: &super::lightning::NodeInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::NodeInfo>> {
        self.get_node_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_routes_opt(&self, req: &super::lightning::QueryRoutesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::QueryRoutesResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_QUERY_ROUTES, req, opt)
    }

    pub fn query_routes(&self, req: &super::lightning::QueryRoutesRequest) -> ::grpcio::Result<super::lightning::QueryRoutesResponse> {
        self.query_routes_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_routes_async_opt(&self, req: &super::lightning::QueryRoutesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::QueryRoutesResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_QUERY_ROUTES, req, opt)
    }

    pub fn query_routes_async(&self, req: &super::lightning::QueryRoutesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::QueryRoutesResponse>> {
        self.query_routes_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_network_info_opt(&self, req: &super::lightning::NetworkInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::NetworkInfo> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_NETWORK_INFO, req, opt)
    }

    pub fn get_network_info(&self, req: &super::lightning::NetworkInfoRequest) -> ::grpcio::Result<super::lightning::NetworkInfo> {
        self.get_network_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_network_info_async_opt(&self, req: &super::lightning::NetworkInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::NetworkInfo>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_NETWORK_INFO, req, opt)
    }

    pub fn get_network_info_async(&self, req: &super::lightning::NetworkInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::NetworkInfo>> {
        self.get_network_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_daemon_opt(&self, req: &super::lightning::StopRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::StopResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_STOP_DAEMON, req, opt)
    }

    pub fn stop_daemon(&self, req: &super::lightning::StopRequest) -> ::grpcio::Result<super::lightning::StopResponse> {
        self.stop_daemon_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_daemon_async_opt(&self, req: &super::lightning::StopRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::StopResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_STOP_DAEMON, req, opt)
    }

    pub fn stop_daemon_async(&self, req: &super::lightning::StopRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::StopResponse>> {
        self.stop_daemon_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_channel_graph_opt(&self, req: &super::lightning::GraphTopologySubscription, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::GraphTopologyUpdate>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_GRAPH, req, opt)
    }

    pub fn subscribe_channel_graph(&self, req: &super::lightning::GraphTopologySubscription) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::GraphTopologyUpdate>> {
        self.subscribe_channel_graph_opt(req, ::grpcio::CallOption::default())
    }

    pub fn debug_level_opt(&self, req: &super::lightning::DebugLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::DebugLevelResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_DEBUG_LEVEL, req, opt)
    }

    pub fn debug_level(&self, req: &super::lightning::DebugLevelRequest) -> ::grpcio::Result<super::lightning::DebugLevelResponse> {
        self.debug_level_opt(req, ::grpcio::CallOption::default())
    }

    pub fn debug_level_async_opt(&self, req: &super::lightning::DebugLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::DebugLevelResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DEBUG_LEVEL, req, opt)
    }

    pub fn debug_level_async(&self, req: &super::lightning::DebugLevelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::DebugLevelResponse>> {
        self.debug_level_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fee_report_opt(&self, req: &super::lightning::FeeReportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::FeeReportResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_FEE_REPORT, req, opt)
    }

    pub fn fee_report(&self, req: &super::lightning::FeeReportRequest) -> ::grpcio::Result<super::lightning::FeeReportResponse> {
        self.fee_report_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fee_report_async_opt(&self, req: &super::lightning::FeeReportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::FeeReportResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_FEE_REPORT, req, opt)
    }

    pub fn fee_report_async(&self, req: &super::lightning::FeeReportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::FeeReportResponse>> {
        self.fee_report_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_channel_policy_opt(&self, req: &super::lightning::PolicyUpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::PolicyUpdateResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_UPDATE_CHANNEL_POLICY, req, opt)
    }

    pub fn update_channel_policy(&self, req: &super::lightning::PolicyUpdateRequest) -> ::grpcio::Result<super::lightning::PolicyUpdateResponse> {
        self.update_channel_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_channel_policy_async_opt(&self, req: &super::lightning::PolicyUpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::PolicyUpdateResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_UPDATE_CHANNEL_POLICY, req, opt)
    }

    pub fn update_channel_policy_async(&self, req: &super::lightning::PolicyUpdateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::PolicyUpdateResponse>> {
        self.update_channel_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn forwarding_history_opt(&self, req: &super::lightning::ForwardingHistoryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ForwardingHistoryResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_FORWARDING_HISTORY, req, opt)
    }

    pub fn forwarding_history(&self, req: &super::lightning::ForwardingHistoryRequest) -> ::grpcio::Result<super::lightning::ForwardingHistoryResponse> {
        self.forwarding_history_opt(req, ::grpcio::CallOption::default())
    }

    pub fn forwarding_history_async_opt(&self, req: &super::lightning::ForwardingHistoryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ForwardingHistoryResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_FORWARDING_HISTORY, req, opt)
    }

    pub fn forwarding_history_async(&self, req: &super::lightning::ForwardingHistoryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ForwardingHistoryResponse>> {
        self.forwarding_history_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_channel_backup_opt(&self, req: &super::lightning::ExportChannelBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ChannelBackup> {
        self.client.unary_call(&METHOD_LIGHTNING_EXPORT_CHANNEL_BACKUP, req, opt)
    }

    pub fn export_channel_backup(&self, req: &super::lightning::ExportChannelBackupRequest) -> ::grpcio::Result<super::lightning::ChannelBackup> {
        self.export_channel_backup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_channel_backup_async_opt(&self, req: &super::lightning::ExportChannelBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChannelBackup>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_EXPORT_CHANNEL_BACKUP, req, opt)
    }

    pub fn export_channel_backup_async(&self, req: &super::lightning::ExportChannelBackupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChannelBackup>> {
        self.export_channel_backup_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_all_channel_backups_opt(&self, req: &super::lightning::ChanBackupExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ChanBackupSnapshot> {
        self.client.unary_call(&METHOD_LIGHTNING_EXPORT_ALL_CHANNEL_BACKUPS, req, opt)
    }

    pub fn export_all_channel_backups(&self, req: &super::lightning::ChanBackupExportRequest) -> ::grpcio::Result<super::lightning::ChanBackupSnapshot> {
        self.export_all_channel_backups_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_all_channel_backups_async_opt(&self, req: &super::lightning::ChanBackupExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChanBackupSnapshot>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_EXPORT_ALL_CHANNEL_BACKUPS, req, opt)
    }

    pub fn export_all_channel_backups_async(&self, req: &super::lightning::ChanBackupExportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ChanBackupSnapshot>> {
        self.export_all_channel_backups_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_chan_backup_opt(&self, req: &super::lightning::ChanBackupSnapshot, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::VerifyChanBackupResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_VERIFY_CHAN_BACKUP, req, opt)
    }

    pub fn verify_chan_backup(&self, req: &super::lightning::ChanBackupSnapshot) -> ::grpcio::Result<super::lightning::VerifyChanBackupResponse> {
        self.verify_chan_backup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_chan_backup_async_opt(&self, req: &super::lightning::ChanBackupSnapshot, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::VerifyChanBackupResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_VERIFY_CHAN_BACKUP, req, opt)
    }

    pub fn verify_chan_backup_async(&self, req: &super::lightning::ChanBackupSnapshot) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::VerifyChanBackupResponse>> {
        self.verify_chan_backup_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn restore_channel_backups_opt(&self, req: &super::lightning::RestoreChanBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::RestoreBackupResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_RESTORE_CHANNEL_BACKUPS, req, opt)
    }

    pub fn restore_channel_backups(&self, req: &super::lightning::RestoreChanBackupRequest) -> ::grpcio::Result<super::lightning::RestoreBackupResponse> {
        self.restore_channel_backups_opt(req, ::grpcio::CallOption::default())
    }

    pub fn restore_channel_backups_async_opt(&self, req: &super::lightning::RestoreChanBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::RestoreBackupResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_RESTORE_CHANNEL_BACKUPS, req, opt)
    }

    pub fn restore_channel_backups_async(&self, req: &super::lightning::RestoreChanBackupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::RestoreBackupResponse>> {
        self.restore_channel_backups_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_channel_backups_opt(&self, req: &super::lightning::ChannelBackupSubscription, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::ChanBackupSnapshot>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_BACKUPS, req, opt)
    }

    pub fn subscribe_channel_backups(&self, req: &super::lightning::ChannelBackupSubscription) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::ChanBackupSnapshot>> {
        self.subscribe_channel_backups_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bake_macaroon_opt(&self, req: &super::lightning::BakeMacaroonRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::BakeMacaroonResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_BAKE_MACAROON, req, opt)
    }

    pub fn bake_macaroon(&self, req: &super::lightning::BakeMacaroonRequest) -> ::grpcio::Result<super::lightning::BakeMacaroonResponse> {
        self.bake_macaroon_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bake_macaroon_async_opt(&self, req: &super::lightning::BakeMacaroonRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::BakeMacaroonResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_BAKE_MACAROON, req, opt)
    }

    pub fn bake_macaroon_async(&self, req: &super::lightning::BakeMacaroonRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::BakeMacaroonResponse>> {
        self.bake_macaroon_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_macaroon_i_ds_opt(&self, req: &super::lightning::ListMacaroonIDsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ListMacaroonIDsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_MACAROON_I_DS, req, opt)
    }

    pub fn list_macaroon_i_ds(&self, req: &super::lightning::ListMacaroonIDsRequest) -> ::grpcio::Result<super::lightning::ListMacaroonIDsResponse> {
        self.list_macaroon_i_ds_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_macaroon_i_ds_async_opt(&self, req: &super::lightning::ListMacaroonIDsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListMacaroonIDsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_MACAROON_I_DS, req, opt)
    }

    pub fn list_macaroon_i_ds_async(&self, req: &super::lightning::ListMacaroonIDsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListMacaroonIDsResponse>> {
        self.list_macaroon_i_ds_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_macaroon_id_opt(&self, req: &super::lightning::DeleteMacaroonIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::DeleteMacaroonIDResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_DELETE_MACAROON_ID, req, opt)
    }

    pub fn delete_macaroon_id(&self, req: &super::lightning::DeleteMacaroonIDRequest) -> ::grpcio::Result<super::lightning::DeleteMacaroonIDResponse> {
        self.delete_macaroon_id_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_macaroon_id_async_opt(&self, req: &super::lightning::DeleteMacaroonIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::DeleteMacaroonIDResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DELETE_MACAROON_ID, req, opt)
    }

    pub fn delete_macaroon_id_async(&self, req: &super::lightning::DeleteMacaroonIDRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::DeleteMacaroonIDResponse>> {
        self.delete_macaroon_id_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_permissions_opt(&self, req: &super::lightning::ListPermissionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::ListPermissionsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_PERMISSIONS, req, opt)
    }

    pub fn list_permissions(&self, req: &super::lightning::ListPermissionsRequest) -> ::grpcio::Result<super::lightning::ListPermissionsResponse> {
        self.list_permissions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_permissions_async_opt(&self, req: &super::lightning::ListPermissionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListPermissionsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_PERMISSIONS, req, opt)
    }

    pub fn list_permissions_async(&self, req: &super::lightning::ListPermissionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::ListPermissionsResponse>> {
        self.list_permissions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_macaroon_permissions_opt(&self, req: &super::lightning::CheckMacPermRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::CheckMacPermResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_CHECK_MACAROON_PERMISSIONS, req, opt)
    }

    pub fn check_macaroon_permissions(&self, req: &super::lightning::CheckMacPermRequest) -> ::grpcio::Result<super::lightning::CheckMacPermResponse> {
        self.check_macaroon_permissions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_macaroon_permissions_async_opt(&self, req: &super::lightning::CheckMacPermRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::CheckMacPermResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_CHECK_MACAROON_PERMISSIONS, req, opt)
    }

    pub fn check_macaroon_permissions_async(&self, req: &super::lightning::CheckMacPermRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::CheckMacPermResponse>> {
        self.check_macaroon_permissions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn register_rpc_middleware_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::lightning::RPCMiddlewareResponse>, ::grpcio::ClientDuplexReceiver<super::lightning::RPCMiddlewareRequest>)> {
        self.client.duplex_streaming(&METHOD_LIGHTNING_REGISTER_RPC_MIDDLEWARE, opt)
    }

    pub fn register_rpc_middleware(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::lightning::RPCMiddlewareResponse>, ::grpcio::ClientDuplexReceiver<super::lightning::RPCMiddlewareRequest>)> {
        self.register_rpc_middleware_opt(::grpcio::CallOption::default())
    }

    pub fn send_custom_message_opt(&self, req: &super::lightning::SendCustomMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::lightning::SendCustomMessageResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SEND_CUSTOM_MESSAGE, req, opt)
    }

    pub fn send_custom_message(&self, req: &super::lightning::SendCustomMessageRequest) -> ::grpcio::Result<super::lightning::SendCustomMessageResponse> {
        self.send_custom_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_custom_message_async_opt(&self, req: &super::lightning::SendCustomMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SendCustomMessageResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SEND_CUSTOM_MESSAGE, req, opt)
    }

    pub fn send_custom_message_async(&self, req: &super::lightning::SendCustomMessageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::lightning::SendCustomMessageResponse>> {
        self.send_custom_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_custom_messages_opt(&self, req: &super::lightning::SubscribeCustomMessagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::CustomMessage>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_CUSTOM_MESSAGES, req, opt)
    }

    pub fn subscribe_custom_messages(&self, req: &super::lightning::SubscribeCustomMessagesRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::lightning::CustomMessage>> {
        self.subscribe_custom_messages_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Lightning {
    fn wallet_balance(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::WalletBalanceRequest, sink: ::grpcio::UnarySink<super::lightning::WalletBalanceResponse>);
    fn channel_balance(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ChannelBalanceRequest, sink: ::grpcio::UnarySink<super::lightning::ChannelBalanceResponse>);
    fn get_transactions(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::GetTransactionsRequest, sink: ::grpcio::UnarySink<super::lightning::TransactionDetails>);
    fn estimate_fee(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::EstimateFeeRequest, sink: ::grpcio::UnarySink<super::lightning::EstimateFeeResponse>);
    fn send_coins(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::SendCoinsRequest, sink: ::grpcio::UnarySink<super::lightning::SendCoinsResponse>);
    fn list_unspent(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ListUnspentRequest, sink: ::grpcio::UnarySink<super::lightning::ListUnspentResponse>);
    fn subscribe_transactions(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::GetTransactionsRequest, sink: ::grpcio::ServerStreamingSink<super::lightning::Transaction>);
    fn send_many(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::SendManyRequest, sink: ::grpcio::UnarySink<super::lightning::SendManyResponse>);
    fn new_address(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::NewAddressRequest, sink: ::grpcio::UnarySink<super::lightning::NewAddressResponse>);
    fn sign_message(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::SignMessageRequest, sink: ::grpcio::UnarySink<super::lightning::SignMessageResponse>);
    fn verify_message(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::VerifyMessageRequest, sink: ::grpcio::UnarySink<super::lightning::VerifyMessageResponse>);
    fn connect_peer(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ConnectPeerRequest, sink: ::grpcio::UnarySink<super::lightning::ConnectPeerResponse>);
    fn disconnect_peer(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::DisconnectPeerRequest, sink: ::grpcio::UnarySink<super::lightning::DisconnectPeerResponse>);
    fn list_peers(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ListPeersRequest, sink: ::grpcio::UnarySink<super::lightning::ListPeersResponse>);
    fn subscribe_peer_events(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::PeerEventSubscription, sink: ::grpcio::ServerStreamingSink<super::lightning::PeerEvent>);
    fn get_info(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::GetInfoRequest, sink: ::grpcio::UnarySink<super::lightning::GetInfoResponse>);
    fn get_recovery_info(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::GetRecoveryInfoRequest, sink: ::grpcio::UnarySink<super::lightning::GetRecoveryInfoResponse>);
    fn pending_channels(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::PendingChannelsRequest, sink: ::grpcio::UnarySink<super::lightning::PendingChannelsResponse>);
    fn list_channels(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ListChannelsRequest, sink: ::grpcio::UnarySink<super::lightning::ListChannelsResponse>);
    fn subscribe_channel_events(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ChannelEventSubscription, sink: ::grpcio::ServerStreamingSink<super::lightning::ChannelEventUpdate>);
    fn closed_channels(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ClosedChannelsRequest, sink: ::grpcio::UnarySink<super::lightning::ClosedChannelsResponse>);
    fn open_channel_sync(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::OpenChannelRequest, sink: ::grpcio::UnarySink<super::lightning::ChannelPoint>);
    fn open_channel(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::OpenChannelRequest, sink: ::grpcio::ServerStreamingSink<super::lightning::OpenStatusUpdate>);
    fn batch_open_channel(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::BatchOpenChannelRequest, sink: ::grpcio::UnarySink<super::lightning::BatchOpenChannelResponse>);
    fn funding_state_step(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::FundingTransitionMsg, sink: ::grpcio::UnarySink<super::lightning::FundingStateStepResp>);
    fn channel_acceptor(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::lightning::ChannelAcceptResponse>, sink: ::grpcio::DuplexSink<super::lightning::ChannelAcceptRequest>);
    fn close_channel(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::CloseChannelRequest, sink: ::grpcio::ServerStreamingSink<super::lightning::CloseStatusUpdate>);
    fn abandon_channel(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::AbandonChannelRequest, sink: ::grpcio::UnarySink<super::lightning::AbandonChannelResponse>);
    fn send_payment(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::lightning::SendRequest>, sink: ::grpcio::DuplexSink<super::lightning::SendResponse>);
    fn send_payment_sync(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::SendRequest, sink: ::grpcio::UnarySink<super::lightning::SendResponse>);
    fn send_to_route(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::lightning::SendToRouteRequest>, sink: ::grpcio::DuplexSink<super::lightning::SendResponse>);
    fn send_to_route_sync(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::SendToRouteRequest, sink: ::grpcio::UnarySink<super::lightning::SendResponse>);
    fn add_invoice(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::Invoice, sink: ::grpcio::UnarySink<super::lightning::AddInvoiceResponse>);
    fn list_invoices(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ListInvoiceRequest, sink: ::grpcio::UnarySink<super::lightning::ListInvoiceResponse>);
    fn lookup_invoice(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::PaymentHash, sink: ::grpcio::UnarySink<super::lightning::Invoice>);
    fn subscribe_invoices(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::InvoiceSubscription, sink: ::grpcio::ServerStreamingSink<super::lightning::Invoice>);
    fn decode_pay_req(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::PayReqString, sink: ::grpcio::UnarySink<super::lightning::PayReq>);
    fn list_payments(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ListPaymentsRequest, sink: ::grpcio::UnarySink<super::lightning::ListPaymentsResponse>);
    fn delete_payment(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::DeletePaymentRequest, sink: ::grpcio::UnarySink<super::lightning::DeletePaymentResponse>);
    fn delete_all_payments(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::DeleteAllPaymentsRequest, sink: ::grpcio::UnarySink<super::lightning::DeleteAllPaymentsResponse>);
    fn describe_graph(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ChannelGraphRequest, sink: ::grpcio::UnarySink<super::lightning::ChannelGraph>);
    fn get_node_metrics(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::NodeMetricsRequest, sink: ::grpcio::UnarySink<super::lightning::NodeMetricsResponse>);
    fn get_chan_info(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ChanInfoRequest, sink: ::grpcio::UnarySink<super::lightning::ChannelEdge>);
    fn get_node_info(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::NodeInfoRequest, sink: ::grpcio::UnarySink<super::lightning::NodeInfo>);
    fn query_routes(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::QueryRoutesRequest, sink: ::grpcio::UnarySink<super::lightning::QueryRoutesResponse>);
    fn get_network_info(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::NetworkInfoRequest, sink: ::grpcio::UnarySink<super::lightning::NetworkInfo>);
    fn stop_daemon(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::StopRequest, sink: ::grpcio::UnarySink<super::lightning::StopResponse>);
    fn subscribe_channel_graph(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::GraphTopologySubscription, sink: ::grpcio::ServerStreamingSink<super::lightning::GraphTopologyUpdate>);
    fn debug_level(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::DebugLevelRequest, sink: ::grpcio::UnarySink<super::lightning::DebugLevelResponse>);
    fn fee_report(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::FeeReportRequest, sink: ::grpcio::UnarySink<super::lightning::FeeReportResponse>);
    fn update_channel_policy(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::PolicyUpdateRequest, sink: ::grpcio::UnarySink<super::lightning::PolicyUpdateResponse>);
    fn forwarding_history(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ForwardingHistoryRequest, sink: ::grpcio::UnarySink<super::lightning::ForwardingHistoryResponse>);
    fn export_channel_backup(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ExportChannelBackupRequest, sink: ::grpcio::UnarySink<super::lightning::ChannelBackup>);
    fn export_all_channel_backups(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ChanBackupExportRequest, sink: ::grpcio::UnarySink<super::lightning::ChanBackupSnapshot>);
    fn verify_chan_backup(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ChanBackupSnapshot, sink: ::grpcio::UnarySink<super::lightning::VerifyChanBackupResponse>);
    fn restore_channel_backups(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::RestoreChanBackupRequest, sink: ::grpcio::UnarySink<super::lightning::RestoreBackupResponse>);
    fn subscribe_channel_backups(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ChannelBackupSubscription, sink: ::grpcio::ServerStreamingSink<super::lightning::ChanBackupSnapshot>);
    fn bake_macaroon(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::BakeMacaroonRequest, sink: ::grpcio::UnarySink<super::lightning::BakeMacaroonResponse>);
    fn list_macaroon_i_ds(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ListMacaroonIDsRequest, sink: ::grpcio::UnarySink<super::lightning::ListMacaroonIDsResponse>);
    fn delete_macaroon_id(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::DeleteMacaroonIDRequest, sink: ::grpcio::UnarySink<super::lightning::DeleteMacaroonIDResponse>);
    fn list_permissions(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::ListPermissionsRequest, sink: ::grpcio::UnarySink<super::lightning::ListPermissionsResponse>);
    fn check_macaroon_permissions(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::CheckMacPermRequest, sink: ::grpcio::UnarySink<super::lightning::CheckMacPermResponse>);
    fn register_rpc_middleware(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::lightning::RPCMiddlewareResponse>, sink: ::grpcio::DuplexSink<super::lightning::RPCMiddlewareRequest>);
    fn send_custom_message(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::SendCustomMessageRequest, sink: ::grpcio::UnarySink<super::lightning::SendCustomMessageResponse>);
    fn subscribe_custom_messages(&mut self, ctx: ::grpcio::RpcContext, req: super::lightning::SubscribeCustomMessagesRequest, sink: ::grpcio::ServerStreamingSink<super::lightning::CustomMessage>);
}

pub fn create_lightning<S: Lightning + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_WALLET_BALANCE, move |ctx, req, resp| {
        instance.wallet_balance(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_CHANNEL_BALANCE, move |ctx, req, resp| {
        instance.channel_balance(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_GET_TRANSACTIONS, move |ctx, req, resp| {
        instance.get_transactions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_ESTIMATE_FEE, move |ctx, req, resp| {
        instance.estimate_fee(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_SEND_COINS, move |ctx, req, resp| {
        instance.send_coins(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_LIST_UNSPENT, move |ctx, req, resp| {
        instance.list_unspent(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_LIGHTNING_SUBSCRIBE_TRANSACTIONS, move |ctx, req, resp| {
        instance.subscribe_transactions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_SEND_MANY, move |ctx, req, resp| {
        instance.send_many(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_NEW_ADDRESS, move |ctx, req, resp| {
        instance.new_address(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_SIGN_MESSAGE, move |ctx, req, resp| {
        instance.sign_message(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_VERIFY_MESSAGE, move |ctx, req, resp| {
        instance.verify_message(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_CONNECT_PEER, move |ctx, req, resp| {
        instance.connect_peer(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_DISCONNECT_PEER, move |ctx, req, resp| {
        instance.disconnect_peer(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_LIST_PEERS, move |ctx, req, resp| {
        instance.list_peers(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_LIGHTNING_SUBSCRIBE_PEER_EVENTS, move |ctx, req, resp| {
        instance.subscribe_peer_events(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_GET_INFO, move |ctx, req, resp| {
        instance.get_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_GET_RECOVERY_INFO, move |ctx, req, resp| {
        instance.get_recovery_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_PENDING_CHANNELS, move |ctx, req, resp| {
        instance.pending_channels(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_LIST_CHANNELS, move |ctx, req, resp| {
        instance.list_channels(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_EVENTS, move |ctx, req, resp| {
        instance.subscribe_channel_events(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_CLOSED_CHANNELS, move |ctx, req, resp| {
        instance.closed_channels(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_OPEN_CHANNEL_SYNC, move |ctx, req, resp| {
        instance.open_channel_sync(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_LIGHTNING_OPEN_CHANNEL, move |ctx, req, resp| {
        instance.open_channel(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_BATCH_OPEN_CHANNEL, move |ctx, req, resp| {
        instance.batch_open_channel(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_FUNDING_STATE_STEP, move |ctx, req, resp| {
        instance.funding_state_step(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_LIGHTNING_CHANNEL_ACCEPTOR, move |ctx, req, resp| {
        instance.channel_acceptor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_LIGHTNING_CLOSE_CHANNEL, move |ctx, req, resp| {
        instance.close_channel(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_ABANDON_CHANNEL, move |ctx, req, resp| {
        instance.abandon_channel(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_LIGHTNING_SEND_PAYMENT, move |ctx, req, resp| {
        instance.send_payment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_SEND_PAYMENT_SYNC, move |ctx, req, resp| {
        instance.send_payment_sync(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_LIGHTNING_SEND_TO_ROUTE, move |ctx, req, resp| {
        instance.send_to_route(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_SEND_TO_ROUTE_SYNC, move |ctx, req, resp| {
        instance.send_to_route_sync(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_ADD_INVOICE, move |ctx, req, resp| {
        instance.add_invoice(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_LIST_INVOICES, move |ctx, req, resp| {
        instance.list_invoices(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_LOOKUP_INVOICE, move |ctx, req, resp| {
        instance.lookup_invoice(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_LIGHTNING_SUBSCRIBE_INVOICES, move |ctx, req, resp| {
        instance.subscribe_invoices(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_DECODE_PAY_REQ, move |ctx, req, resp| {
        instance.decode_pay_req(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_LIST_PAYMENTS, move |ctx, req, resp| {
        instance.list_payments(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_DELETE_PAYMENT, move |ctx, req, resp| {
        instance.delete_payment(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_DELETE_ALL_PAYMENTS, move |ctx, req, resp| {
        instance.delete_all_payments(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_DESCRIBE_GRAPH, move |ctx, req, resp| {
        instance.describe_graph(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_GET_NODE_METRICS, move |ctx, req, resp| {
        instance.get_node_metrics(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_GET_CHAN_INFO, move |ctx, req, resp| {
        instance.get_chan_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_GET_NODE_INFO, move |ctx, req, resp| {
        instance.get_node_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_QUERY_ROUTES, move |ctx, req, resp| {
        instance.query_routes(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_GET_NETWORK_INFO, move |ctx, req, resp| {
        instance.get_network_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_STOP_DAEMON, move |ctx, req, resp| {
        instance.stop_daemon(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_GRAPH, move |ctx, req, resp| {
        instance.subscribe_channel_graph(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_DEBUG_LEVEL, move |ctx, req, resp| {
        instance.debug_level(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_FEE_REPORT, move |ctx, req, resp| {
        instance.fee_report(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_UPDATE_CHANNEL_POLICY, move |ctx, req, resp| {
        instance.update_channel_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_FORWARDING_HISTORY, move |ctx, req, resp| {
        instance.forwarding_history(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_EXPORT_CHANNEL_BACKUP, move |ctx, req, resp| {
        instance.export_channel_backup(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_EXPORT_ALL_CHANNEL_BACKUPS, move |ctx, req, resp| {
        instance.export_all_channel_backups(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_VERIFY_CHAN_BACKUP, move |ctx, req, resp| {
        instance.verify_chan_backup(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_RESTORE_CHANNEL_BACKUPS, move |ctx, req, resp| {
        instance.restore_channel_backups(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_BACKUPS, move |ctx, req, resp| {
        instance.subscribe_channel_backups(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_BAKE_MACAROON, move |ctx, req, resp| {
        instance.bake_macaroon(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_LIST_MACAROON_I_DS, move |ctx, req, resp| {
        instance.list_macaroon_i_ds(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_DELETE_MACAROON_ID, move |ctx, req, resp| {
        instance.delete_macaroon_id(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_LIST_PERMISSIONS, move |ctx, req, resp| {
        instance.list_permissions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_CHECK_MACAROON_PERMISSIONS, move |ctx, req, resp| {
        instance.check_macaroon_permissions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_LIGHTNING_REGISTER_RPC_MIDDLEWARE, move |ctx, req, resp| {
        instance.register_rpc_middleware(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_SEND_CUSTOM_MESSAGE, move |ctx, req, resp| {
        instance.send_custom_message(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_LIGHTNING_SUBSCRIBE_CUSTOM_MESSAGES, move |ctx, req, resp| {
        instance.subscribe_custom_messages(ctx, req, resp)
    });
    builder.build()
}
