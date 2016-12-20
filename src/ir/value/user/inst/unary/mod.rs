pub use self::alloca::AllocaInst;

pub mod alloca;

use ir::Instruction;

pub struct UnaryInst<'ctx>(Instruction<'ctx>);
impl_upcast!(UnaryInst => Instruction);
