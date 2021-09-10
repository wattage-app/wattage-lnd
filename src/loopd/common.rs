// This file is generated by rust-protobuf 2.24.1. Do not edit
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
//! Generated file from `common.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_24_1;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct HopHint {
    // message fields
    pub node_id: ::std::string::String,
    pub chan_id: u64,
    pub fee_base_msat: u32,
    pub fee_proportional_millionths: u32,
    pub cltv_expiry_delta: u32,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HopHint {
    fn default() -> &'a HopHint {
        <HopHint as ::protobuf::Message>::default_instance()
    }
}

impl HopHint {
    pub fn new() -> HopHint {
        ::std::default::Default::default()
    }

    // string node_id = 1;


    pub fn get_node_id(&self) -> &str {
        &self.node_id
    }
    pub fn clear_node_id(&mut self) {
        self.node_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: ::std::string::String) {
        self.node_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node_id(&mut self) -> &mut ::std::string::String {
        &mut self.node_id
    }

    // Take field
    pub fn take_node_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.node_id, ::std::string::String::new())
    }

    // uint64 chan_id = 2;


    pub fn get_chan_id(&self) -> u64 {
        self.chan_id
    }
    pub fn clear_chan_id(&mut self) {
        self.chan_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_chan_id(&mut self, v: u64) {
        self.chan_id = v;
    }

    // uint32 fee_base_msat = 3;


    pub fn get_fee_base_msat(&self) -> u32 {
        self.fee_base_msat
    }
    pub fn clear_fee_base_msat(&mut self) {
        self.fee_base_msat = 0;
    }

    // Param is passed by value, moved
    pub fn set_fee_base_msat(&mut self, v: u32) {
        self.fee_base_msat = v;
    }

    // uint32 fee_proportional_millionths = 4;


    pub fn get_fee_proportional_millionths(&self) -> u32 {
        self.fee_proportional_millionths
    }
    pub fn clear_fee_proportional_millionths(&mut self) {
        self.fee_proportional_millionths = 0;
    }

    // Param is passed by value, moved
    pub fn set_fee_proportional_millionths(&mut self, v: u32) {
        self.fee_proportional_millionths = v;
    }

    // uint32 cltv_expiry_delta = 5;


    pub fn get_cltv_expiry_delta(&self) -> u32 {
        self.cltv_expiry_delta
    }
    pub fn clear_cltv_expiry_delta(&mut self) {
        self.cltv_expiry_delta = 0;
    }

    // Param is passed by value, moved
    pub fn set_cltv_expiry_delta(&mut self, v: u32) {
        self.cltv_expiry_delta = v;
    }
}

