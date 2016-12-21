pub use self::br::BranchInst;
pub use self::catchreturn::CatchReturnInst;
pub use self::cleanupreturn::CleanupReturnInst;
pub use self::indirectbr::IndirectBrInst;
pub use self::invoke::InvokeInst;
pub use self::resume::ResumeInst;
pub use self::ret::ReturnInst;
pub use self::switch::SwitchInst;
pub use self::unreachable::UnreachableInst;

pub mod br;
pub mod catchreturn;
pub mod cleanupreturn;
pub mod indirectbr;
pub mod invoke;
pub mod resume;
pub mod ret;
pub mod switch;
pub mod unreachable;

use ir::Instruction;

pub struct TerminatorInst<'ctx>(Instruction<'ctx>);
impl_upcast!(TerminatorInst => Instruction);
