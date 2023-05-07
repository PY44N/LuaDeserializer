use crate::util::{read_stream::ReadStream, write_stream::WriteStream};

#[derive(Debug)]
pub struct Local {
    pub name: String,
    pub start: u64,
    pub end: u64,
}

impl Local {
    pub fn new(memory_stream: &mut ReadStream) -> Self {
        Self {
            name: memory_stream.read_string(),
            start: memory_stream.read_int(),
            end: memory_stream.read_int(),
        }
    }

    pub fn serialize(&self, write_stream: &mut WriteStream) {
        write_stream.write_string(&self.name);
        write_stream.write_int(self.start);
        write_stream.write_int(self.end);
    }
}
