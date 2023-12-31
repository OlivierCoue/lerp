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

//! Generated file from `common.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:Point)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Point {
    // message fields
    // @@protoc_insertion_point(field:Point.x)
    pub x: f32,
    // @@protoc_insertion_point(field:Point.y)
    pub y: f32,
    // special fields
    // @@protoc_insertion_point(special_field:Point.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Point {
    fn default() -> &'a Point {
        <Point as ::protobuf::Message>::default_instance()
    }
}

impl Point {
    pub fn new() -> Point {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "x",
            |m: &Point| { &m.x },
            |m: &mut Point| { &mut m.x },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "y",
            |m: &Point| { &m.y },
            |m: &mut Point| { &mut m.y },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Point>(
            "Point",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Point {
    const NAME: &'static str = "Point";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                13 => {
                    self.x = is.read_float()?;
                },
                21 => {
                    self.y = is.read_float()?;
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
        if self.x != 0. {
            my_size += 1 + 4;
        }
        if self.y != 0. {
            my_size += 1 + 4;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.x != 0. {
            os.write_float(1, self.x)?;
        }
        if self.y != 0. {
            os.write_float(2, self.y)?;
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

    fn new() -> Point {
        Point::new()
    }

    fn clear(&mut self) {
        self.x = 0.;
        self.y = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Point {
        static instance: Point = Point {
            x: 0.,
            y: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Point {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Point").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Point {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:GameEntityBaseType)
pub enum GameEntityBaseType {
    // @@protoc_insertion_point(enum_value:GameEntityBaseType.CHARACTER)
    CHARACTER = 0,
    // @@protoc_insertion_point(enum_value:GameEntityBaseType.PROJECTILE)
    PROJECTILE = 1,
    // @@protoc_insertion_point(enum_value:GameEntityBaseType.ENEMY)
    ENEMY = 2,
}

impl ::protobuf::Enum for GameEntityBaseType {
    const NAME: &'static str = "GameEntityBaseType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GameEntityBaseType> {
        match value {
            0 => ::std::option::Option::Some(GameEntityBaseType::CHARACTER),
            1 => ::std::option::Option::Some(GameEntityBaseType::PROJECTILE),
            2 => ::std::option::Option::Some(GameEntityBaseType::ENEMY),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<GameEntityBaseType> {
        match str {
            "CHARACTER" => ::std::option::Option::Some(GameEntityBaseType::CHARACTER),
            "PROJECTILE" => ::std::option::Option::Some(GameEntityBaseType::PROJECTILE),
            "ENEMY" => ::std::option::Option::Some(GameEntityBaseType::ENEMY),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [GameEntityBaseType] = &[
        GameEntityBaseType::CHARACTER,
        GameEntityBaseType::PROJECTILE,
        GameEntityBaseType::ENEMY,
    ];
}

impl ::protobuf::EnumFull for GameEntityBaseType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("GameEntityBaseType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for GameEntityBaseType {
    fn default() -> Self {
        GameEntityBaseType::CHARACTER
    }
}

impl GameEntityBaseType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<GameEntityBaseType>("GameEntityBaseType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ccommon.proto\"#\n\x05Point\x12\x0c\n\x01x\x18\x01\x20\x01(\x02R\
    \x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\x02R\x01y*>\n\x12GameEntityBaseTyp\
    e\x12\r\n\tCHARACTER\x10\0\x12\x0e\n\nPROJECTILE\x10\x01\x12\t\n\x05ENEM\
    Y\x10\x02b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Point::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(GameEntityBaseType::generated_enum_descriptor_data());
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
