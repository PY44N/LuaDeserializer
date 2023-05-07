use std::fs;

use crate::{structs::chunk::Chunk, util::write_stream::WriteStream};

pub struct Serializer {
    write_stream: WriteStream,
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            write_stream: WriteStream::new(),
        }
    }

    pub fn from_size(size_t_size: u8, int_size: u8) -> Self {
        Self {
            write_stream: WriteStream::from_size(size_t_size, int_size),
        }
    }

    pub fn serialize(&mut self, main_chunk: Chunk) -> Vec<u8> {
        self.write_stream.write_string_len(
            &(String::from_utf8(vec![27]).expect("Failed to create string") + "Lua"),
        );

        self.write_stream.write_int8(0x51);
        self.write_stream.write_int8(0);

        //TODO: Endianess
        self.write_stream.write_int8(1);

        self.write_stream.write_int8(self.write_stream.int_size);
        self.write_stream.write_int8(self.write_stream.size_t_size);

        self.write_stream.write_int8(4);

        //TODO: 4-byte floating point support
        self.write_stream.write_int8(8);
        self.write_stream.write_int8(0);

        main_chunk.serialize(&mut self.write_stream);

        self.write_stream.bytes.clone()
    }

    pub fn serialize_to_file(&mut self, main_chunk: Chunk, file: &str) {
        fs::write(file, self.serialize(main_chunk))
            .expect(&format!("Failed to write to file {}", file));
    }
}
