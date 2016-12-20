pub use self::call::CallInst;
pub use self::store::StoreInst;
pub use self::terminator::*;
pub use self::unary::*;

pub mod call;
pub mod store;
pub mod terminator;
pub mod unary;

use ir::User;

pub struct Instruction<'ctx>(User<'ctx>);
impl_upcast!(Instruction => User);

macro_rules! define_unimplemented_inst {
    ($name:ident) => {
        /// Unimplemented instruction.
        pub struct $name<'ctx>(Instruction<'ctx>);

        impl_upcast!($name => Instruction);
    }
}

define_unimplemented_inst!(AtomicCmpXchgInst);
define_unimplemented_inst!(AtomicRMWInst);
define_unimplemented_inst!(BinaryOperator);
define_unimplemented_inst!(IntrinsicInst);
define_unimplemented_inst!(CmpInst);
define_unimplemented_inst!(FCmpInst);
define_unimplemented_inst!(ICmpInst);
define_unimplemented_inst!(ExtractElementInst);
define_unimplemented_inst!(FenceInst);
define_unimplemented_inst!(FuncletPadInst);
define_unimplemented_inst!(CatchPadInst);
define_unimplemented_inst!(CleanupPadInst);
define_unimplemented_inst!(GetElementPtrInst);
define_unimplemented_inst!(InsertElementInst);
define_unimplemented_inst!(InsertValueInst);
define_unimplemented_inst!(LandingPadInst);
define_unimplemented_inst!(SelectInst);
define_unimplemented_inst!(ShuffleVectorInst);
define_unimplemented_inst!(CatchReturnInst);
define_unimplemented_inst!(CatchSwitchInst);
define_unimplemented_inst!(CleanupReturnInst);
define_unimplemented_inst!(IndirectBrInst);
define_unimplemented_inst!(InvokeInst);
define_unimplemented_inst!(ResumeInst);
define_unimplemented_inst!(SwitchInst);
define_unimplemented_inst!(UnreachableInst);
define_unimplemented_inst!(AddrSpaceCastInst);
define_unimplemented_inst!(BitCastInst);
define_unimplemented_inst!(FPExtInst);
define_unimplemented_inst!(FPToSIInst);
define_unimplemented_inst!(FPToUIInst);
define_unimplemented_inst!(FPTruncInst);
define_unimplemented_inst!(IntToPtrInst);
define_unimplemented_inst!(PtrToIntInst);
define_unimplemented_inst!(SExtInst);
define_unimplemented_inst!(SIToFPInst);
define_unimplemented_inst!(TruncInst);
define_unimplemented_inst!(UIToFPInst);
define_unimplemented_inst!(ZExtInst);
define_unimplemented_inst!(ExtractValueInst);
define_unimplemented_inst!(VAArgInst);