impl ::protobuf::Message for HopHint {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.node_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.chan_id = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.fee_base_msat = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.fee_proportional_millionths = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cltv_expiry_delta = tmp;
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
        if !self.node_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.node_id);
        }
        if self.chan_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.chan_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fee_base_msat != 0 {
            my_size += ::protobuf::rt::value_size(3, self.fee_base_msat, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.fee_proportional_millionths != 0 {
            my_size += ::protobuf::rt::value_size(4, self.fee_proportional_millionths, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.cltv_expiry_delta != 0 {
            my_size += ::protobuf::rt::value_size(5, self.cltv_expiry_delta, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.node_id.is_empty() {
            os.write_string(1, &self.node_id)?;
        }
        if self.chan_id != 0 {
            os.write_uint64(2, self.chan_id)?;
        }
        if self.fee_base_msat != 0 {
            os.write_uint32(3, self.fee_base_msat)?;
        }
        if self.fee_proportional_millionths != 0 {
            os.write_uint32(4, self.fee_proportional_millionths)?;
        }
        if self.cltv_expiry_delta != 0 {
            os.write_uint32(5, self.cltv_expiry_delta)?;
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

    fn new() -> HopHint {
        HopHint::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "node_id",
                |m: &HopHint| { &m.node_id },
                |m: &mut HopHint| { &mut m.node_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "chan_id",
                |m: &HopHint| { &m.chan_id },
                |m: &mut HopHint| { &mut m.chan_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "fee_base_msat",
                |m: &HopHint| { &m.fee_base_msat },
                |m: &mut HopHint| { &mut m.fee_base_msat },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "fee_proportional_millionths",
                |m: &HopHint| { &m.fee_proportional_millionths },
                |m: &mut HopHint| { &mut m.fee_proportional_millionths },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "cltv_expiry_delta",
                |m: &HopHint| { &m.cltv_expiry_delta },
                |m: &mut HopHint| { &mut m.cltv_expiry_delta },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<HopHint>(
                "HopHint",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static HopHint {
        static instance: ::protobuf::rt::LazyV2<HopHint> = ::protobuf::rt::LazyV2::INIT;
        instance.get(HopHint::new)
    }
}

impl ::protobuf::Clear for HopHint {
    fn clear(&mut self) {
        self.node_id.clear();
        self.chan_id = 0;
        self.fee_base_msat = 0;
        self.fee_proportional_millionths = 0;
        self.cltv_expiry_delta = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HopHint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HopHint {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct RouteHint {
    // message fields
    pub hop_hints: ::protobuf::RepeatedField<HopHint>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RouteHint {
    fn default() -> &'a RouteHint {
        <RouteHint as ::protobuf::Message>::default_instance()
    }
}

impl RouteHint {
    pub fn new() -> RouteHint {
        ::std::default::Default::default()
    }

    // repeated .looprpc.HopHint hop_hints = 1;


    pub fn get_hop_hints(&self) -> &[HopHint] {
        &self.hop_hints
    }
    pub fn clear_hop_hints(&mut self) {
        self.hop_hints.clear();
    }

    // Param is passed by value, moved
    pub fn set_hop_hints(&mut self, v: ::protobuf::RepeatedField<HopHint>) {
        self.hop_hints = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hop_hints(&mut self) -> &mut ::protobuf::RepeatedField<HopHint> {
        &mut self.hop_hints
    }

    // Take field
    pub fn take_hop_hints(&mut self) -> ::protobuf::RepeatedField<HopHint> {
        ::std::mem::replace(&mut self.hop_hints, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for RouteHint {
    fn is_initialized(&self) -> bool {
        for v in &self.hop_hints {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.hop_hints)?;
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
        for value in &self.hop_hints {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.hop_hints {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> RouteHint {
        RouteHint::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HopHint>>(
                "hop_hints",
                |m: &RouteHint| { &m.hop_hints },
                |m: &mut RouteHint| { &mut m.hop_hints },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RouteHint>(
                "RouteHint",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RouteHint {
        static instance: ::protobuf::rt::LazyV2<RouteHint> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RouteHint::new)
    }
}

impl ::protobuf::Clear for RouteHint {
    fn clear(&mut self) {
        self.hop_hints.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteHint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteHint {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ccommon.proto\x12\x07looprpc\"\xcb\x01\n\x07HopHint\x12\x17\n\x07no\
    de_id\x18\x01\x20\x01(\tR\x06nodeId\x12\x17\n\x07chan_id\x18\x02\x20\x01\
    (\x04R\x06chanId\x12\"\n\rfee_base_msat\x18\x03\x20\x01(\rR\x0bfeeBaseMs\
    at\x12>\n\x1bfee_proportional_millionths\x18\x04\x20\x01(\rR\x19feePropo\
    rtionalMillionths\x12*\n\x11cltv_expiry_delta\x18\x05\x20\x01(\rR\x0fclt\
    vExpiryDelta\":\n\tRouteHint\x12-\n\thop_hints\x18\x01\x20\x03(\x0b2\x10\
    .looprpc.HopHintR\x08hopHintsB'Z%github.com/lightninglabs/loop/looprpcb\
    \x06proto3\
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
