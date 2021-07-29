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
//! Generated file from `example/shoppingcart/shoppingcart.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_24_1;

#[derive(PartialEq,Clone,Default)]
pub struct AddLineItem {
    // message fields
    pub user_id: ::std::string::String,
    pub product_id: ::std::string::String,
    pub name: ::std::string::String,
    pub quantity: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a AddLineItem {
    fn default() -> &'a AddLineItem {
        <AddLineItem as ::protobuf::Message>::default_instance()
    }
}

impl AddLineItem {
    pub fn new() -> AddLineItem {
        ::std::default::Default::default()
    }

    // string user_id = 1;


    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
    pub fn clear_user_id(&mut self) {
        self.user_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: ::std::string::String) {
        self.user_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_id(&mut self) -> &mut ::std::string::String {
        &mut self.user_id
    }

    // Take field
    pub fn take_user_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_id, ::std::string::String::new())
    }

    // string product_id = 2;


    pub fn get_product_id(&self) -> &str {
        &self.product_id
    }
    pub fn clear_product_id(&mut self) {
        self.product_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_product_id(&mut self, v: ::std::string::String) {
        self.product_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_product_id(&mut self) -> &mut ::std::string::String {
        &mut self.product_id
    }

    // Take field
    pub fn take_product_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.product_id, ::std::string::String::new())
    }

    // string name = 3;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // int32 quantity = 4;


    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }
    pub fn clear_quantity(&mut self) {
        self.quantity = 0;
    }

    // Param is passed by value, moved
    pub fn set_quantity(&mut self, v: i32) {
        self.quantity = v;
    }
}

