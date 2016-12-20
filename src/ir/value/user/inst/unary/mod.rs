pub use self::alloca::AllocaInst;
pub use self::load::LoadInst;

pub mod alloca;
pub mod load;

use ir::Instruction;

pub struct UnaryInst<'ctx>(Instruction<'ctx>);
impl_upcast!(UnaryInst => Instruction);
