// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
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

//! Generated file from `messages.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RequestMatch)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RequestMatch {
    // message fields
    // @@protoc_insertion_point(field:RequestMatch.queue)
    pub queue: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:RequestMatch.players)
    pub players: ::std::vec::Vec<i32>,
    // special fields
    // @@protoc_insertion_point(special_field:RequestMatch.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RequestMatch {
    fn default() -> &'a RequestMatch {
        <RequestMatch as ::protobuf::Message>::default_instance()
    }
}

impl RequestMatch {
    pub fn new() -> RequestMatch {
        ::std::default::Default::default()
    }

    // required int32 queue = 1;

    pub fn queue(&self) -> i32 {
        self.queue.unwrap_or(0)
    }

    pub fn clear_queue(&mut self) {
        self.queue = ::std::option::Option::None;
    }

    pub fn has_queue(&self) -> bool {
        self.queue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_queue(&mut self, v: i32) {
        self.queue = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "queue",
            |m: &RequestMatch| { &m.queue },
            |m: &mut RequestMatch| { &mut m.queue },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "players",
            |m: &RequestMatch| { &m.players },
            |m: &mut RequestMatch| { &mut m.players },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RequestMatch>(
            "RequestMatch",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RequestMatch {
    const NAME: &'static str = "RequestMatch";

    fn is_initialized(&self) -> bool {
        if self.queue.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.queue = ::std::option::Option::Some(is.read_int32()?);
                },
                18 => {
                    is.read_repeated_packed_int32_into(&mut self.players)?;
                },
                16 => {
                    self.players.push(is.read_int32()?);
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
        if let Some(v) = self.queue {
            my_size += ::protobuf::rt::int32_size(1, v);
        }
        for value in &self.players {
            my_size += ::protobuf::rt::int32_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.queue {
            os.write_int32(1, v)?;
        }
        for v in &self.players {
            os.write_int32(2, *v)?;
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

    fn new() -> RequestMatch {
        RequestMatch::new()
    }

    fn clear(&mut self) {
        self.queue = ::std::option::Option::None;
        self.players.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RequestMatch {
        static instance: RequestMatch = RequestMatch {
            queue: ::std::option::Option::None,
            players: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RequestMatch {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RequestMatch").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RequestMatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestMatch {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:SetInQueue)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetInQueue {
    // message fields
    // @@protoc_insertion_point(field:SetInQueue.queue)
    pub queue: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:SetInQueue.state)
    pub state: ::std::option::Option<i32>,
    // special fields
    // @@protoc_insertion_point(special_field:SetInQueue.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetInQueue {
    fn default() -> &'a SetInQueue {
        <SetInQueue as ::protobuf::Message>::default_instance()
    }
}

impl SetInQueue {
    pub fn new() -> SetInQueue {
        ::std::default::Default::default()
    }

    // required int32 queue = 1;

    pub fn queue(&self) -> i32 {
        self.queue.unwrap_or(0)
    }

    pub fn clear_queue(&mut self) {
        self.queue = ::std::option::Option::None;
    }

    pub fn has_queue(&self) -> bool {
        self.queue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_queue(&mut self, v: i32) {
        self.queue = ::std::option::Option::Some(v);
    }

    // required int32 state = 2;

    pub fn state(&self) -> i32 {
        self.state.unwrap_or(0)
    }

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: i32) {
        self.state = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "queue",
            |m: &SetInQueue| { &m.queue },
            |m: &mut SetInQueue| { &mut m.queue },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "state",
            |m: &SetInQueue| { &m.state },
            |m: &mut SetInQueue| { &mut m.state },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetInQueue>(
            "SetInQueue",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetInQueue {
    const NAME: &'static str = "SetInQueue";

    fn is_initialized(&self) -> bool {
        if self.queue.is_none() {
            return false;
        }
        if self.state.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.queue = ::std::option::Option::Some(is.read_int32()?);
                },
                16 => {
                    self.state = ::std::option::Option::Some(is.read_int32()?);
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
        if let Some(v) = self.queue {
            my_size += ::protobuf::rt::int32_size(1, v);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::int32_size(2, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.queue {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.state {
            os.write_int32(2, v)?;
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

    fn new() -> SetInQueue {
        SetInQueue::new()
    }

    fn clear(&mut self) {
        self.queue = ::std::option::Option::None;
        self.state = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetInQueue {
        static instance: SetInQueue = SetInQueue {
            queue: ::std::option::Option::None,
            state: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetInQueue {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetInQueue").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetInQueue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetInQueue {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:Match)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Match {
    // message fields
    // @@protoc_insertion_point(field:Match.moonstone_port)
    pub moonstone_port: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:Match.players)
    pub players: ::std::vec::Vec<i32>,
    // @@protoc_insertion_point(field:Match.token)
    pub token: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:Match.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Match {
    fn default() -> &'a Match {
        <Match as ::protobuf::Message>::default_instance()
    }
}

impl Match {
    pub fn new() -> Match {
        ::std::default::Default::default()
    }

    // required int32 moonstone_port = 1;

    pub fn moonstone_port(&self) -> i32 {
        self.moonstone_port.unwrap_or(0)
    }

    pub fn clear_moonstone_port(&mut self) {
        self.moonstone_port = ::std::option::Option::None;
    }

    pub fn has_moonstone_port(&self) -> bool {
        self.moonstone_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_moonstone_port(&mut self, v: i32) {
        self.moonstone_port = ::std::option::Option::Some(v);
    }

    // required string token = 3;

    pub fn token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_token(&mut self) {
        self.token = ::std::option::Option::None;
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token = ::std::option::Option::Some(::std::string::String::new());
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "moonstone_port",
            |m: &Match| { &m.moonstone_port },
            |m: &mut Match| { &mut m.moonstone_port },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "players",
            |m: &Match| { &m.players },
            |m: &mut Match| { &mut m.players },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "token",
            |m: &Match| { &m.token },
            |m: &mut Match| { &mut m.token },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Match>(
            "Match",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Match {
    const NAME: &'static str = "Match";

    fn is_initialized(&self) -> bool {
        if self.moonstone_port.is_none() {
            return false;
        }
        if self.token.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.moonstone_port = ::std::option::Option::Some(is.read_int32()?);
                },
                18 => {
                    is.read_repeated_packed_int32_into(&mut self.players)?;
                },
                16 => {
                    self.players.push(is.read_int32()?);
                },
                26 => {
                    self.token = ::std::option::Option::Some(is.read_string()?);
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
        if let Some(v) = self.moonstone_port {
            my_size += ::protobuf::rt::int32_size(1, v);
        }
        for value in &self.players {
            my_size += ::protobuf::rt::int32_size(2, *value);
        };
        if let Some(v) = self.token.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.moonstone_port {
            os.write_int32(1, v)?;
        }
        for v in &self.players {
            os.write_int32(2, *v)?;
        };
        if let Some(v) = self.token.as_ref() {
            os.write_string(3, v)?;
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

    fn new() -> Match {
        Match::new()
    }

    fn clear(&mut self) {
        self.moonstone_port = ::std::option::Option::None;
        self.players.clear();
        self.token = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Match {
        static instance: Match = Match {
            moonstone_port: ::std::option::Option::None,
            players: ::std::vec::Vec::new(),
            token: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Match {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Match").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Match {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Match {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0emessages.proto\">\n\x0cRequestMatch\x12\x14\n\x05queue\x18\x01\x20\
    \x02(\x05R\x05queue\x12\x18\n\x07players\x18\x02\x20\x03(\x05R\x07player\
    s\"8\n\nSetInQueue\x12\x14\n\x05queue\x18\x01\x20\x02(\x05R\x05queue\x12\
    \x14\n\x05state\x18\x02\x20\x02(\x05R\x05state\"^\n\x05Match\x12%\n\x0em\
    oonstone_port\x18\x01\x20\x02(\x05R\rmoonstonePort\x12\x18\n\x07players\
    \x18\x02\x20\x03(\x05R\x07players\x12\x14\n\x05token\x18\x03\x20\x02(\tR\
    \x05tokenb\x06proto2\
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
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(RequestMatch::generated_message_descriptor_data());
            messages.push(SetInQueue::generated_message_descriptor_data());
            messages.push(Match::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
