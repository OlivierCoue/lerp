// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc 3.19.4
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `udp-down.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:UdpMsgDownGameEntityUpdate)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UdpMsgDownGameEntityUpdate {
    // message fields
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityUpdate.id)
    pub id: u32,
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityUpdate.object_type)
    pub object_type: ::protobuf::EnumOrUnknown<super::common::GameEntityBaseType>,
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityUpdate.location_current)
    pub location_current: ::protobuf::MessageField<super::common::Point>,
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityUpdate.location_target)
    pub location_target: ::protobuf::MessageField<super::common::Point>,
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityUpdate.location_speed)
    pub location_speed: ::std::option::Option<f32>,
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityUpdate.location_timestamp_at_target)
    pub location_timestamp_at_target: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityUpdate.location_distance_to_target)
    pub location_distance_to_target: ::std::option::Option<f32>,
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityUpdate.location_shape)
    pub location_shape: ::protobuf::MessageField<super::common::Point>,
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityUpdate.health_current)
    pub health_current: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityUpdate.is_self)
    pub is_self: bool,
    // special fields
    // @@protoc_insertion_point(special_field:UdpMsgDownGameEntityUpdate.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UdpMsgDownGameEntityUpdate {
    fn default() -> &'a UdpMsgDownGameEntityUpdate {
        <UdpMsgDownGameEntityUpdate as ::protobuf::Message>::default_instance()
    }
}

impl UdpMsgDownGameEntityUpdate {
    pub fn new() -> UdpMsgDownGameEntityUpdate {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &UdpMsgDownGameEntityUpdate| { &m.id },
            |m: &mut UdpMsgDownGameEntityUpdate| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "object_type",
            |m: &UdpMsgDownGameEntityUpdate| { &m.object_type },
            |m: &mut UdpMsgDownGameEntityUpdate| { &mut m.object_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::Point>(
            "location_current",
            |m: &UdpMsgDownGameEntityUpdate| { &m.location_current },
            |m: &mut UdpMsgDownGameEntityUpdate| { &mut m.location_current },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::Point>(
            "location_target",
            |m: &UdpMsgDownGameEntityUpdate| { &m.location_target },
            |m: &mut UdpMsgDownGameEntityUpdate| { &mut m.location_target },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "location_speed",
            |m: &UdpMsgDownGameEntityUpdate| { &m.location_speed },
            |m: &mut UdpMsgDownGameEntityUpdate| { &mut m.location_speed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "location_timestamp_at_target",
            |m: &UdpMsgDownGameEntityUpdate| { &m.location_timestamp_at_target },
            |m: &mut UdpMsgDownGameEntityUpdate| { &mut m.location_timestamp_at_target },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "location_distance_to_target",
            |m: &UdpMsgDownGameEntityUpdate| { &m.location_distance_to_target },
            |m: &mut UdpMsgDownGameEntityUpdate| { &mut m.location_distance_to_target },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::Point>(
            "location_shape",
            |m: &UdpMsgDownGameEntityUpdate| { &m.location_shape },
            |m: &mut UdpMsgDownGameEntityUpdate| { &mut m.location_shape },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "health_current",
            |m: &UdpMsgDownGameEntityUpdate| { &m.health_current },
            |m: &mut UdpMsgDownGameEntityUpdate| { &mut m.health_current },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_self",
            |m: &UdpMsgDownGameEntityUpdate| { &m.is_self },
            |m: &mut UdpMsgDownGameEntityUpdate| { &mut m.is_self },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UdpMsgDownGameEntityUpdate>(
            "UdpMsgDownGameEntityUpdate",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UdpMsgDownGameEntityUpdate {
    const NAME: &'static str = "UdpMsgDownGameEntityUpdate";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = is.read_uint32()?;
                },
                16 => {
                    self.object_type = is.read_enum_or_unknown()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.location_current)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.location_target)?;
                },
                45 => {
                    self.location_speed = ::std::option::Option::Some(is.read_float()?);
                },
                72 => {
                    self.location_timestamp_at_target = ::std::option::Option::Some(is.read_uint64()?);
                },
                85 => {
                    self.location_distance_to_target = ::std::option::Option::Some(is.read_float()?);
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.location_shape)?;
                },
                56 => {
                    self.health_current = ::std::option::Option::Some(is.read_uint32()?);
                },
                64 => {
                    self.is_self = is.read_bool()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.id);
        }
        if self.object_type != ::protobuf::EnumOrUnknown::new(super::common::GameEntityBaseType::CHARACTER) {
            my_size += ::protobuf::rt::int32_size(2, self.object_type.value());
        }
        if let Some(v) = self.location_current.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.location_target.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.location_speed {
            my_size += 1 + 4;
        }
        if let Some(v) = self.location_timestamp_at_target {
            my_size += ::protobuf::rt::uint64_size(9, v);
        }
        if let Some(v) = self.location_distance_to_target {
            my_size += 1 + 4;
        }
        if let Some(v) = self.location_shape.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.health_current {
            my_size += ::protobuf::rt::uint32_size(7, v);
        }
        if self.is_self != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if self.object_type != ::protobuf::EnumOrUnknown::new(super::common::GameEntityBaseType::CHARACTER) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.object_type))?;
        }
        if let Some(v) = self.location_current.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.location_target.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.location_speed {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.location_timestamp_at_target {
            os.write_uint64(9, v)?;
        }
        if let Some(v) = self.location_distance_to_target {
            os.write_float(10, v)?;
        }
        if let Some(v) = self.location_shape.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.health_current {
            os.write_uint32(7, v)?;
        }
        if self.is_self != false {
            os.write_bool(8, self.is_self)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> UdpMsgDownGameEntityUpdate {
        UdpMsgDownGameEntityUpdate::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.object_type = ::protobuf::EnumOrUnknown::new(super::common::GameEntityBaseType::CHARACTER);
        self.location_current.clear();
        self.location_target.clear();
        self.location_speed = ::std::option::Option::None;
        self.location_timestamp_at_target = ::std::option::Option::None;
        self.location_distance_to_target = ::std::option::Option::None;
        self.location_shape.clear();
        self.health_current = ::std::option::Option::None;
        self.is_self = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UdpMsgDownGameEntityUpdate {
        static instance: UdpMsgDownGameEntityUpdate = UdpMsgDownGameEntityUpdate {
            id: 0,
            object_type: ::protobuf::EnumOrUnknown::from_i32(0),
            location_current: ::protobuf::MessageField::none(),
            location_target: ::protobuf::MessageField::none(),
            location_speed: ::std::option::Option::None,
            location_timestamp_at_target: ::std::option::Option::None,
            location_distance_to_target: ::std::option::Option::None,
            location_shape: ::protobuf::MessageField::none(),
            health_current: ::std::option::Option::None,
            is_self: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UdpMsgDownGameEntityUpdate {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UdpMsgDownGameEntityUpdate").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UdpMsgDownGameEntityUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UdpMsgDownGameEntityUpdate {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:UdpMsgDownGameEntityRemoved)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UdpMsgDownGameEntityRemoved {
    // message fields
    // @@protoc_insertion_point(field:UdpMsgDownGameEntityRemoved.id)
    pub id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UdpMsgDownGameEntityRemoved.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UdpMsgDownGameEntityRemoved {
    fn default() -> &'a UdpMsgDownGameEntityRemoved {
        <UdpMsgDownGameEntityRemoved as ::protobuf::Message>::default_instance()
    }
}

impl UdpMsgDownGameEntityRemoved {
    pub fn new() -> UdpMsgDownGameEntityRemoved {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &UdpMsgDownGameEntityRemoved| { &m.id },
            |m: &mut UdpMsgDownGameEntityRemoved| { &mut m.id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UdpMsgDownGameEntityRemoved>(
            "UdpMsgDownGameEntityRemoved",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UdpMsgDownGameEntityRemoved {
    const NAME: &'static str = "UdpMsgDownGameEntityRemoved";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> UdpMsgDownGameEntityRemoved {
        UdpMsgDownGameEntityRemoved::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UdpMsgDownGameEntityRemoved {
        static instance: UdpMsgDownGameEntityRemoved = UdpMsgDownGameEntityRemoved {
            id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UdpMsgDownGameEntityRemoved {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UdpMsgDownGameEntityRemoved").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UdpMsgDownGameEntityRemoved {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UdpMsgDownGameEntityRemoved {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:UdpMsgDown)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UdpMsgDown {
    // message fields
    // @@protoc_insertion_point(field:UdpMsgDown._type)
    pub _type: ::protobuf::EnumOrUnknown<UdpMsgDownType>,
    // @@protoc_insertion_point(field:UdpMsgDown.game_entity_update)
    pub game_entity_update: ::protobuf::MessageField<UdpMsgDownGameEntityUpdate>,
    // @@protoc_insertion_point(field:UdpMsgDown.game_entity_removed)
    pub game_entity_removed: ::protobuf::MessageField<UdpMsgDownGameEntityRemoved>,
    // special fields
    // @@protoc_insertion_point(special_field:UdpMsgDown.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UdpMsgDown {
    fn default() -> &'a UdpMsgDown {
        <UdpMsgDown as ::protobuf::Message>::default_instance()
    }
}

impl UdpMsgDown {
    pub fn new() -> UdpMsgDown {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "_type",
            |m: &UdpMsgDown| { &m._type },
            |m: &mut UdpMsgDown| { &mut m._type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, UdpMsgDownGameEntityUpdate>(
            "game_entity_update",
            |m: &UdpMsgDown| { &m.game_entity_update },
            |m: &mut UdpMsgDown| { &mut m.game_entity_update },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, UdpMsgDownGameEntityRemoved>(
            "game_entity_removed",
            |m: &UdpMsgDown| { &m.game_entity_removed },
            |m: &mut UdpMsgDown| { &mut m.game_entity_removed },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UdpMsgDown>(
            "UdpMsgDown",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UdpMsgDown {
    const NAME: &'static str = "UdpMsgDown";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self._type = is.read_enum_or_unknown()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.game_entity_update)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.game_entity_removed)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self._type != ::protobuf::EnumOrUnknown::new(UdpMsgDownType::GAME_ENTITY_UPDATE) {
            my_size += ::protobuf::rt::int32_size(1, self._type.value());
        }
        if let Some(v) = self.game_entity_update.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.game_entity_removed.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self._type != ::protobuf::EnumOrUnknown::new(UdpMsgDownType::GAME_ENTITY_UPDATE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self._type))?;
        }
        if let Some(v) = self.game_entity_update.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.game_entity_removed.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> UdpMsgDown {
        UdpMsgDown::new()
    }

    fn clear(&mut self) {
        self._type = ::protobuf::EnumOrUnknown::new(UdpMsgDownType::GAME_ENTITY_UPDATE);
        self.game_entity_update.clear();
        self.game_entity_removed.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UdpMsgDown {
        static instance: UdpMsgDown = UdpMsgDown {
            _type: ::protobuf::EnumOrUnknown::from_i32(0),
            game_entity_update: ::protobuf::MessageField::none(),
            game_entity_removed: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UdpMsgDown {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UdpMsgDown").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UdpMsgDown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UdpMsgDown {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:UdpMsgDownWrapper)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UdpMsgDownWrapper {
    // message fields
    // @@protoc_insertion_point(field:UdpMsgDownWrapper.server_time)
    pub server_time: u64,
    // @@protoc_insertion_point(field:UdpMsgDownWrapper.messages)
    pub messages: ::std::vec::Vec<UdpMsgDown>,
    // special fields
    // @@protoc_insertion_point(special_field:UdpMsgDownWrapper.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UdpMsgDownWrapper {
    fn default() -> &'a UdpMsgDownWrapper {
        <UdpMsgDownWrapper as ::protobuf::Message>::default_instance()
    }
}

impl UdpMsgDownWrapper {
    pub fn new() -> UdpMsgDownWrapper {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "server_time",
            |m: &UdpMsgDownWrapper| { &m.server_time },
            |m: &mut UdpMsgDownWrapper| { &mut m.server_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "messages",
            |m: &UdpMsgDownWrapper| { &m.messages },
            |m: &mut UdpMsgDownWrapper| { &mut m.messages },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UdpMsgDownWrapper>(
            "UdpMsgDownWrapper",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UdpMsgDownWrapper {
    const NAME: &'static str = "UdpMsgDownWrapper";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.server_time = is.read_uint64()?;
                },
                18 => {
                    self.messages.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.server_time != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.server_time);
        }
        for value in &self.messages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.server_time != 0 {
            os.write_uint64(1, self.server_time)?;
        }
        for v in &self.messages {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> UdpMsgDownWrapper {
        UdpMsgDownWrapper::new()
    }

    fn clear(&mut self) {
        self.server_time = 0;
        self.messages.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UdpMsgDownWrapper {
        static instance: UdpMsgDownWrapper = UdpMsgDownWrapper {
            server_time: 0,
            messages: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UdpMsgDownWrapper {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UdpMsgDownWrapper").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UdpMsgDownWrapper {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UdpMsgDownWrapper {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:UdpMsgDownType)
pub enum UdpMsgDownType {
    // @@protoc_insertion_point(enum_value:UdpMsgDownType.GAME_ENTITY_UPDATE)
    GAME_ENTITY_UPDATE = 0,
    // @@protoc_insertion_point(enum_value:UdpMsgDownType.GAME_ENTITY_REMOVED)
    GAME_ENTITY_REMOVED = 1,
}

impl ::protobuf::Enum for UdpMsgDownType {
    const NAME: &'static str = "UdpMsgDownType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UdpMsgDownType> {
        match value {
            0 => ::std::option::Option::Some(UdpMsgDownType::GAME_ENTITY_UPDATE),
            1 => ::std::option::Option::Some(UdpMsgDownType::GAME_ENTITY_REMOVED),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<UdpMsgDownType> {
        match str {
            "GAME_ENTITY_UPDATE" => ::std::option::Option::Some(UdpMsgDownType::GAME_ENTITY_UPDATE),
            "GAME_ENTITY_REMOVED" => ::std::option::Option::Some(UdpMsgDownType::GAME_ENTITY_REMOVED),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [UdpMsgDownType] = &[
        UdpMsgDownType::GAME_ENTITY_UPDATE,
        UdpMsgDownType::GAME_ENTITY_REMOVED,
    ];
}

impl ::protobuf::EnumFull for UdpMsgDownType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("UdpMsgDownType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for UdpMsgDownType {
    fn default() -> Self {
        UdpMsgDownType::GAME_ENTITY_UPDATE
    }
}

impl UdpMsgDownType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<UdpMsgDownType>("UdpMsgDownType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eudp-down.proto\x1a\x0ccommon.proto\"\xa2\x05\n\x1aUdpMsgDownGameEn\
    tityUpdate\x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\x124\n\x0bobject_ty\
    pe\x18\x02\x20\x01(\x0e2\x13.GameEntityBaseTypeR\nobjectType\x126\n\x10l\
    ocation_current\x18\x03\x20\x01(\x0b2\x06.PointH\0R\x0flocationCurrent\
    \x88\x01\x01\x124\n\x0flocation_target\x18\x04\x20\x01(\x0b2\x06.PointH\
    \x01R\x0elocationTarget\x88\x01\x01\x12*\n\x0elocation_speed\x18\x05\x20\
    \x01(\x02H\x02R\rlocationSpeed\x88\x01\x01\x12D\n\x1clocation_timestamp_\
    at_target\x18\t\x20\x01(\x04H\x03R\x19locationTimestampAtTarget\x88\x01\
    \x01\x12B\n\x1blocation_distance_to_target\x18\n\x20\x01(\x02H\x04R\x18l\
    ocationDistanceToTarget\x88\x01\x01\x122\n\x0elocation_shape\x18\x06\x20\
    \x01(\x0b2\x06.PointH\x05R\rlocationShape\x88\x01\x01\x12*\n\x0ehealth_c\
    urrent\x18\x07\x20\x01(\rH\x06R\rhealthCurrent\x88\x01\x01\x12\x17\n\x07\
    is_self\x18\x08\x20\x01(\x08R\x06isSelfB\x13\n\x11_location_currentB\x12\
    \n\x10_location_targetB\x11\n\x0f_location_speedB\x1f\n\x1d_location_tim\
    estamp_at_targetB\x1e\n\x1c_location_distance_to_targetB\x11\n\x0f_locat\
    ion_shapeB\x11\n\x0f_health_current\"-\n\x1bUdpMsgDownGameEntityRemoved\
    \x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\"\x84\x02\n\nUdpMsgDown\x12$\
    \n\x05_type\x18\x01\x20\x01(\x0e2\x0f.UdpMsgDownTypeR\x04Type\x12N\n\x12\
    game_entity_update\x18\x02\x20\x01(\x0b2\x1b.UdpMsgDownGameEntityUpdateH\
    \0R\x10gameEntityUpdate\x88\x01\x01\x12Q\n\x13game_entity_removed\x18\
    \x03\x20\x01(\x0b2\x1c.UdpMsgDownGameEntityRemovedH\x01R\x11gameEntityRe\
    moved\x88\x01\x01B\x15\n\x13_game_entity_updateB\x16\n\x14_game_entity_r\
    emoved\"]\n\x11UdpMsgDownWrapper\x12\x1f\n\x0bserver_time\x18\x01\x20\
    \x01(\x04R\nserverTime\x12'\n\x08messages\x18\x02\x20\x03(\x0b2\x0b.UdpM\
    sgDownR\x08messages*A\n\x0eUdpMsgDownType\x12\x16\n\x12GAME_ENTITY_UPDAT\
    E\x10\0\x12\x17\n\x13GAME_ENTITY_REMOVED\x10\x01b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::common::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(UdpMsgDownGameEntityUpdate::generated_message_descriptor_data());
            messages.push(UdpMsgDownGameEntityRemoved::generated_message_descriptor_data());
            messages.push(UdpMsgDown::generated_message_descriptor_data());
            messages.push(UdpMsgDownWrapper::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(UdpMsgDownType::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
