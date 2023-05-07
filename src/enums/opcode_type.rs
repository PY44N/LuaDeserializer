//I hate programming

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpcodeType {
    OpMove,
    OpLoadConst,
    OpLoadBool,
    OpLoadNil,
    OpGetUpval,
    OpGetGlobal,
    OpGetTable,
    OpSetGlobal,
    OpSetUpval,
    OpSetTable,
    OpNewTable,
    OpSelf,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpPow,
    OpUnm,
    OpNot,
    OpLen,
    OpConcat,
    OpJmp,
    OpEq,
    OpLt,
    OpLe,
    OpTest,
    OpTestSet,
    OpCall,
    OpTailCall,
    OpReturn,
    OpForLoop,
    OpForPrep,
    OpTForLoop,
    OpSetList,
    OpClose,
    OpClosure,
    OpVarArg,
}

pub static OPCODE_TYPE_MAP: [OpcodeType; 38] = [
    OpcodeType::OpMove,
    OpcodeType::OpLoadConst,
    OpcodeType::OpLoadBool,
    OpcodeType::OpLoadNil,
    OpcodeType::OpGetUpval,
    OpcodeType::OpGetGlobal,
    OpcodeType::OpGetTable,
    OpcodeType::OpSetGlobal,
    OpcodeType::OpSetUpval,
    OpcodeType::OpSetTable,
    OpcodeType::OpNewTable,
    OpcodeType::OpSelf,
    OpcodeType::OpAdd,
    OpcodeType::OpSub,
    OpcodeType::OpMul,
    OpcodeType::OpDiv,
    OpcodeType::OpMod,
    OpcodeType::OpPow,
    OpcodeType::OpUnm,
    OpcodeType::OpNot,
    OpcodeType::OpLen,
    OpcodeType::OpConcat,
    OpcodeType::OpJmp,
    OpcodeType::OpEq,
    OpcodeType::OpLt,
    OpcodeType::OpLe,
    OpcodeType::OpTest,
    OpcodeType::OpTestSet,
    OpcodeType::OpCall,
    OpcodeType::OpTailCall,
    OpcodeType::OpReturn,
    OpcodeType::OpForLoop,
    OpcodeType::OpForPrep,
    OpcodeType::OpTForLoop,
    OpcodeType::OpSetList,
    OpcodeType::OpClose,
    OpcodeType::OpClosure,
    OpcodeType::OpVarArg,
];
