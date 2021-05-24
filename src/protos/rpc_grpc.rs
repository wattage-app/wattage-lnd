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

const METHOD_LIGHTNING_WALLET_BALANCE: ::grpcio::Method<super::rpc::WalletBalanceRequest, super::rpc::WalletBalanceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/WalletBalance",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CHANNEL_BALANCE: ::grpcio::Method<super::rpc::ChannelBalanceRequest, super::rpc::ChannelBalanceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ChannelBalance",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_TRANSACTIONS: ::grpcio::Method<super::rpc::GetTransactionsRequest, super::rpc::TransactionDetails> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetTransactions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_ESTIMATE_FEE: ::grpcio::Method<super::rpc::EstimateFeeRequest, super::rpc::EstimateFeeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/EstimateFee",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_COINS: ::grpcio::Method<super::rpc::SendCoinsRequest, super::rpc::SendCoinsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SendCoins",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_UNSPENT: ::grpcio::Method<super::rpc::ListUnspentRequest, super::rpc::ListUnspentResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListUnspent",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_TRANSACTIONS: ::grpcio::Method<super::rpc::GetTransactionsRequest, super::rpc::Transaction> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeTransactions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_MANY: ::grpcio::Method<super::rpc::SendManyRequest, super::rpc::SendManyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SendMany",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_NEW_ADDRESS: ::grpcio::Method<super::rpc::NewAddressRequest, super::rpc::NewAddressResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/NewAddress",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SIGN_MESSAGE: ::grpcio::Method<super::rpc::SignMessageRequest, super::rpc::SignMessageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SignMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_VERIFY_MESSAGE: ::grpcio::Method<super::rpc::VerifyMessageRequest, super::rpc::VerifyMessageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/VerifyMessage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CONNECT_PEER: ::grpcio::Method<super::rpc::ConnectPeerRequest, super::rpc::ConnectPeerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ConnectPeer",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DISCONNECT_PEER: ::grpcio::Method<super::rpc::DisconnectPeerRequest, super::rpc::DisconnectPeerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DisconnectPeer",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_PEERS: ::grpcio::Method<super::rpc::ListPeersRequest, super::rpc::ListPeersResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListPeers",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_PEER_EVENTS: ::grpcio::Method<super::rpc::PeerEventSubscription, super::rpc::PeerEvent> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribePeerEvents",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_INFO: ::grpcio::Method<super::rpc::GetInfoRequest, super::rpc::GetInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_RECOVERY_INFO: ::grpcio::Method<super::rpc::GetRecoveryInfoRequest, super::rpc::GetRecoveryInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetRecoveryInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_PENDING_CHANNELS: ::grpcio::Method<super::rpc::PendingChannelsRequest, super::rpc::PendingChannelsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/PendingChannels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_CHANNELS: ::grpcio::Method<super::rpc::ListChannelsRequest, super::rpc::ListChannelsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListChannels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_EVENTS: ::grpcio::Method<super::rpc::ChannelEventSubscription, super::rpc::ChannelEventUpdate> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeChannelEvents",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CLOSED_CHANNELS: ::grpcio::Method<super::rpc::ClosedChannelsRequest, super::rpc::ClosedChannelsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ClosedChannels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_OPEN_CHANNEL_SYNC: ::grpcio::Method<super::rpc::OpenChannelRequest, super::rpc::ChannelPoint> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/OpenChannelSync",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_OPEN_CHANNEL: ::grpcio::Method<super::rpc::OpenChannelRequest, super::rpc::OpenStatusUpdate> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/OpenChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_FUNDING_STATE_STEP: ::grpcio::Method<super::rpc::FundingTransitionMsg, super::rpc::FundingStateStepResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/FundingStateStep",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CHANNEL_ACCEPTOR: ::grpcio::Method<super::rpc::ChannelAcceptResponse, super::rpc::ChannelAcceptRequest> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/lnrpc.Lightning/ChannelAcceptor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_CLOSE_CHANNEL: ::grpcio::Method<super::rpc::CloseChannelRequest, super::rpc::CloseStatusUpdate> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/CloseChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_ABANDON_CHANNEL: ::grpcio::Method<super::rpc::AbandonChannelRequest, super::rpc::AbandonChannelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/AbandonChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_PAYMENT: ::grpcio::Method<super::rpc::SendRequest, super::rpc::SendResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/lnrpc.Lightning/SendPayment",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_PAYMENT_SYNC: ::grpcio::Method<super::rpc::SendRequest, super::rpc::SendResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SendPaymentSync",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_TO_ROUTE: ::grpcio::Method<super::rpc::SendToRouteRequest, super::rpc::SendResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/lnrpc.Lightning/SendToRoute",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SEND_TO_ROUTE_SYNC: ::grpcio::Method<super::rpc::SendToRouteRequest, super::rpc::SendResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/SendToRouteSync",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_ADD_INVOICE: ::grpcio::Method<super::rpc::Invoice, super::rpc::AddInvoiceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/AddInvoice",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_INVOICES: ::grpcio::Method<super::rpc::ListInvoiceRequest, super::rpc::ListInvoiceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListInvoices",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LOOKUP_INVOICE: ::grpcio::Method<super::rpc::PaymentHash, super::rpc::Invoice> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/LookupInvoice",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_INVOICES: ::grpcio::Method<super::rpc::InvoiceSubscription, super::rpc::Invoice> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeInvoices",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DECODE_PAY_REQ: ::grpcio::Method<super::rpc::PayReqString, super::rpc::PayReq> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DecodePayReq",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_PAYMENTS: ::grpcio::Method<super::rpc::ListPaymentsRequest, super::rpc::ListPaymentsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListPayments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DELETE_ALL_PAYMENTS: ::grpcio::Method<super::rpc::DeleteAllPaymentsRequest, super::rpc::DeleteAllPaymentsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DeleteAllPayments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DESCRIBE_GRAPH: ::grpcio::Method<super::rpc::ChannelGraphRequest, super::rpc::ChannelGraph> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DescribeGraph",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_NODE_METRICS: ::grpcio::Method<super::rpc::NodeMetricsRequest, super::rpc::NodeMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetNodeMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_CHAN_INFO: ::grpcio::Method<super::rpc::ChanInfoRequest, super::rpc::ChannelEdge> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetChanInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_NODE_INFO: ::grpcio::Method<super::rpc::NodeInfoRequest, super::rpc::NodeInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetNodeInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_QUERY_ROUTES: ::grpcio::Method<super::rpc::QueryRoutesRequest, super::rpc::QueryRoutesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/QueryRoutes",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_GET_NETWORK_INFO: ::grpcio::Method<super::rpc::NetworkInfoRequest, super::rpc::NetworkInfo> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/GetNetworkInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_STOP_DAEMON: ::grpcio::Method<super::rpc::StopRequest, super::rpc::StopResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/StopDaemon",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_GRAPH: ::grpcio::Method<super::rpc::GraphTopologySubscription, super::rpc::GraphTopologyUpdate> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeChannelGraph",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DEBUG_LEVEL: ::grpcio::Method<super::rpc::DebugLevelRequest, super::rpc::DebugLevelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DebugLevel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_FEE_REPORT: ::grpcio::Method<super::rpc::FeeReportRequest, super::rpc::FeeReportResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/FeeReport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_UPDATE_CHANNEL_POLICY: ::grpcio::Method<super::rpc::PolicyUpdateRequest, super::rpc::PolicyUpdateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/UpdateChannelPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_FORWARDING_HISTORY: ::grpcio::Method<super::rpc::ForwardingHistoryRequest, super::rpc::ForwardingHistoryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ForwardingHistory",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_EXPORT_CHANNEL_BACKUP: ::grpcio::Method<super::rpc::ExportChannelBackupRequest, super::rpc::ChannelBackup> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ExportChannelBackup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_EXPORT_ALL_CHANNEL_BACKUPS: ::grpcio::Method<super::rpc::ChanBackupExportRequest, super::rpc::ChanBackupSnapshot> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ExportAllChannelBackups",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_VERIFY_CHAN_BACKUP: ::grpcio::Method<super::rpc::ChanBackupSnapshot, super::rpc::VerifyChanBackupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/VerifyChanBackup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_RESTORE_CHANNEL_BACKUPS: ::grpcio::Method<super::rpc::RestoreChanBackupRequest, super::rpc::RestoreBackupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/RestoreChannelBackups",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_BACKUPS: ::grpcio::Method<super::rpc::ChannelBackupSubscription, super::rpc::ChanBackupSnapshot> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/lnrpc.Lightning/SubscribeChannelBackups",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_BAKE_MACAROON: ::grpcio::Method<super::rpc::BakeMacaroonRequest, super::rpc::BakeMacaroonResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/BakeMacaroon",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_MACAROON_I_DS: ::grpcio::Method<super::rpc::ListMacaroonIDsRequest, super::rpc::ListMacaroonIDsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListMacaroonIDs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_DELETE_MACAROON_ID: ::grpcio::Method<super::rpc::DeleteMacaroonIDRequest, super::rpc::DeleteMacaroonIDResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/DeleteMacaroonID",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LIGHTNING_LIST_PERMISSIONS: ::grpcio::Method<super::rpc::ListPermissionsRequest, super::rpc::ListPermissionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/lnrpc.Lightning/ListPermissions",
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

    pub fn wallet_balance_opt(&self, req: &super::rpc::WalletBalanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::WalletBalanceResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_WALLET_BALANCE, req, opt)
    }

    pub fn wallet_balance(&self, req: &super::rpc::WalletBalanceRequest) -> ::grpcio::Result<super::rpc::WalletBalanceResponse> {
        self.wallet_balance_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wallet_balance_async_opt(&self, req: &super::rpc::WalletBalanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::WalletBalanceResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_WALLET_BALANCE, req, opt)
    }

    pub fn wallet_balance_async(&self, req: &super::rpc::WalletBalanceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::WalletBalanceResponse>> {
        self.wallet_balance_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_balance_opt(&self, req: &super::rpc::ChannelBalanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ChannelBalanceResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_CHANNEL_BALANCE, req, opt)
    }

    pub fn channel_balance(&self, req: &super::rpc::ChannelBalanceRequest) -> ::grpcio::Result<super::rpc::ChannelBalanceResponse> {
        self.channel_balance_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_balance_async_opt(&self, req: &super::rpc::ChannelBalanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChannelBalanceResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_CHANNEL_BALANCE, req, opt)
    }

    pub fn channel_balance_async(&self, req: &super::rpc::ChannelBalanceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChannelBalanceResponse>> {
        self.channel_balance_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_transactions_opt(&self, req: &super::rpc::GetTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::TransactionDetails> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_TRANSACTIONS, req, opt)
    }

    pub fn get_transactions(&self, req: &super::rpc::GetTransactionsRequest) -> ::grpcio::Result<super::rpc::TransactionDetails> {
        self.get_transactions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_transactions_async_opt(&self, req: &super::rpc::GetTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::TransactionDetails>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_TRANSACTIONS, req, opt)
    }

    pub fn get_transactions_async(&self, req: &super::rpc::GetTransactionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::TransactionDetails>> {
        self.get_transactions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn estimate_fee_opt(&self, req: &super::rpc::EstimateFeeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::EstimateFeeResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_ESTIMATE_FEE, req, opt)
    }

    pub fn estimate_fee(&self, req: &super::rpc::EstimateFeeRequest) -> ::grpcio::Result<super::rpc::EstimateFeeResponse> {
        self.estimate_fee_opt(req, ::grpcio::CallOption::default())
    }

    pub fn estimate_fee_async_opt(&self, req: &super::rpc::EstimateFeeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::EstimateFeeResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_ESTIMATE_FEE, req, opt)
    }

    pub fn estimate_fee_async(&self, req: &super::rpc::EstimateFeeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::EstimateFeeResponse>> {
        self.estimate_fee_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_coins_opt(&self, req: &super::rpc::SendCoinsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::SendCoinsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SEND_COINS, req, opt)
    }

    pub fn send_coins(&self, req: &super::rpc::SendCoinsRequest) -> ::grpcio::Result<super::rpc::SendCoinsResponse> {
        self.send_coins_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_coins_async_opt(&self, req: &super::rpc::SendCoinsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::SendCoinsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SEND_COINS, req, opt)
    }

    pub fn send_coins_async(&self, req: &super::rpc::SendCoinsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::SendCoinsResponse>> {
        self.send_coins_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_unspent_opt(&self, req: &super::rpc::ListUnspentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ListUnspentResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_UNSPENT, req, opt)
    }

    pub fn list_unspent(&self, req: &super::rpc::ListUnspentRequest) -> ::grpcio::Result<super::rpc::ListUnspentResponse> {
        self.list_unspent_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_unspent_async_opt(&self, req: &super::rpc::ListUnspentRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListUnspentResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_UNSPENT, req, opt)
    }

    pub fn list_unspent_async(&self, req: &super::rpc::ListUnspentRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListUnspentResponse>> {
        self.list_unspent_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_transactions_opt(&self, req: &super::rpc::GetTransactionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::Transaction>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_TRANSACTIONS, req, opt)
    }

    pub fn subscribe_transactions(&self, req: &super::rpc::GetTransactionsRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::Transaction>> {
        self.subscribe_transactions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_many_opt(&self, req: &super::rpc::SendManyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::SendManyResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SEND_MANY, req, opt)
    }

    pub fn send_many(&self, req: &super::rpc::SendManyRequest) -> ::grpcio::Result<super::rpc::SendManyResponse> {
        self.send_many_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_many_async_opt(&self, req: &super::rpc::SendManyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::SendManyResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SEND_MANY, req, opt)
    }

    pub fn send_many_async(&self, req: &super::rpc::SendManyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::SendManyResponse>> {
        self.send_many_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn new_address_opt(&self, req: &super::rpc::NewAddressRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::NewAddressResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_NEW_ADDRESS, req, opt)
    }

    pub fn new_address(&self, req: &super::rpc::NewAddressRequest) -> ::grpcio::Result<super::rpc::NewAddressResponse> {
        self.new_address_opt(req, ::grpcio::CallOption::default())
    }

    pub fn new_address_async_opt(&self, req: &super::rpc::NewAddressRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::NewAddressResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_NEW_ADDRESS, req, opt)
    }

    pub fn new_address_async(&self, req: &super::rpc::NewAddressRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::NewAddressResponse>> {
        self.new_address_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_message_opt(&self, req: &super::rpc::SignMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::SignMessageResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SIGN_MESSAGE, req, opt)
    }

    pub fn sign_message(&self, req: &super::rpc::SignMessageRequest) -> ::grpcio::Result<super::rpc::SignMessageResponse> {
        self.sign_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn sign_message_async_opt(&self, req: &super::rpc::SignMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::SignMessageResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SIGN_MESSAGE, req, opt)
    }

    pub fn sign_message_async(&self, req: &super::rpc::SignMessageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::SignMessageResponse>> {
        self.sign_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_message_opt(&self, req: &super::rpc::VerifyMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::VerifyMessageResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_VERIFY_MESSAGE, req, opt)
    }

    pub fn verify_message(&self, req: &super::rpc::VerifyMessageRequest) -> ::grpcio::Result<super::rpc::VerifyMessageResponse> {
        self.verify_message_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_message_async_opt(&self, req: &super::rpc::VerifyMessageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::VerifyMessageResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_VERIFY_MESSAGE, req, opt)
    }

    pub fn verify_message_async(&self, req: &super::rpc::VerifyMessageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::VerifyMessageResponse>> {
        self.verify_message_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn connect_peer_opt(&self, req: &super::rpc::ConnectPeerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ConnectPeerResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_CONNECT_PEER, req, opt)
    }

    pub fn connect_peer(&self, req: &super::rpc::ConnectPeerRequest) -> ::grpcio::Result<super::rpc::ConnectPeerResponse> {
        self.connect_peer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn connect_peer_async_opt(&self, req: &super::rpc::ConnectPeerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ConnectPeerResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_CONNECT_PEER, req, opt)
    }

    pub fn connect_peer_async(&self, req: &super::rpc::ConnectPeerRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ConnectPeerResponse>> {
        self.connect_peer_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn disconnect_peer_opt(&self, req: &super::rpc::DisconnectPeerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::DisconnectPeerResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_DISCONNECT_PEER, req, opt)
    }

    pub fn disconnect_peer(&self, req: &super::rpc::DisconnectPeerRequest) -> ::grpcio::Result<super::rpc::DisconnectPeerResponse> {
        self.disconnect_peer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn disconnect_peer_async_opt(&self, req: &super::rpc::DisconnectPeerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DisconnectPeerResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DISCONNECT_PEER, req, opt)
    }

    pub fn disconnect_peer_async(&self, req: &super::rpc::DisconnectPeerRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DisconnectPeerResponse>> {
        self.disconnect_peer_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_peers_opt(&self, req: &super::rpc::ListPeersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ListPeersResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_PEERS, req, opt)
    }

    pub fn list_peers(&self, req: &super::rpc::ListPeersRequest) -> ::grpcio::Result<super::rpc::ListPeersResponse> {
        self.list_peers_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_peers_async_opt(&self, req: &super::rpc::ListPeersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListPeersResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_PEERS, req, opt)
    }

    pub fn list_peers_async(&self, req: &super::rpc::ListPeersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListPeersResponse>> {
        self.list_peers_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_peer_events_opt(&self, req: &super::rpc::PeerEventSubscription, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::PeerEvent>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_PEER_EVENTS, req, opt)
    }

    pub fn subscribe_peer_events(&self, req: &super::rpc::PeerEventSubscription) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::PeerEvent>> {
        self.subscribe_peer_events_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_info_opt(&self, req: &super::rpc::GetInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::GetInfoResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_INFO, req, opt)
    }

    pub fn get_info(&self, req: &super::rpc::GetInfoRequest) -> ::grpcio::Result<super::rpc::GetInfoResponse> {
        self.get_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_info_async_opt(&self, req: &super::rpc::GetInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::GetInfoResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_INFO, req, opt)
    }

    pub fn get_info_async(&self, req: &super::rpc::GetInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::GetInfoResponse>> {
        self.get_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_recovery_info_opt(&self, req: &super::rpc::GetRecoveryInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::GetRecoveryInfoResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_RECOVERY_INFO, req, opt)
    }

    pub fn get_recovery_info(&self, req: &super::rpc::GetRecoveryInfoRequest) -> ::grpcio::Result<super::rpc::GetRecoveryInfoResponse> {
        self.get_recovery_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_recovery_info_async_opt(&self, req: &super::rpc::GetRecoveryInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::GetRecoveryInfoResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_RECOVERY_INFO, req, opt)
    }

    pub fn get_recovery_info_async(&self, req: &super::rpc::GetRecoveryInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::GetRecoveryInfoResponse>> {
        self.get_recovery_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn pending_channels_opt(&self, req: &super::rpc::PendingChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::PendingChannelsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_PENDING_CHANNELS, req, opt)
    }

    pub fn pending_channels(&self, req: &super::rpc::PendingChannelsRequest) -> ::grpcio::Result<super::rpc::PendingChannelsResponse> {
        self.pending_channels_opt(req, ::grpcio::CallOption::default())
    }

    pub fn pending_channels_async_opt(&self, req: &super::rpc::PendingChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::PendingChannelsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_PENDING_CHANNELS, req, opt)
    }

    pub fn pending_channels_async(&self, req: &super::rpc::PendingChannelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::PendingChannelsResponse>> {
        self.pending_channels_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_channels_opt(&self, req: &super::rpc::ListChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ListChannelsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_CHANNELS, req, opt)
    }

    pub fn list_channels(&self, req: &super::rpc::ListChannelsRequest) -> ::grpcio::Result<super::rpc::ListChannelsResponse> {
        self.list_channels_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_channels_async_opt(&self, req: &super::rpc::ListChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListChannelsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_CHANNELS, req, opt)
    }

    pub fn list_channels_async(&self, req: &super::rpc::ListChannelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListChannelsResponse>> {
        self.list_channels_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_channel_events_opt(&self, req: &super::rpc::ChannelEventSubscription, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::ChannelEventUpdate>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_EVENTS, req, opt)
    }

    pub fn subscribe_channel_events(&self, req: &super::rpc::ChannelEventSubscription) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::ChannelEventUpdate>> {
        self.subscribe_channel_events_opt(req, ::grpcio::CallOption::default())
    }

    pub fn closed_channels_opt(&self, req: &super::rpc::ClosedChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ClosedChannelsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_CLOSED_CHANNELS, req, opt)
    }

    pub fn closed_channels(&self, req: &super::rpc::ClosedChannelsRequest) -> ::grpcio::Result<super::rpc::ClosedChannelsResponse> {
        self.closed_channels_opt(req, ::grpcio::CallOption::default())
    }

    pub fn closed_channels_async_opt(&self, req: &super::rpc::ClosedChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ClosedChannelsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_CLOSED_CHANNELS, req, opt)
    }

    pub fn closed_channels_async(&self, req: &super::rpc::ClosedChannelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ClosedChannelsResponse>> {
        self.closed_channels_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn open_channel_sync_opt(&self, req: &super::rpc::OpenChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ChannelPoint> {
        self.client.unary_call(&METHOD_LIGHTNING_OPEN_CHANNEL_SYNC, req, opt)
    }

    pub fn open_channel_sync(&self, req: &super::rpc::OpenChannelRequest) -> ::grpcio::Result<super::rpc::ChannelPoint> {
        self.open_channel_sync_opt(req, ::grpcio::CallOption::default())
    }

    pub fn open_channel_sync_async_opt(&self, req: &super::rpc::OpenChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChannelPoint>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_OPEN_CHANNEL_SYNC, req, opt)
    }

    pub fn open_channel_sync_async(&self, req: &super::rpc::OpenChannelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChannelPoint>> {
        self.open_channel_sync_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn open_channel_opt(&self, req: &super::rpc::OpenChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::OpenStatusUpdate>> {
        self.client.server_streaming(&METHOD_LIGHTNING_OPEN_CHANNEL, req, opt)
    }

    pub fn open_channel(&self, req: &super::rpc::OpenChannelRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::OpenStatusUpdate>> {
        self.open_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn funding_state_step_opt(&self, req: &super::rpc::FundingTransitionMsg, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::FundingStateStepResp> {
        self.client.unary_call(&METHOD_LIGHTNING_FUNDING_STATE_STEP, req, opt)
    }

    pub fn funding_state_step(&self, req: &super::rpc::FundingTransitionMsg) -> ::grpcio::Result<super::rpc::FundingStateStepResp> {
        self.funding_state_step_opt(req, ::grpcio::CallOption::default())
    }

    pub fn funding_state_step_async_opt(&self, req: &super::rpc::FundingTransitionMsg, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::FundingStateStepResp>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_FUNDING_STATE_STEP, req, opt)
    }

    pub fn funding_state_step_async(&self, req: &super::rpc::FundingTransitionMsg) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::FundingStateStepResp>> {
        self.funding_state_step_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_acceptor_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::rpc::ChannelAcceptResponse>, ::grpcio::ClientDuplexReceiver<super::rpc::ChannelAcceptRequest>)> {
        self.client.duplex_streaming(&METHOD_LIGHTNING_CHANNEL_ACCEPTOR, opt)
    }

    pub fn channel_acceptor(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::rpc::ChannelAcceptResponse>, ::grpcio::ClientDuplexReceiver<super::rpc::ChannelAcceptRequest>)> {
        self.channel_acceptor_opt(::grpcio::CallOption::default())
    }

    pub fn close_channel_opt(&self, req: &super::rpc::CloseChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::CloseStatusUpdate>> {
        self.client.server_streaming(&METHOD_LIGHTNING_CLOSE_CHANNEL, req, opt)
    }

    pub fn close_channel(&self, req: &super::rpc::CloseChannelRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::CloseStatusUpdate>> {
        self.close_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn abandon_channel_opt(&self, req: &super::rpc::AbandonChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AbandonChannelResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_ABANDON_CHANNEL, req, opt)
    }

    pub fn abandon_channel(&self, req: &super::rpc::AbandonChannelRequest) -> ::grpcio::Result<super::rpc::AbandonChannelResponse> {
        self.abandon_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn abandon_channel_async_opt(&self, req: &super::rpc::AbandonChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AbandonChannelResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_ABANDON_CHANNEL, req, opt)
    }

    pub fn abandon_channel_async(&self, req: &super::rpc::AbandonChannelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AbandonChannelResponse>> {
        self.abandon_channel_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_payment_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::rpc::SendRequest>, ::grpcio::ClientDuplexReceiver<super::rpc::SendResponse>)> {
        self.client.duplex_streaming(&METHOD_LIGHTNING_SEND_PAYMENT, opt)
    }

    pub fn send_payment(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::rpc::SendRequest>, ::grpcio::ClientDuplexReceiver<super::rpc::SendResponse>)> {
        self.send_payment_opt(::grpcio::CallOption::default())
    }

    pub fn send_payment_sync_opt(&self, req: &super::rpc::SendRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::SendResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SEND_PAYMENT_SYNC, req, opt)
    }

    pub fn send_payment_sync(&self, req: &super::rpc::SendRequest) -> ::grpcio::Result<super::rpc::SendResponse> {
        self.send_payment_sync_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_payment_sync_async_opt(&self, req: &super::rpc::SendRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::SendResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SEND_PAYMENT_SYNC, req, opt)
    }

    pub fn send_payment_sync_async(&self, req: &super::rpc::SendRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::SendResponse>> {
        self.send_payment_sync_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_to_route_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::rpc::SendToRouteRequest>, ::grpcio::ClientDuplexReceiver<super::rpc::SendResponse>)> {
        self.client.duplex_streaming(&METHOD_LIGHTNING_SEND_TO_ROUTE, opt)
    }

    pub fn send_to_route(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::rpc::SendToRouteRequest>, ::grpcio::ClientDuplexReceiver<super::rpc::SendResponse>)> {
        self.send_to_route_opt(::grpcio::CallOption::default())
    }

    pub fn send_to_route_sync_opt(&self, req: &super::rpc::SendToRouteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::SendResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_SEND_TO_ROUTE_SYNC, req, opt)
    }

    pub fn send_to_route_sync(&self, req: &super::rpc::SendToRouteRequest) -> ::grpcio::Result<super::rpc::SendResponse> {
        self.send_to_route_sync_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_to_route_sync_async_opt(&self, req: &super::rpc::SendToRouteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::SendResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_SEND_TO_ROUTE_SYNC, req, opt)
    }

    pub fn send_to_route_sync_async(&self, req: &super::rpc::SendToRouteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::SendResponse>> {
        self.send_to_route_sync_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_invoice_opt(&self, req: &super::rpc::Invoice, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::AddInvoiceResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_ADD_INVOICE, req, opt)
    }

    pub fn add_invoice(&self, req: &super::rpc::Invoice) -> ::grpcio::Result<super::rpc::AddInvoiceResponse> {
        self.add_invoice_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_invoice_async_opt(&self, req: &super::rpc::Invoice, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AddInvoiceResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_ADD_INVOICE, req, opt)
    }

    pub fn add_invoice_async(&self, req: &super::rpc::Invoice) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::AddInvoiceResponse>> {
        self.add_invoice_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_invoices_opt(&self, req: &super::rpc::ListInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ListInvoiceResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_INVOICES, req, opt)
    }

    pub fn list_invoices(&self, req: &super::rpc::ListInvoiceRequest) -> ::grpcio::Result<super::rpc::ListInvoiceResponse> {
        self.list_invoices_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_invoices_async_opt(&self, req: &super::rpc::ListInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListInvoiceResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_INVOICES, req, opt)
    }

    pub fn list_invoices_async(&self, req: &super::rpc::ListInvoiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListInvoiceResponse>> {
        self.list_invoices_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lookup_invoice_opt(&self, req: &super::rpc::PaymentHash, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::Invoice> {
        self.client.unary_call(&METHOD_LIGHTNING_LOOKUP_INVOICE, req, opt)
    }

    pub fn lookup_invoice(&self, req: &super::rpc::PaymentHash) -> ::grpcio::Result<super::rpc::Invoice> {
        self.lookup_invoice_opt(req, ::grpcio::CallOption::default())
    }

    pub fn lookup_invoice_async_opt(&self, req: &super::rpc::PaymentHash, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::Invoice>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LOOKUP_INVOICE, req, opt)
    }

    pub fn lookup_invoice_async(&self, req: &super::rpc::PaymentHash) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::Invoice>> {
        self.lookup_invoice_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_invoices_opt(&self, req: &super::rpc::InvoiceSubscription, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::Invoice>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_INVOICES, req, opt)
    }

    pub fn subscribe_invoices(&self, req: &super::rpc::InvoiceSubscription) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::Invoice>> {
        self.subscribe_invoices_opt(req, ::grpcio::CallOption::default())
    }

    pub fn decode_pay_req_opt(&self, req: &super::rpc::PayReqString, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::PayReq> {
        self.client.unary_call(&METHOD_LIGHTNING_DECODE_PAY_REQ, req, opt)
    }

    pub fn decode_pay_req(&self, req: &super::rpc::PayReqString) -> ::grpcio::Result<super::rpc::PayReq> {
        self.decode_pay_req_opt(req, ::grpcio::CallOption::default())
    }

    pub fn decode_pay_req_async_opt(&self, req: &super::rpc::PayReqString, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::PayReq>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DECODE_PAY_REQ, req, opt)
    }

    pub fn decode_pay_req_async(&self, req: &super::rpc::PayReqString) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::PayReq>> {
        self.decode_pay_req_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_payments_opt(&self, req: &super::rpc::ListPaymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ListPaymentsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_PAYMENTS, req, opt)
    }

    pub fn list_payments(&self, req: &super::rpc::ListPaymentsRequest) -> ::grpcio::Result<super::rpc::ListPaymentsResponse> {
        self.list_payments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_payments_async_opt(&self, req: &super::rpc::ListPaymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListPaymentsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_PAYMENTS, req, opt)
    }

    pub fn list_payments_async(&self, req: &super::rpc::ListPaymentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListPaymentsResponse>> {
        self.list_payments_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_all_payments_opt(&self, req: &super::rpc::DeleteAllPaymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::DeleteAllPaymentsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_DELETE_ALL_PAYMENTS, req, opt)
    }

    pub fn delete_all_payments(&self, req: &super::rpc::DeleteAllPaymentsRequest) -> ::grpcio::Result<super::rpc::DeleteAllPaymentsResponse> {
        self.delete_all_payments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_all_payments_async_opt(&self, req: &super::rpc::DeleteAllPaymentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DeleteAllPaymentsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DELETE_ALL_PAYMENTS, req, opt)
    }

    pub fn delete_all_payments_async(&self, req: &super::rpc::DeleteAllPaymentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DeleteAllPaymentsResponse>> {
        self.delete_all_payments_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn describe_graph_opt(&self, req: &super::rpc::ChannelGraphRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ChannelGraph> {
        self.client.unary_call(&METHOD_LIGHTNING_DESCRIBE_GRAPH, req, opt)
    }

    pub fn describe_graph(&self, req: &super::rpc::ChannelGraphRequest) -> ::grpcio::Result<super::rpc::ChannelGraph> {
        self.describe_graph_opt(req, ::grpcio::CallOption::default())
    }

    pub fn describe_graph_async_opt(&self, req: &super::rpc::ChannelGraphRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChannelGraph>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DESCRIBE_GRAPH, req, opt)
    }

    pub fn describe_graph_async(&self, req: &super::rpc::ChannelGraphRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChannelGraph>> {
        self.describe_graph_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_metrics_opt(&self, req: &super::rpc::NodeMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::NodeMetricsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_NODE_METRICS, req, opt)
    }

    pub fn get_node_metrics(&self, req: &super::rpc::NodeMetricsRequest) -> ::grpcio::Result<super::rpc::NodeMetricsResponse> {
        self.get_node_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_metrics_async_opt(&self, req: &super::rpc::NodeMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::NodeMetricsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_NODE_METRICS, req, opt)
    }

    pub fn get_node_metrics_async(&self, req: &super::rpc::NodeMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::NodeMetricsResponse>> {
        self.get_node_metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_chan_info_opt(&self, req: &super::rpc::ChanInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ChannelEdge> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_CHAN_INFO, req, opt)
    }

    pub fn get_chan_info(&self, req: &super::rpc::ChanInfoRequest) -> ::grpcio::Result<super::rpc::ChannelEdge> {
        self.get_chan_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_chan_info_async_opt(&self, req: &super::rpc::ChanInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChannelEdge>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_CHAN_INFO, req, opt)
    }

    pub fn get_chan_info_async(&self, req: &super::rpc::ChanInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChannelEdge>> {
        self.get_chan_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_info_opt(&self, req: &super::rpc::NodeInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::NodeInfo> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_NODE_INFO, req, opt)
    }

    pub fn get_node_info(&self, req: &super::rpc::NodeInfoRequest) -> ::grpcio::Result<super::rpc::NodeInfo> {
        self.get_node_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_info_async_opt(&self, req: &super::rpc::NodeInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::NodeInfo>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_NODE_INFO, req, opt)
    }

    pub fn get_node_info_async(&self, req: &super::rpc::NodeInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::NodeInfo>> {
        self.get_node_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_routes_opt(&self, req: &super::rpc::QueryRoutesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::QueryRoutesResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_QUERY_ROUTES, req, opt)
    }

    pub fn query_routes(&self, req: &super::rpc::QueryRoutesRequest) -> ::grpcio::Result<super::rpc::QueryRoutesResponse> {
        self.query_routes_opt(req, ::grpcio::CallOption::default())
    }

    pub fn query_routes_async_opt(&self, req: &super::rpc::QueryRoutesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::QueryRoutesResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_QUERY_ROUTES, req, opt)
    }

    pub fn query_routes_async(&self, req: &super::rpc::QueryRoutesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::QueryRoutesResponse>> {
        self.query_routes_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_network_info_opt(&self, req: &super::rpc::NetworkInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::NetworkInfo> {
        self.client.unary_call(&METHOD_LIGHTNING_GET_NETWORK_INFO, req, opt)
    }

    pub fn get_network_info(&self, req: &super::rpc::NetworkInfoRequest) -> ::grpcio::Result<super::rpc::NetworkInfo> {
        self.get_network_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_network_info_async_opt(&self, req: &super::rpc::NetworkInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::NetworkInfo>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_GET_NETWORK_INFO, req, opt)
    }

    pub fn get_network_info_async(&self, req: &super::rpc::NetworkInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::NetworkInfo>> {
        self.get_network_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_daemon_opt(&self, req: &super::rpc::StopRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::StopResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_STOP_DAEMON, req, opt)
    }

    pub fn stop_daemon(&self, req: &super::rpc::StopRequest) -> ::grpcio::Result<super::rpc::StopResponse> {
        self.stop_daemon_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_daemon_async_opt(&self, req: &super::rpc::StopRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::StopResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_STOP_DAEMON, req, opt)
    }

    pub fn stop_daemon_async(&self, req: &super::rpc::StopRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::StopResponse>> {
        self.stop_daemon_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_channel_graph_opt(&self, req: &super::rpc::GraphTopologySubscription, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::GraphTopologyUpdate>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_GRAPH, req, opt)
    }

    pub fn subscribe_channel_graph(&self, req: &super::rpc::GraphTopologySubscription) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::GraphTopologyUpdate>> {
        self.subscribe_channel_graph_opt(req, ::grpcio::CallOption::default())
    }

    pub fn debug_level_opt(&self, req: &super::rpc::DebugLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::DebugLevelResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_DEBUG_LEVEL, req, opt)
    }

    pub fn debug_level(&self, req: &super::rpc::DebugLevelRequest) -> ::grpcio::Result<super::rpc::DebugLevelResponse> {
        self.debug_level_opt(req, ::grpcio::CallOption::default())
    }

    pub fn debug_level_async_opt(&self, req: &super::rpc::DebugLevelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DebugLevelResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DEBUG_LEVEL, req, opt)
    }

    pub fn debug_level_async(&self, req: &super::rpc::DebugLevelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DebugLevelResponse>> {
        self.debug_level_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fee_report_opt(&self, req: &super::rpc::FeeReportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::FeeReportResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_FEE_REPORT, req, opt)
    }

    pub fn fee_report(&self, req: &super::rpc::FeeReportRequest) -> ::grpcio::Result<super::rpc::FeeReportResponse> {
        self.fee_report_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fee_report_async_opt(&self, req: &super::rpc::FeeReportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::FeeReportResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_FEE_REPORT, req, opt)
    }

    pub fn fee_report_async(&self, req: &super::rpc::FeeReportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::FeeReportResponse>> {
        self.fee_report_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_channel_policy_opt(&self, req: &super::rpc::PolicyUpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::PolicyUpdateResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_UPDATE_CHANNEL_POLICY, req, opt)
    }

    pub fn update_channel_policy(&self, req: &super::rpc::PolicyUpdateRequest) -> ::grpcio::Result<super::rpc::PolicyUpdateResponse> {
        self.update_channel_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_channel_policy_async_opt(&self, req: &super::rpc::PolicyUpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::PolicyUpdateResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_UPDATE_CHANNEL_POLICY, req, opt)
    }

    pub fn update_channel_policy_async(&self, req: &super::rpc::PolicyUpdateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::PolicyUpdateResponse>> {
        self.update_channel_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn forwarding_history_opt(&self, req: &super::rpc::ForwardingHistoryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ForwardingHistoryResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_FORWARDING_HISTORY, req, opt)
    }

    pub fn forwarding_history(&self, req: &super::rpc::ForwardingHistoryRequest) -> ::grpcio::Result<super::rpc::ForwardingHistoryResponse> {
        self.forwarding_history_opt(req, ::grpcio::CallOption::default())
    }

    pub fn forwarding_history_async_opt(&self, req: &super::rpc::ForwardingHistoryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ForwardingHistoryResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_FORWARDING_HISTORY, req, opt)
    }

    pub fn forwarding_history_async(&self, req: &super::rpc::ForwardingHistoryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ForwardingHistoryResponse>> {
        self.forwarding_history_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_channel_backup_opt(&self, req: &super::rpc::ExportChannelBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ChannelBackup> {
        self.client.unary_call(&METHOD_LIGHTNING_EXPORT_CHANNEL_BACKUP, req, opt)
    }

    pub fn export_channel_backup(&self, req: &super::rpc::ExportChannelBackupRequest) -> ::grpcio::Result<super::rpc::ChannelBackup> {
        self.export_channel_backup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_channel_backup_async_opt(&self, req: &super::rpc::ExportChannelBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChannelBackup>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_EXPORT_CHANNEL_BACKUP, req, opt)
    }

    pub fn export_channel_backup_async(&self, req: &super::rpc::ExportChannelBackupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChannelBackup>> {
        self.export_channel_backup_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_all_channel_backups_opt(&self, req: &super::rpc::ChanBackupExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ChanBackupSnapshot> {
        self.client.unary_call(&METHOD_LIGHTNING_EXPORT_ALL_CHANNEL_BACKUPS, req, opt)
    }

    pub fn export_all_channel_backups(&self, req: &super::rpc::ChanBackupExportRequest) -> ::grpcio::Result<super::rpc::ChanBackupSnapshot> {
        self.export_all_channel_backups_opt(req, ::grpcio::CallOption::default())
    }

    pub fn export_all_channel_backups_async_opt(&self, req: &super::rpc::ChanBackupExportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChanBackupSnapshot>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_EXPORT_ALL_CHANNEL_BACKUPS, req, opt)
    }

    pub fn export_all_channel_backups_async(&self, req: &super::rpc::ChanBackupExportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ChanBackupSnapshot>> {
        self.export_all_channel_backups_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_chan_backup_opt(&self, req: &super::rpc::ChanBackupSnapshot, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::VerifyChanBackupResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_VERIFY_CHAN_BACKUP, req, opt)
    }

    pub fn verify_chan_backup(&self, req: &super::rpc::ChanBackupSnapshot) -> ::grpcio::Result<super::rpc::VerifyChanBackupResponse> {
        self.verify_chan_backup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_chan_backup_async_opt(&self, req: &super::rpc::ChanBackupSnapshot, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::VerifyChanBackupResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_VERIFY_CHAN_BACKUP, req, opt)
    }

    pub fn verify_chan_backup_async(&self, req: &super::rpc::ChanBackupSnapshot) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::VerifyChanBackupResponse>> {
        self.verify_chan_backup_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn restore_channel_backups_opt(&self, req: &super::rpc::RestoreChanBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::RestoreBackupResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_RESTORE_CHANNEL_BACKUPS, req, opt)
    }

    pub fn restore_channel_backups(&self, req: &super::rpc::RestoreChanBackupRequest) -> ::grpcio::Result<super::rpc::RestoreBackupResponse> {
        self.restore_channel_backups_opt(req, ::grpcio::CallOption::default())
    }

    pub fn restore_channel_backups_async_opt(&self, req: &super::rpc::RestoreChanBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::RestoreBackupResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_RESTORE_CHANNEL_BACKUPS, req, opt)
    }

    pub fn restore_channel_backups_async(&self, req: &super::rpc::RestoreChanBackupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::RestoreBackupResponse>> {
        self.restore_channel_backups_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn subscribe_channel_backups_opt(&self, req: &super::rpc::ChannelBackupSubscription, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::ChanBackupSnapshot>> {
        self.client.server_streaming(&METHOD_LIGHTNING_SUBSCRIBE_CHANNEL_BACKUPS, req, opt)
    }

    pub fn subscribe_channel_backups(&self, req: &super::rpc::ChannelBackupSubscription) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::ChanBackupSnapshot>> {
        self.subscribe_channel_backups_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bake_macaroon_opt(&self, req: &super::rpc::BakeMacaroonRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::BakeMacaroonResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_BAKE_MACAROON, req, opt)
    }

    pub fn bake_macaroon(&self, req: &super::rpc::BakeMacaroonRequest) -> ::grpcio::Result<super::rpc::BakeMacaroonResponse> {
        self.bake_macaroon_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bake_macaroon_async_opt(&self, req: &super::rpc::BakeMacaroonRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::BakeMacaroonResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_BAKE_MACAROON, req, opt)
    }

    pub fn bake_macaroon_async(&self, req: &super::rpc::BakeMacaroonRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::BakeMacaroonResponse>> {
        self.bake_macaroon_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_macaroon_i_ds_opt(&self, req: &super::rpc::ListMacaroonIDsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ListMacaroonIDsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_MACAROON_I_DS, req, opt)
    }

    pub fn list_macaroon_i_ds(&self, req: &super::rpc::ListMacaroonIDsRequest) -> ::grpcio::Result<super::rpc::ListMacaroonIDsResponse> {
        self.list_macaroon_i_ds_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_macaroon_i_ds_async_opt(&self, req: &super::rpc::ListMacaroonIDsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListMacaroonIDsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_MACAROON_I_DS, req, opt)
    }

    pub fn list_macaroon_i_ds_async(&self, req: &super::rpc::ListMacaroonIDsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListMacaroonIDsResponse>> {
        self.list_macaroon_i_ds_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_macaroon_id_opt(&self, req: &super::rpc::DeleteMacaroonIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::DeleteMacaroonIDResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_DELETE_MACAROON_ID, req, opt)
    }

    pub fn delete_macaroon_id(&self, req: &super::rpc::DeleteMacaroonIDRequest) -> ::grpcio::Result<super::rpc::DeleteMacaroonIDResponse> {
        self.delete_macaroon_id_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_macaroon_id_async_opt(&self, req: &super::rpc::DeleteMacaroonIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DeleteMacaroonIDResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_DELETE_MACAROON_ID, req, opt)
    }

    pub fn delete_macaroon_id_async(&self, req: &super::rpc::DeleteMacaroonIDRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::DeleteMacaroonIDResponse>> {
        self.delete_macaroon_id_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_permissions_opt(&self, req: &super::rpc::ListPermissionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::rpc::ListPermissionsResponse> {
        self.client.unary_call(&METHOD_LIGHTNING_LIST_PERMISSIONS, req, opt)
    }

    pub fn list_permissions(&self, req: &super::rpc::ListPermissionsRequest) -> ::grpcio::Result<super::rpc::ListPermissionsResponse> {
        self.list_permissions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_permissions_async_opt(&self, req: &super::rpc::ListPermissionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListPermissionsResponse>> {
        self.client.unary_call_async(&METHOD_LIGHTNING_LIST_PERMISSIONS, req, opt)
    }

    pub fn list_permissions_async(&self, req: &super::rpc::ListPermissionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::rpc::ListPermissionsResponse>> {
        self.list_permissions_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Lightning {
    fn wallet_balance(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::WalletBalanceRequest, sink: ::grpcio::UnarySink<super::rpc::WalletBalanceResponse>);
    fn channel_balance(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ChannelBalanceRequest, sink: ::grpcio::UnarySink<super::rpc::ChannelBalanceResponse>);
    fn get_transactions(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::GetTransactionsRequest, sink: ::grpcio::UnarySink<super::rpc::TransactionDetails>);
    fn estimate_fee(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::EstimateFeeRequest, sink: ::grpcio::UnarySink<super::rpc::EstimateFeeResponse>);
    fn send_coins(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::SendCoinsRequest, sink: ::grpcio::UnarySink<super::rpc::SendCoinsResponse>);
    fn list_unspent(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ListUnspentRequest, sink: ::grpcio::UnarySink<super::rpc::ListUnspentResponse>);
    fn subscribe_transactions(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::GetTransactionsRequest, sink: ::grpcio::ServerStreamingSink<super::rpc::Transaction>);
    fn send_many(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::SendManyRequest, sink: ::grpcio::UnarySink<super::rpc::SendManyResponse>);
    fn new_address(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::NewAddressRequest, sink: ::grpcio::UnarySink<super::rpc::NewAddressResponse>);
    fn sign_message(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::SignMessageRequest, sink: ::grpcio::UnarySink<super::rpc::SignMessageResponse>);
    fn verify_message(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::VerifyMessageRequest, sink: ::grpcio::UnarySink<super::rpc::VerifyMessageResponse>);
    fn connect_peer(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ConnectPeerRequest, sink: ::grpcio::UnarySink<super::rpc::ConnectPeerResponse>);
    fn disconnect_peer(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::DisconnectPeerRequest, sink: ::grpcio::UnarySink<super::rpc::DisconnectPeerResponse>);
    fn list_peers(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ListPeersRequest, sink: ::grpcio::UnarySink<super::rpc::ListPeersResponse>);
    fn subscribe_peer_events(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::PeerEventSubscription, sink: ::grpcio::ServerStreamingSink<super::rpc::PeerEvent>);
    fn get_info(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::GetInfoRequest, sink: ::grpcio::UnarySink<super::rpc::GetInfoResponse>);
    fn get_recovery_info(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::GetRecoveryInfoRequest, sink: ::grpcio::UnarySink<super::rpc::GetRecoveryInfoResponse>);
    fn pending_channels(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::PendingChannelsRequest, sink: ::grpcio::UnarySink<super::rpc::PendingChannelsResponse>);
    fn list_channels(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ListChannelsRequest, sink: ::grpcio::UnarySink<super::rpc::ListChannelsResponse>);
    fn subscribe_channel_events(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ChannelEventSubscription, sink: ::grpcio::ServerStreamingSink<super::rpc::ChannelEventUpdate>);
    fn closed_channels(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ClosedChannelsRequest, sink: ::grpcio::UnarySink<super::rpc::ClosedChannelsResponse>);
    fn open_channel_sync(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::OpenChannelRequest, sink: ::grpcio::UnarySink<super::rpc::ChannelPoint>);
    fn open_channel(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::OpenChannelRequest, sink: ::grpcio::ServerStreamingSink<super::rpc::OpenStatusUpdate>);
    fn funding_state_step(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::FundingTransitionMsg, sink: ::grpcio::UnarySink<super::rpc::FundingStateStepResp>);
    fn channel_acceptor(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::rpc::ChannelAcceptResponse>, sink: ::grpcio::DuplexSink<super::rpc::ChannelAcceptRequest>);
    fn close_channel(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::CloseChannelRequest, sink: ::grpcio::ServerStreamingSink<super::rpc::CloseStatusUpdate>);
    fn abandon_channel(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::AbandonChannelRequest, sink: ::grpcio::UnarySink<super::rpc::AbandonChannelResponse>);
    fn send_payment(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::rpc::SendRequest>, sink: ::grpcio::DuplexSink<super::rpc::SendResponse>);
    fn send_payment_sync(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::SendRequest, sink: ::grpcio::UnarySink<super::rpc::SendResponse>);
    fn send_to_route(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::rpc::SendToRouteRequest>, sink: ::grpcio::DuplexSink<super::rpc::SendResponse>);
    fn send_to_route_sync(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::SendToRouteRequest, sink: ::grpcio::UnarySink<super::rpc::SendResponse>);
    fn add_invoice(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::Invoice, sink: ::grpcio::UnarySink<super::rpc::AddInvoiceResponse>);
    fn list_invoices(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ListInvoiceRequest, sink: ::grpcio::UnarySink<super::rpc::ListInvoiceResponse>);
    fn lookup_invoice(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::PaymentHash, sink: ::grpcio::UnarySink<super::rpc::Invoice>);
    fn subscribe_invoices(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::InvoiceSubscription, sink: ::grpcio::ServerStreamingSink<super::rpc::Invoice>);
    fn decode_pay_req(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::PayReqString, sink: ::grpcio::UnarySink<super::rpc::PayReq>);
    fn list_payments(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ListPaymentsRequest, sink: ::grpcio::UnarySink<super::rpc::ListPaymentsResponse>);
    fn delete_all_payments(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::DeleteAllPaymentsRequest, sink: ::grpcio::UnarySink<super::rpc::DeleteAllPaymentsResponse>);
    fn describe_graph(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ChannelGraphRequest, sink: ::grpcio::UnarySink<super::rpc::ChannelGraph>);
    fn get_node_metrics(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::NodeMetricsRequest, sink: ::grpcio::UnarySink<super::rpc::NodeMetricsResponse>);
    fn get_chan_info(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ChanInfoRequest, sink: ::grpcio::UnarySink<super::rpc::ChannelEdge>);
    fn get_node_info(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::NodeInfoRequest, sink: ::grpcio::UnarySink<super::rpc::NodeInfo>);
    fn query_routes(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::QueryRoutesRequest, sink: ::grpcio::UnarySink<super::rpc::QueryRoutesResponse>);
    fn get_network_info(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::NetworkInfoRequest, sink: ::grpcio::UnarySink<super::rpc::NetworkInfo>);
    fn stop_daemon(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::StopRequest, sink: ::grpcio::UnarySink<super::rpc::StopResponse>);
    fn subscribe_channel_graph(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::GraphTopologySubscription, sink: ::grpcio::ServerStreamingSink<super::rpc::GraphTopologyUpdate>);
    fn debug_level(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::DebugLevelRequest, sink: ::grpcio::UnarySink<super::rpc::DebugLevelResponse>);
    fn fee_report(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::FeeReportRequest, sink: ::grpcio::UnarySink<super::rpc::FeeReportResponse>);
    fn update_channel_policy(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::PolicyUpdateRequest, sink: ::grpcio::UnarySink<super::rpc::PolicyUpdateResponse>);
    fn forwarding_history(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ForwardingHistoryRequest, sink: ::grpcio::UnarySink<super::rpc::ForwardingHistoryResponse>);
    fn export_channel_backup(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ExportChannelBackupRequest, sink: ::grpcio::UnarySink<super::rpc::ChannelBackup>);
    fn export_all_channel_backups(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ChanBackupExportRequest, sink: ::grpcio::UnarySink<super::rpc::ChanBackupSnapshot>);
    fn verify_chan_backup(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ChanBackupSnapshot, sink: ::grpcio::UnarySink<super::rpc::VerifyChanBackupResponse>);
    fn restore_channel_backups(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::RestoreChanBackupRequest, sink: ::grpcio::UnarySink<super::rpc::RestoreBackupResponse>);
    fn subscribe_channel_backups(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ChannelBackupSubscription, sink: ::grpcio::ServerStreamingSink<super::rpc::ChanBackupSnapshot>);
    fn bake_macaroon(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::BakeMacaroonRequest, sink: ::grpcio::UnarySink<super::rpc::BakeMacaroonResponse>);
    fn list_macaroon_i_ds(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ListMacaroonIDsRequest, sink: ::grpcio::UnarySink<super::rpc::ListMacaroonIDsResponse>);
    fn delete_macaroon_id(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::DeleteMacaroonIDRequest, sink: ::grpcio::UnarySink<super::rpc::DeleteMacaroonIDResponse>);
    fn list_permissions(&mut self, ctx: ::grpcio::RpcContext, req: super::rpc::ListPermissionsRequest, sink: ::grpcio::UnarySink<super::rpc::ListPermissionsResponse>);
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
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_LIGHTNING_LIST_PERMISSIONS, move |ctx, req, resp| {
        instance.list_permissions(ctx, req, resp)
    });
    builder.build()
}
