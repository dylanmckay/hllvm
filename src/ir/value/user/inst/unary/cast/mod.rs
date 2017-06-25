use ir::UnaryInst;

pub struct CastInst<'ctx>(UnaryInst<'ctx>);
impl_subtype!(CastInst => UnaryInst);

/// Define a generic cast instruction.
macro_rules! define_cast_instruction {
    ($name:ident => $ctor:ident) => {
        pub struct $name<'ctx>($crate::ir::CastInst<'ctx>);

        impl<'ctx> $name<'ctx>
        {
            /// Creates a new instruction.
            pub fn new(value: &$crate::ir::Value,
                       ty: &$crate::ir::Type) -> Self {
                use $crate::SafeWrapper;

                unsafe {
                    let inner = $crate::sys::$ctor(value.inner(), ty.inner());
                    $name($crate::ir::CastInst(
                        $crate::ir::UnaryInst($crate::ir::Instruction(
                                $crate::ir::User($crate::ir::Value::from_inner(inner))))))
                }
            }
        }

        impl_subtype!($name => CastInst);
    }
}

define_cast_instruction!(AddrSpaceCastInst => LLVMRustCreateAddrSpaceCastInst);
define_cast_instruction!(BitCastInst => LLVMRustCreateBitCastInst);
define_cast_instruction!(FPExtInst => LLVMRustCreateFPExtInst);
define_cast_instruction!(FPToSIInst => LLVMRustCreateFPToSIInst);
define_cast_instruction!(FPToUIInst => LLVMRustCreateFPToUIInst);
define_cast_instruction!(FPTruncInst => LLVMRustCreateFPTruncInst);
define_cast_instruction!(IntToPtrInst => LLVMRustCreateIntToPtrInst);
define_cast_instruction!(PtrToIntInst => LLVMRustCreatePtrToIntInst);
define_cast_instruction!(SExtInst => LLVMRustCreateSExtInst);
define_cast_instruction!(SIToFPInst => LLVMRustCreateSIToFPInst);
define_cast_instruction!(TruncInst => LLVMRustCreateTruncInst);
define_cast_instruction!(UIToFPInst => LLVMRustCreateUIToFPInst);
define_cast_instruction!(ZExtInst => LLVMRustCreateZExtInst);
