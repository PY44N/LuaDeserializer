use crate::{
    enums::lua_type::LuaType,
    util::{read_stream::ReadStream, write_stream::WriteStream},
};

#[derive(Debug)]
pub struct Constant {
    pub lua_type: LuaType,
}

impl Constant {
    pub fn new(memory_stream: &mut ReadStream) -> Self {
        Self {
            lua_type: match memory_stream.read_int8() {
                0 => LuaType::NIL,
                1 => LuaType::BOOLEAN(memory_stream.read_int8() == 1),
                3 => LuaType::NUMBER(memory_stream.read_double()),
                4 => LuaType::STRING(memory_stream.read_string()),
                _ => LuaType::INVALID,
            },
        }
    }

    pub fn serialize(&self, write_stream: &mut WriteStream) {
        write_stream.write_int8(match self.lua_type {
            LuaType::NIL => 0,
            LuaType::BOOLEAN(_) => 1,
            LuaType::INVALID => 2,
            LuaType::NUMBER(_) => 3,
            LuaType::STRING(_) => 4,
        });

        match &self.lua_type {
            LuaType::NIL => {}
            LuaType::BOOLEAN(data) => write_stream.write_int8(if *data { 1 } else { 0 }),
            LuaType::INVALID => unreachable!(),
            LuaType::NUMBER(data) => write_stream.write_double(*data),
            LuaType::STRING(data) => write_stream.write_string(&data),
        }
    }
}
