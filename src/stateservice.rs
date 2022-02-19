// This file is generated by rust-protobuf 2.23.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `stateservice.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_23_0;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct SubscribeStateRequest {
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SubscribeStateRequest {
    fn default() -> &'a SubscribeStateRequest {
        <SubscribeStateRequest as ::protobuf::Message>::default_instance()
    }
}

impl SubscribeStateRequest {
    pub fn new() -> SubscribeStateRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for SubscribeStateRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SubscribeStateRequest {
        SubscribeStateRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SubscribeStateRequest>(
                "SubscribeStateRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SubscribeStateRequest {
        static instance: ::protobuf::rt::LazyV2<SubscribeStateRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SubscribeStateRequest::new)
    }
}

impl ::protobuf::Clear for SubscribeStateRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubscribeStateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubscribeStateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct SubscribeStateResponse {
    // message fields
    pub state: WalletState,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SubscribeStateResponse {
    fn default() -> &'a SubscribeStateResponse {
        <SubscribeStateResponse as ::protobuf::Message>::default_instance()
    }
}

impl SubscribeStateResponse {
    pub fn new() -> SubscribeStateResponse {
        ::std::default::Default::default()
    }

    // .lnrpc.WalletState state = 1;


    pub fn get_state(&self) -> WalletState {
        self.state
    }
    pub fn clear_state(&mut self) {
        self.state = WalletState::NON_EXISTING;
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: WalletState) {
        self.state = v;
    }
}

impl ::protobuf::Message for SubscribeStateResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.state, 1, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.state != WalletState::NON_EXISTING {
            my_size += ::protobuf::rt::enum_size(1, self.state);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.state != WalletState::NON_EXISTING {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.state))?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SubscribeStateResponse {
        SubscribeStateResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<WalletState>>(
                "state",
                |m: &SubscribeStateResponse| { &m.state },
                |m: &mut SubscribeStateResponse| { &mut m.state },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SubscribeStateResponse>(
                "SubscribeStateResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SubscribeStateResponse {
        static instance: ::protobuf::rt::LazyV2<SubscribeStateResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SubscribeStateResponse::new)
    }
}

impl ::protobuf::Clear for SubscribeStateResponse {
    fn clear(&mut self) {
        self.state = WalletState::NON_EXISTING;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubscribeStateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubscribeStateResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct GetStateRequest {
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetStateRequest {
    fn default() -> &'a GetStateRequest {
        <GetStateRequest as ::protobuf::Message>::default_instance()
    }
}

impl GetStateRequest {
    pub fn new() -> GetStateRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for GetStateRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GetStateRequest {
        GetStateRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GetStateRequest>(
                "GetStateRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GetStateRequest {
        static instance: ::protobuf::rt::LazyV2<GetStateRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GetStateRequest::new)
    }
}

impl ::protobuf::Clear for GetStateRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetStateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetStateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct GetStateResponse {
    // message fields
    pub state: WalletState,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetStateResponse {
    fn default() -> &'a GetStateResponse {
        <GetStateResponse as ::protobuf::Message>::default_instance()
    }
}

impl GetStateResponse {
    pub fn new() -> GetStateResponse {
        ::std::default::Default::default()
    }

    // .lnrpc.WalletState state = 1;


    pub fn get_state(&self) -> WalletState {
        self.state
    }
    pub fn clear_state(&mut self) {
        self.state = WalletState::NON_EXISTING;
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: WalletState) {
        self.state = v;
    }
}

impl ::protobuf::Message for GetStateResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.state, 1, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.state != WalletState::NON_EXISTING {
            my_size += ::protobuf::rt::enum_size(1, self.state);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.state != WalletState::NON_EXISTING {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.state))?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GetStateResponse {
        GetStateResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<WalletState>>(
                "state",
                |m: &GetStateResponse| { &m.state },
                |m: &mut GetStateResponse| { &mut m.state },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GetStateResponse>(
                "GetStateResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GetStateResponse {
        static instance: ::protobuf::rt::LazyV2<GetStateResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GetStateResponse::new)
    }
}

impl ::protobuf::Clear for GetStateResponse {
    fn clear(&mut self) {
        self.state = WalletState::NON_EXISTING;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetStateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetStateResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum WalletState {
    NON_EXISTING = 0,
    LOCKED = 1,
    UNLOCKED = 2,
    RPC_ACTIVE = 3,
    SERVER_ACTIVE = 4,
    WAITING_TO_START = 255,
}

impl ::protobuf::ProtobufEnum for WalletState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<WalletState> {
        match value {
            0 => ::std::option::Option::Some(WalletState::NON_EXISTING),
            1 => ::std::option::Option::Some(WalletState::LOCKED),
            2 => ::std::option::Option::Some(WalletState::UNLOCKED),
            3 => ::std::option::Option::Some(WalletState::RPC_ACTIVE),
            4 => ::std::option::Option::Some(WalletState::SERVER_ACTIVE),
            255 => ::std::option::Option::Some(WalletState::WAITING_TO_START),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [WalletState] = &[
            WalletState::NON_EXISTING,
            WalletState::LOCKED,
            WalletState::UNLOCKED,
            WalletState::RPC_ACTIVE,
            WalletState::SERVER_ACTIVE,
            WalletState::WAITING_TO_START,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<WalletState>("WalletState", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for WalletState {
}

impl ::std::default::Default for WalletState {
    fn default() -> Self {
        WalletState::NON_EXISTING
    }
}

impl ::protobuf::reflect::ProtobufValue for WalletState {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12stateservice.proto\x12\x05lnrpc\"\x17\n\x15SubscribeStateRequest\"\
    B\n\x16SubscribeStateResponse\x12(\n\x05state\x18\x01\x20\x01(\x0e2\x12.\
    lnrpc.WalletStateR\x05state\"\x11\n\x0fGetStateRequest\"<\n\x10GetStateR\
    esponse\x12(\n\x05state\x18\x01\x20\x01(\x0e2\x12.lnrpc.WalletStateR\x05\
    state*s\n\x0bWalletState\x12\x10\n\x0cNON_EXISTING\x10\0\x12\n\n\x06LOCK\
    ED\x10\x01\x12\x0c\n\x08UNLOCKED\x10\x02\x12\x0e\n\nRPC_ACTIVE\x10\x03\
    \x12\x11\n\rSERVER_ACTIVE\x10\x04\x12\x15\n\x10WAITING_TO_START\x10\xff\
    \x012\x95\x01\n\x05State\x12O\n\x0eSubscribeState\x12\x1c.lnrpc.Subscrib\
    eStateRequest\x1a\x1d.lnrpc.SubscribeStateResponse0\x01\x12;\n\x08GetSta\
    te\x12\x16.lnrpc.GetStateRequest\x1a\x17.lnrpc.GetStateResponseB'Z%githu\
    b.com/lightningnetwork/lnd/lnrpcb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
