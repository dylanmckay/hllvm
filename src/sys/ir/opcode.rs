use libc;

/// An operation code for an IR instruction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OpCode {
    Terminator(TerminatorOpCode),
    Binary(BinaryOpCode),
    Logical(LogicalOpCode),
    Memory(MemoryOpCode),
    Cast(CastOpCode),
    Other(OtherOpCode),
}

/// An opcode for a block terminator instruction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum TerminatorOpCode {
    Ret = 1,
    Br = 2,
    Switch = 3,
    IndirectBr = 4,
    Invoke = 5,
    Resume = 6,
    Unreachable = 7,
    CleanupRet = 8,
    CatchRet = 9,
    CatchSwitch = 10,
}

/// An opcode for a binary operation instruction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum BinaryOpCode {
    Add = 11,
    FAdd = 12,
    Sub = 13,
    FSub = 14,
    Mul = 15,
    FMul = 16,
    UDiv = 17,
    SDiv = 18,
    FDiv = 19,
    URem = 20,
    SRem = 21,
    FRem = 22,
}

/// An opcode for a logical operation instruction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum LogicalOpCode {
    Shl = 23,
    LShr = 24,
    AShr = 25,
    And = 26,
    Or = 27,
    Xor = 28,
}

/// An opcode for a memory instruction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum MemoryOpCode {
    Alloca = 29,
    Load = 30,
    Store = 31,
    GetElementPtr = 32,
    Fence = 33,
    AtomicCmpXchg = 34,
    AtomicRMW = 35,
}

/// An opcode for a cast instruction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum CastOpCode {
    Trunc = 36,
    ZExt = 37,
    SExt = 38,
    FPToUI = 39,
    FPToSI = 40,
    UIToFP = 41,
    SIToFP = 42,
    FPTrunc = 43,
    FPExt = 44,
    PtrToInt = 45,
    IntToPtr = 46,
    BitCast = 47,
    AddrSpaceCast = 48,
}

/// An opcode for some other instruction.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum OtherOpCode {
    CleanupPad = 49,
    CatchPad = 50,
    ICmp = 51,
    FCmp = 52,
    PHI = 53,
    Call = 54,
    Select = 55,
    UserOp1 = 56,
    UserOp2 = 57,
    VAArg = 58,
    ExtractElement = 59,
    InsertElement = 60,
    ShuffleVector = 61,
    ExtractValue = 62,
    InsertValue = 63,
    LandingPad = 64,
}

impl OpCode {
    /// Gets the numerical code for the opcode.
    pub fn code(self) -> libc::c_uint {
        match self {
            OpCode::Terminator(op) => op as libc::c_uint,
            OpCode::Binary(op) => op as libc::c_uint,
            OpCode::Logical(op) => op as libc::c_uint,
            OpCode::Memory(op) => op as libc::c_uint,
            OpCode::Cast(op) => op as libc::c_uint,
            OpCode::Other(op) => op as libc::c_uint,
        }
    }
}
