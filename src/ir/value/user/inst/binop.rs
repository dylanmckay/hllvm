use SafeWrapper;
use ir::{User, Instruction, Value, BinaryOpCode};
use sys;

/// An binary operator instruction.
pub struct BinaryOperatorInst<'ctx>(Instruction<'ctx>);

impl<'ctx> BinaryOperatorInst<'ctx>
{
    /// Creates a new extract element instruction.
    pub fn new(opcode: BinaryOpCode,
               lhs: &Value,
               rhs: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateBinaryOperator(opcode as _, lhs.inner(), rhs.inner());
            wrap_value!(inner => User => Instruction => BinaryOperatorInst)
        }
    }

    /// Creates a new 'no signed wrap' binary op.
    pub fn new_nsw(opcode: BinaryOpCode,
                   lhs: &Value,
                   rhs: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateBinaryOperatorNSW(opcode as _, lhs.inner(), rhs.inner());
            wrap_value!(inner => User => Instruction => BinaryOperatorInst)
        }
    }

    /// Creates a new 'no unsigned wrap' binary op.
    pub fn new_nuw(opcode: BinaryOpCode,
                   lhs: &Value,
                   rhs: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateBinaryOperatorNUW(opcode as _, lhs.inner(), rhs.inner());
            wrap_value!(inner => User => Instruction => BinaryOperatorInst)
        }
    }

    /// Creates a new 'exact' binary op.
    pub fn new_exact(opcode: BinaryOpCode,
                     lhs: &Value,
                     rhs: &Value) -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateBinaryOperatorExact(opcode as _, lhs.inner(), rhs.inner());
            wrap_value!(inner => User => Instruction => BinaryOperatorInst)
        }
    }
}

impl_subtype!(BinaryOperatorInst => Instruction);
