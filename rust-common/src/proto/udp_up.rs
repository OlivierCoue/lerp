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

//! Generated file from `udp-up.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:UdpMsgUp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UdpMsgUp {
    // message fields
    // @@protoc_insertion_point(field:UdpMsgUp._type)
    pub _type: ::protobuf::EnumOrUnknown<UdpMsgUpType>,
    // @@protoc_insertion_point(field:UdpMsgUp.player_move)
    pub player_move: ::protobuf::MessageField<super::common::Point>,
    // @@protoc_insertion_point(field:UdpMsgUp.player_teleport)
    pub player_teleport: ::protobuf::MessageField<super::common::Point>,
    // @@protoc_insertion_point(field:UdpMsgUp.player_throw_projectile)
    pub player_throw_projectile: ::protobuf::MessageField<super::common::Point>,
    // @@protoc_insertion_point(field:UdpMsgUp.player_throw_frozen_orb)
    pub player_throw_frozen_orb: ::protobuf::MessageField<super::common::Point>,
    // special fields
    // @@protoc_insertion_point(special_field:UdpMsgUp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UdpMsgUp {
    fn default() -> &'a UdpMsgUp {
        <UdpMsgUp as ::protobuf::Message>::default_instance()
    }
}

impl UdpMsgUp {
    pub fn new() -> UdpMsgUp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "_type",
            |m: &UdpMsgUp| { &m._type },
            |m: &mut UdpMsgUp| { &mut m._type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::Point>(
            "player_move",
            |m: &UdpMsgUp| { &m.player_move },
            |m: &mut UdpMsgUp| { &mut m.player_move },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::Point>(
            "player_teleport",
            |m: &UdpMsgUp| { &m.player_teleport },
            |m: &mut UdpMsgUp| { &mut m.player_teleport },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::Point>(
            "player_throw_projectile",
            |m: &UdpMsgUp| { &m.player_throw_projectile },
            |m: &mut UdpMsgUp| { &mut m.player_throw_projectile },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::common::Point>(
            "player_throw_frozen_orb",
            |m: &UdpMsgUp| { &m.player_throw_frozen_orb },
            |m: &mut UdpMsgUp| { &mut m.player_throw_frozen_orb },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UdpMsgUp>(
            "UdpMsgUp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UdpMsgUp {
    const NAME: &'static str = "UdpMsgUp";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.player_move)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.player_teleport)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.player_throw_projectile)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.player_throw_frozen_orb)?;
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
        if self._type != ::protobuf::EnumOrUnknown::new(UdpMsgUpType::GAME_PAUSE) {
            my_size += ::protobuf::rt::int32_size(1, self._type.value());
        }
        if let Some(v) = self.player_move.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.player_teleport.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.player_throw_projectile.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.player_throw_frozen_orb.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self._type != ::protobuf::EnumOrUnknown::new(UdpMsgUpType::GAME_PAUSE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self._type))?;
        }
        if let Some(v) = self.player_move.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.player_teleport.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.player_throw_projectile.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.player_throw_frozen_orb.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> UdpMsgUp {
        UdpMsgUp::new()
    }

    fn clear(&mut self) {
        self._type = ::protobuf::EnumOrUnknown::new(UdpMsgUpType::GAME_PAUSE);
        self.player_move.clear();
        self.player_teleport.clear();
        self.player_throw_projectile.clear();
        self.player_throw_frozen_orb.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UdpMsgUp {
        static instance: UdpMsgUp = UdpMsgUp {
            _type: ::protobuf::EnumOrUnknown::from_i32(0),
            player_move: ::protobuf::MessageField::none(),
            player_teleport: ::protobuf::MessageField::none(),
            player_throw_projectile: ::protobuf::MessageField::none(),
            player_throw_frozen_orb: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UdpMsgUp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UdpMsgUp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UdpMsgUp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UdpMsgUp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:UdpMsgUpWrapper)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UdpMsgUpWrapper {
    // message fields
    // @@protoc_insertion_point(field:UdpMsgUpWrapper.messages)
    pub messages: ::std::vec::Vec<UdpMsgUp>,
    // special fields
    // @@protoc_insertion_point(special_field:UdpMsgUpWrapper.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UdpMsgUpWrapper {
    fn default() -> &'a UdpMsgUpWrapper {
        <UdpMsgUpWrapper as ::protobuf::Message>::default_instance()
    }
}

impl UdpMsgUpWrapper {
    pub fn new() -> UdpMsgUpWrapper {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "messages",
            |m: &UdpMsgUpWrapper| { &m.messages },
            |m: &mut UdpMsgUpWrapper| { &mut m.messages },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UdpMsgUpWrapper>(
            "UdpMsgUpWrapper",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UdpMsgUpWrapper {
    const NAME: &'static str = "UdpMsgUpWrapper";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
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
        for value in &self.messages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.messages {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> UdpMsgUpWrapper {
        UdpMsgUpWrapper::new()
    }

    fn clear(&mut self) {
        self.messages.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UdpMsgUpWrapper {
        static instance: UdpMsgUpWrapper = UdpMsgUpWrapper {
            messages: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UdpMsgUpWrapper {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UdpMsgUpWrapper").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UdpMsgUpWrapper {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UdpMsgUpWrapper {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:UdpMsgUpType)
pub enum UdpMsgUpType {
    // @@protoc_insertion_point(enum_value:UdpMsgUpType.GAME_PAUSE)
    GAME_PAUSE = 0,
    // @@protoc_insertion_point(enum_value:UdpMsgUpType.PLAYER_INIT)
    PLAYER_INIT = 1,
    // @@protoc_insertion_point(enum_value:UdpMsgUpType.PLAYER_PING)
    PLAYER_PING = 2,
    // @@protoc_insertion_point(enum_value:UdpMsgUpType.PLAYER_MOVE)
    PLAYER_MOVE = 3,
    // @@protoc_insertion_point(enum_value:UdpMsgUpType.PLAYER_TELEPORT)
    PLAYER_TELEPORT = 4,
    // @@protoc_insertion_point(enum_value:UdpMsgUpType.PLAYER_TOGGLE_HIDDEN)
    PLAYER_TOGGLE_HIDDEN = 5,
    // @@protoc_insertion_point(enum_value:UdpMsgUpType.PLAYER_THROW_PROJECTILE)
    PLAYER_THROW_PROJECTILE = 6,
    // @@protoc_insertion_point(enum_value:UdpMsgUpType.PLAYER_THROW_FROZEN_ORB)
    PLAYER_THROW_FROZEN_ORB = 7,
}

impl ::protobuf::Enum for UdpMsgUpType {
    const NAME: &'static str = "UdpMsgUpType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UdpMsgUpType> {
        match value {
            0 => ::std::option::Option::Some(UdpMsgUpType::GAME_PAUSE),
            1 => ::std::option::Option::Some(UdpMsgUpType::PLAYER_INIT),
            2 => ::std::option::Option::Some(UdpMsgUpType::PLAYER_PING),
            3 => ::std::option::Option::Some(UdpMsgUpType::PLAYER_MOVE),
            4 => ::std::option::Option::Some(UdpMsgUpType::PLAYER_TELEPORT),
            5 => ::std::option::Option::Some(UdpMsgUpType::PLAYER_TOGGLE_HIDDEN),
            6 => ::std::option::Option::Some(UdpMsgUpType::PLAYER_THROW_PROJECTILE),
            7 => ::std::option::Option::Some(UdpMsgUpType::PLAYER_THROW_FROZEN_ORB),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<UdpMsgUpType> {
        match str {
            "GAME_PAUSE" => ::std::option::Option::Some(UdpMsgUpType::GAME_PAUSE),
            "PLAYER_INIT" => ::std::option::Option::Some(UdpMsgUpType::PLAYER_INIT),
            "PLAYER_PING" => ::std::option::Option::Some(UdpMsgUpType::PLAYER_PING),
            "PLAYER_MOVE" => ::std::option::Option::Some(UdpMsgUpType::PLAYER_MOVE),
            "PLAYER_TELEPORT" => ::std::option::Option::Some(UdpMsgUpType::PLAYER_TELEPORT),
            "PLAYER_TOGGLE_HIDDEN" => ::std::option::Option::Some(UdpMsgUpType::PLAYER_TOGGLE_HIDDEN),
            "PLAYER_THROW_PROJECTILE" => ::std::option::Option::Some(UdpMsgUpType::PLAYER_THROW_PROJECTILE),
            "PLAYER_THROW_FROZEN_ORB" => ::std::option::Option::Some(UdpMsgUpType::PLAYER_THROW_FROZEN_ORB),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [UdpMsgUpType] = &[
        UdpMsgUpType::GAME_PAUSE,
        UdpMsgUpType::PLAYER_INIT,
        UdpMsgUpType::PLAYER_PING,
        UdpMsgUpType::PLAYER_MOVE,
        UdpMsgUpType::PLAYER_TELEPORT,
        UdpMsgUpType::PLAYER_TOGGLE_HIDDEN,
        UdpMsgUpType::PLAYER_THROW_PROJECTILE,
        UdpMsgUpType::PLAYER_THROW_FROZEN_ORB,
    ];
}

impl ::protobuf::EnumFull for UdpMsgUpType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("UdpMsgUpType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for UdpMsgUpType {
    fn default() -> Self {
        UdpMsgUpType::GAME_PAUSE
    }
}

impl UdpMsgUpType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<UdpMsgUpType>("UdpMsgUpType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cudp-up.proto\x1a\x0ccommon.proto\"\xf7\x02\n\x08UdpMsgUp\x12\"\n\
    \x05_type\x18\x01\x20\x01(\x0e2\r.UdpMsgUpTypeR\x04Type\x12,\n\x0bplayer\
    _move\x18\x02\x20\x01(\x0b2\x06.PointH\0R\nplayerMove\x88\x01\x01\x124\n\
    \x0fplayer_teleport\x18\x03\x20\x01(\x0b2\x06.PointH\x01R\x0eplayerTelep\
    ort\x88\x01\x01\x12C\n\x17player_throw_projectile\x18\x04\x20\x01(\x0b2\
    \x06.PointH\x02R\x15playerThrowProjectile\x88\x01\x01\x12B\n\x17player_t\
    hrow_frozen_orb\x18\x05\x20\x01(\x0b2\x06.PointH\x03R\x14playerThrowFroz\
    enOrb\x88\x01\x01B\x0e\n\x0c_player_moveB\x12\n\x10_player_teleportB\x1a\
    \n\x18_player_throw_projectileB\x1a\n\x18_player_throw_frozen_orb\"8\n\
    \x0fUdpMsgUpWrapper\x12%\n\x08messages\x18\x01\x20\x03(\x0b2\t.UdpMsgUpR\
    \x08messages*\xba\x01\n\x0cUdpMsgUpType\x12\x0e\n\nGAME_PAUSE\x10\0\x12\
    \x0f\n\x0bPLAYER_INIT\x10\x01\x12\x0f\n\x0bPLAYER_PING\x10\x02\x12\x0f\n\
    \x0bPLAYER_MOVE\x10\x03\x12\x13\n\x0fPLAYER_TELEPORT\x10\x04\x12\x18\n\
    \x14PLAYER_TOGGLE_HIDDEN\x10\x05\x12\x1b\n\x17PLAYER_THROW_PROJECTILE\
    \x10\x06\x12\x1b\n\x17PLAYER_THROW_FROZEN_ORB\x10\x07b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(UdpMsgUp::generated_message_descriptor_data());
            messages.push(UdpMsgUpWrapper::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(UdpMsgUpType::generated_enum_descriptor_data());
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
