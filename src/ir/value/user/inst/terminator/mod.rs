pub use self::ret::ReturnInst;

pub mod ret;

use ir::Instruction;

pub struct TerminatorInst<'ctx>(Instruction<'ctx>);
impl_upcast!(TerminatorInst => Instruction);
