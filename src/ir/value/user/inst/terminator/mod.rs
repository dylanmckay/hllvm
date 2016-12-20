pub use self::ret::ReturnInst;
pub use self::br::BranchInst;
pub use self::unreachable::UnreachableInst;

pub mod ret;
pub mod br;
pub mod unreachable;

use ir::Instruction;

pub struct TerminatorInst<'ctx>(Instruction<'ctx>);
impl_upcast!(TerminatorInst => Instruction);