impl ::protobuf::Message for AddLineItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.product_id)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.quantity = tmp;
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
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.user_id);
        }
        if !self.product_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.product_id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        }
        if self.quantity != 0 {
            my_size += ::protobuf::rt::value_size(4, self.quantity, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.user_id.is_empty() {
            os.write_string(1, &self.user_id)?;
        }
        if !self.product_id.is_empty() {
            os.write_string(2, &self.product_id)?;
        }
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
        }
        if self.quantity != 0 {
            os.write_int32(4, self.quantity)?;
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

    fn new() -> AddLineItem {
        AddLineItem::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "user_id",
                |m: &AddLineItem| { &m.user_id },
                |m: &mut AddLineItem| { &mut m.user_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "product_id",
                |m: &AddLineItem| { &m.product_id },
                |m: &mut AddLineItem| { &mut m.product_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &AddLineItem| { &m.name },
                |m: &mut AddLineItem| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "quantity",
                |m: &AddLineItem| { &m.quantity },
                |m: &mut AddLineItem| { &mut m.quantity },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<AddLineItem>(
                "AddLineItem",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static AddLineItem {
        static instance: ::protobuf::rt::LazyV2<AddLineItem> = ::protobuf::rt::LazyV2::INIT;
        instance.get(AddLineItem::new)
    }
}

impl ::protobuf::Clear for AddLineItem {
    fn clear(&mut self) {
        self.user_id.clear();
        self.product_id.clear();
        self.name.clear();
        self.quantity = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddLineItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddLineItem {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveLineItem {
    // message fields
    pub user_id: ::std::string::String,
    pub product_id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RemoveLineItem {
    fn default() -> &'a RemoveLineItem {
        <RemoveLineItem as ::protobuf::Message>::default_instance()
    }
}

impl RemoveLineItem {
    pub fn new() -> RemoveLineItem {
        ::std::default::Default::default()
    }

    // string user_id = 1;


    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
    pub fn clear_user_id(&mut self) {
        self.user_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: ::std::string::String) {
        self.user_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_id(&mut self) -> &mut ::std::string::String {
        &mut self.user_id
    }

    // Take field
    pub fn take_user_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_id, ::std::string::String::new())
    }

    // string product_id = 2;


    pub fn get_product_id(&self) -> &str {
        &self.product_id
    }
    pub fn clear_product_id(&mut self) {
        self.product_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_product_id(&mut self, v: ::std::string::String) {
        self.product_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_product_id(&mut self) -> &mut ::std::string::String {
        &mut self.product_id
    }

    // Take field
    pub fn take_product_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.product_id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for RemoveLineItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.product_id)?;
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
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.user_id);
        }
        if !self.product_id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.product_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.user_id.is_empty() {
            os.write_string(1, &self.user_id)?;
        }
        if !self.product_id.is_empty() {
            os.write_string(2, &self.product_id)?;
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

    fn new() -> RemoveLineItem {
        RemoveLineItem::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "user_id",
                |m: &RemoveLineItem| { &m.user_id },
                |m: &mut RemoveLineItem| { &mut m.user_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "product_id",
                |m: &RemoveLineItem| { &m.product_id },
                |m: &mut RemoveLineItem| { &mut m.product_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RemoveLineItem>(
                "RemoveLineItem",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RemoveLineItem {
        static instance: ::protobuf::rt::LazyV2<RemoveLineItem> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RemoveLineItem::new)
    }
}

impl ::protobuf::Clear for RemoveLineItem {
    fn clear(&mut self) {
        self.user_id.clear();
        self.product_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveLineItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveLineItem {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetShoppingCart {
    // message fields
    pub user_id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetShoppingCart {
    fn default() -> &'a GetShoppingCart {
        <GetShoppingCart as ::protobuf::Message>::default_instance()
    }
}

impl GetShoppingCart {
    pub fn new() -> GetShoppingCart {
        ::std::default::Default::default()
    }

    // string user_id = 1;


    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
    pub fn clear_user_id(&mut self) {
        self.user_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: ::std::string::String) {
        self.user_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user_id(&mut self) -> &mut ::std::string::String {
        &mut self.user_id
    }

    // Take field
    pub fn take_user_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.user_id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for GetShoppingCart {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.user_id)?;
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
        if !self.user_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.user_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.user_id.is_empty() {
            os.write_string(1, &self.user_id)?;
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

    fn new() -> GetShoppingCart {
        GetShoppingCart::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "user_id",
                |m: &GetShoppingCart| { &m.user_id },
                |m: &mut GetShoppingCart| { &mut m.user_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GetShoppingCart>(
                "GetShoppingCart",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GetShoppingCart {
        static instance: ::protobuf::rt::LazyV2<GetShoppingCart> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GetShoppingCart::new)
    }
}

impl ::protobuf::Clear for GetShoppingCart {
    fn clear(&mut self) {
        self.user_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetShoppingCart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetShoppingCart {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LineItem {
    // message fields
    pub product_id: ::std::string::String,
    pub name: ::std::string::String,
    pub quantity: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LineItem {
    fn default() -> &'a LineItem {
        <LineItem as ::protobuf::Message>::default_instance()
    }
}

impl LineItem {
    pub fn new() -> LineItem {
        ::std::default::Default::default()
    }

    // string product_id = 1;


    pub fn get_product_id(&self) -> &str {
        &self.product_id
    }
    pub fn clear_product_id(&mut self) {
        self.product_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_product_id(&mut self, v: ::std::string::String) {
        self.product_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_product_id(&mut self) -> &mut ::std::string::String {
        &mut self.product_id
    }

    // Take field
    pub fn take_product_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.product_id, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // int32 quantity = 3;


    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }
    pub fn clear_quantity(&mut self) {
        self.quantity = 0;
    }

    // Param is passed by value, moved
    pub fn set_quantity(&mut self, v: i32) {
        self.quantity = v;
    }
}

impl ::protobuf::Message for LineItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.product_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.quantity = tmp;
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
        if !self.product_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.product_id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if self.quantity != 0 {
            my_size += ::protobuf::rt::value_size(3, self.quantity, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.product_id.is_empty() {
            os.write_string(1, &self.product_id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.quantity != 0 {
            os.write_int32(3, self.quantity)?;
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

    fn new() -> LineItem {
        LineItem::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "product_id",
                |m: &LineItem| { &m.product_id },
                |m: &mut LineItem| { &mut m.product_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &LineItem| { &m.name },
                |m: &mut LineItem| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "quantity",
                |m: &LineItem| { &m.quantity },
                |m: &mut LineItem| { &mut m.quantity },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<LineItem>(
                "LineItem",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static LineItem {
        static instance: ::protobuf::rt::LazyV2<LineItem> = ::protobuf::rt::LazyV2::INIT;
        instance.get(LineItem::new)
    }
}

impl ::protobuf::Clear for LineItem {
    fn clear(&mut self) {
        self.product_id.clear();
        self.name.clear();
        self.quantity = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LineItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LineItem {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Cart {
    // message fields
    pub items: ::protobuf::RepeatedField<LineItem>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Cart {
    fn default() -> &'a Cart {
        <Cart as ::protobuf::Message>::default_instance()
    }
}

impl Cart {
    pub fn new() -> Cart {
        ::std::default::Default::default()
    }

    // repeated .com.example.shoppingcart.LineItem items = 1;


    pub fn get_items(&self) -> &[LineItem] {
        &self.items
    }
    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<LineItem>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<LineItem> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<LineItem> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Cart {
    fn is_initialized(&self) -> bool {
        for v in &self.items {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
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
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
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

    fn new() -> Cart {
        Cart::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LineItem>>(
                "items",
                |m: &Cart| { &m.items },
                |m: &mut Cart| { &mut m.items },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Cart>(
                "Cart",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Cart {
        static instance: ::protobuf::rt::LazyV2<Cart> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Cart::new)
    }
}

impl ::protobuf::Clear for Cart {
    fn clear(&mut self) {
        self.items.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cart {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'example/shoppingcart/shoppingcart.proto\x12\x18com.example.shoppingca\
    rt\x1a\x18google/proto/empty.proto\x1a\x1bcloudstate/entity_key.proto\
    \x1a\x1cgoogle/api/annotations.proto\x1a\x15google/api/http.proto\"{\n\
    \x0bAddLineItem\x12\x1d\n\x07user_id\x18\x01\x20\x01(\tR\x06userIdB\x04\
    \x90\xb5\x18\x01\x12\x1d\n\nproduct_id\x18\x02\x20\x01(\tR\tproductId\
    \x12\x12\n\x04name\x18\x03\x20\x01(\tR\x04name\x12\x1a\n\x08quantity\x18\
    \x04\x20\x01(\x05R\x08quantity\"N\n\x0eRemoveLineItem\x12\x1d\n\x07user_\
    id\x18\x01\x20\x01(\tR\x06userIdB\x04\x90\xb5\x18\x01\x12\x1d\n\nproduct\
    _id\x18\x02\x20\x01(\tR\tproductId\"0\n\x0fGetShoppingCart\x12\x1d\n\x07\
    user_id\x18\x01\x20\x01(\tR\x06userIdB\x04\x90\xb5\x18\x01\"Y\n\x08LineI\
    tem\x12\x1d\n\nproduct_id\x18\x01\x20\x01(\tR\tproductId\x12\x12\n\x04na\
    me\x18\x02\x20\x01(\tR\x04name\x12\x1a\n\x08quantity\x18\x03\x20\x01(\
    \x05R\x08quantity\"@\n\x04Cart\x128\n\x05items\x18\x01\x20\x03(\x0b2\".c\
    om.example.shoppingcart.LineItemR\x05items2\x94\x03\n\x0cShoppingCart\
    \x12n\n\x07AddItem\x12%.com.example.shoppingcart.AddLineItem\x1a\x16.goo\
    gle.protobuf.Empty\"$\x82\xd3\xe4\x93\x02\x1e\"\x19/cart/{user_id}/items\
    /add:\x01*\x12\x81\x01\n\nRemoveItem\x12(.com.example.shoppingcart.Remov\
    eLineItem\x1a\x16.google.protobuf.Empty\"1\x82\xd3\xe4\x93\x02+\")/cart/\
    {user_id}/items/{product_id}/remove\x12\x8f\x01\n\x07GetCart\x12).com.ex\
    ample.shoppingcart.GetShoppingCart\x1a\x1e.com.example.shoppingcart.Cart\
    \"9\x82\xd3\xe4\x93\x023\x12\x10/carts/{user_id}Z\x1f\x12\x16/carts/{use\
    r_id}/itemsb\x05itemsb\x06proto3\
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
