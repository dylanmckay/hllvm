pub use self::ret::ReturnInst;
pub use self::br::BranchInst;

pub mod ret;
pub mod br;

use ir::Instruction;

pub struct TerminatorInst<'ctx>(Instruction<'ctx>);
impl_upcast!(TerminatorInst => Instruction);
