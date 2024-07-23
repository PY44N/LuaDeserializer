#[derive(Clone, Debug, PartialEq)]
pub enum LuaType {
    NIL,
    BOOLEAN(bool),
    INVALID, //Who decided that this was a good idea?
    NUMBER(f64),
    STRING(String),
}
