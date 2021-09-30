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

const METHOD_INVOICES_SUBSCRIBE_SINGLE_INVOICE: ::grpcio::Method<super::invoices::SubscribeSingleInvoiceRequest, super::rpc::Invoice> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/invoicesrpc.Invoices/SubscribeSingleInvoice",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INVOICES_CANCEL_INVOICE: ::grpcio::Method<super::invoices::CancelInvoiceMsg, super::invoices::CancelInvoiceResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/invoicesrpc.Invoices/CancelInvoice",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INVOICES_ADD_HOLD_INVOICE: ::grpcio::Method<super::invoices::AddHoldInvoiceRequest, super::invoices::AddHoldInvoiceResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/invoicesrpc.Invoices/AddHoldInvoice",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INVOICES_SETTLE_INVOICE: ::grpcio::Method<super::invoices::SettleInvoiceMsg, super::invoices::SettleInvoiceResp> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/invoicesrpc.Invoices/SettleInvoice",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct InvoicesClient {
    client: ::grpcio::Client,
}

impl InvoicesClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        InvoicesClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn subscribe_single_invoice_opt(&self, req: &super::invoices::SubscribeSingleInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::Invoice>> {
        self.client.server_streaming(&METHOD_INVOICES_SUBSCRIBE_SINGLE_INVOICE, req, opt)
    }

    pub fn subscribe_single_invoice(&self, req: &super::invoices::SubscribeSingleInvoiceRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::rpc::Invoice>> {
        self.subscribe_single_invoice_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_invoice_opt(&self, req: &super::invoices::CancelInvoiceMsg, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::invoices::CancelInvoiceResp> {
        self.client.unary_call(&METHOD_INVOICES_CANCEL_INVOICE, req, opt)
    }

    pub fn cancel_invoice(&self, req: &super::invoices::CancelInvoiceMsg) -> ::grpcio::Result<super::invoices::CancelInvoiceResp> {
        self.cancel_invoice_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_invoice_async_opt(&self, req: &super::invoices::CancelInvoiceMsg, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoices::CancelInvoiceResp>> {
        self.client.unary_call_async(&METHOD_INVOICES_CANCEL_INVOICE, req, opt)
    }

    pub fn cancel_invoice_async(&self, req: &super::invoices::CancelInvoiceMsg) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoices::CancelInvoiceResp>> {
        self.cancel_invoice_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_hold_invoice_opt(&self, req: &super::invoices::AddHoldInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::invoices::AddHoldInvoiceResp> {
        self.client.unary_call(&METHOD_INVOICES_ADD_HOLD_INVOICE, req, opt)
    }

    pub fn add_hold_invoice(&self, req: &super::invoices::AddHoldInvoiceRequest) -> ::grpcio::Result<super::invoices::AddHoldInvoiceResp> {
        self.add_hold_invoice_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_hold_invoice_async_opt(&self, req: &super::invoices::AddHoldInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoices::AddHoldInvoiceResp>> {
        self.client.unary_call_async(&METHOD_INVOICES_ADD_HOLD_INVOICE, req, opt)
    }

    pub fn add_hold_invoice_async(&self, req: &super::invoices::AddHoldInvoiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoices::AddHoldInvoiceResp>> {
        self.add_hold_invoice_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn settle_invoice_opt(&self, req: &super::invoices::SettleInvoiceMsg, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::invoices::SettleInvoiceResp> {
        self.client.unary_call(&METHOD_INVOICES_SETTLE_INVOICE, req, opt)
    }

    pub fn settle_invoice(&self, req: &super::invoices::SettleInvoiceMsg) -> ::grpcio::Result<super::invoices::SettleInvoiceResp> {
        self.settle_invoice_opt(req, ::grpcio::CallOption::default())
    }

    pub fn settle_invoice_async_opt(&self, req: &super::invoices::SettleInvoiceMsg, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoices::SettleInvoiceResp>> {
        self.client.unary_call_async(&METHOD_INVOICES_SETTLE_INVOICE, req, opt)
    }

    pub fn settle_invoice_async(&self, req: &super::invoices::SettleInvoiceMsg) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoices::SettleInvoiceResp>> {
        self.settle_invoice_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Invoices {
    fn subscribe_single_invoice(&mut self, ctx: ::grpcio::RpcContext, req: super::invoices::SubscribeSingleInvoiceRequest, sink: ::grpcio::ServerStreamingSink<super::rpc::Invoice>);
    fn cancel_invoice(&mut self, ctx: ::grpcio::RpcContext, req: super::invoices::CancelInvoiceMsg, sink: ::grpcio::UnarySink<super::invoices::CancelInvoiceResp>);
    fn add_hold_invoice(&mut self, ctx: ::grpcio::RpcContext, req: super::invoices::AddHoldInvoiceRequest, sink: ::grpcio::UnarySink<super::invoices::AddHoldInvoiceResp>);
    fn settle_invoice(&mut self, ctx: ::grpcio::RpcContext, req: super::invoices::SettleInvoiceMsg, sink: ::grpcio::UnarySink<super::invoices::SettleInvoiceResp>);
}

pub fn create_invoices<S: Invoices + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_INVOICES_SUBSCRIBE_SINGLE_INVOICE, move |ctx, req, resp| {
        instance.subscribe_single_invoice(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INVOICES_CANCEL_INVOICE, move |ctx, req, resp| {
        instance.cancel_invoice(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INVOICES_ADD_HOLD_INVOICE, move |ctx, req, resp| {
        instance.add_hold_invoice(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_INVOICES_SETTLE_INVOICE, move |ctx, req, resp| {
        instance.settle_invoice(ctx, req, resp)
    });
    builder.build()
}
