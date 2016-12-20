pub use self::br::BranchInst;
pub use self::invoke::InvokeInst;
pub use self::ret::ReturnInst;
pub use self::unreachable::UnreachableInst;

pub mod br;
pub mod invoke;
pub mod ret;
pub mod unreachable;

use ir::Instruction;

pub struct TerminatorInst<'ctx>(Instruction<'ctx>);
impl_upcast!(TerminatorInst => Instruction);
